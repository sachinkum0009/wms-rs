use anyhow::{Context, Result};
use dotenv::dotenv;
use sqlx::{postgres::PgPoolOptions, PgPool, Pool, Postgres, Row, FromRow};
use std::env;
use std::time::Duration;
use tracing::{info, error, warn};
use chrono::{DateTime, Utc};

/// Database configuration structure
#[derive(Debug, Clone)]
pub struct DatabaseConfig {
    pub database_url: String,
    pub max_connections: u32,
    pub min_connections: u32,
    pub connection_timeout: Duration,
    pub idle_timeout: Duration,
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            database_url: "postgresql://localhost/wms_dev".to_string(),
            max_connections: 10,
            min_connections: 1,
            connection_timeout: Duration::from_secs(30),
            idle_timeout: Duration::from_secs(600),
        }
    }
}

impl DatabaseConfig {
    /// Load configuration from environment variables
    pub fn from_env() -> Result<Self> {
        // Load .env file if it exists
        let _ = dotenv();

        let database_url = env::var("DATABASE_URL")
            .context("DATABASE_URL environment variable must be set")?;

        let max_connections = env::var("DB_MAX_CONNECTIONS")
            .unwrap_or_else(|_| "10".to_string())
            .parse()
            .context("Invalid DB_MAX_CONNECTIONS value")?;

        let min_connections = env::var("DB_MIN_CONNECTIONS")
            .unwrap_or_else(|_| "1".to_string())
            .parse()
            .context("Invalid DB_MIN_CONNECTIONS value")?;

        let connection_timeout_secs = env::var("DB_CONNECTION_TIMEOUT_SECS")
            .unwrap_or_else(|_| "30".to_string())
            .parse()
            .context("Invalid DB_CONNECTION_TIMEOUT_SECS value")?;

        let idle_timeout_secs = env::var("DB_IDLE_TIMEOUT_SECS")
            .unwrap_or_else(|_| "600".to_string())
            .parse()
            .context("Invalid DB_IDLE_TIMEOUT_SECS value")?;

        Ok(Self {
            database_url,
            max_connections,
            min_connections,
            connection_timeout: Duration::from_secs(connection_timeout_secs),
            idle_timeout: Duration::from_secs(idle_timeout_secs),
        })
    }
}

/// Database connection pool wrapper
#[derive(Debug, Clone)]
pub struct Database {
    pool: PgPool,
}

impl Database {
    /// Initialize a new database connection pool
    pub async fn new(config: DatabaseConfig) -> Result<Self> {
        info!("Initializing database connection pool...");
        info!("Database URL: {}", mask_database_url(&config.database_url));
        info!("Max connections: {}", config.max_connections);
        info!("Min connections: {}", config.min_connections);

        let pool = PgPoolOptions::new()
            .max_connections(config.max_connections)
            .min_connections(config.min_connections)
            .acquire_timeout(config.connection_timeout)
            .idle_timeout(config.idle_timeout)
            .connect(&config.database_url)
            .await
            .context("Failed to create database connection pool")?;

        info!("Database connection pool initialized successfully");

        Ok(Self { pool })
    }

    /// Initialize database with default configuration from environment
    pub async fn from_env() -> Result<Self> {
        let config = DatabaseConfig::from_env()?;
        Self::new(config).await
    }

    /// Get a reference to the underlying connection pool
    pub fn pool(&self) -> &PgPool {
        &self.pool
    }

    /// Run database health check
    pub async fn health_check(&self) -> Result<()> {
        info!("Running database health check...");
        
        match sqlx::query("SELECT 1 as health_check")
            .fetch_one(&self.pool)
            .await
        {
            Ok(row) => {
                let result: i32 = row.try_get("health_check")?;
                if result == 1 {
                    info!("Database health check passed");
                    Ok(())
                } else {
                    error!("Database health check failed: unexpected result {}", result);
                    anyhow::bail!("Database health check failed: unexpected result")
                }
            }
            Err(e) => {
                error!("Database health check failed: {}", e);
                Err(e.into())
            }
        }
    }

    /// Run pending migrations
    pub async fn migrate(&self) -> Result<()> {
        info!("Running database migrations...");
        
        sqlx::migrate!("./migrations")
            .run(&self.pool)
            .await
            .context("Failed to run database migrations")?;
        
        info!("Database migrations completed successfully");
        Ok(())
    }

    /// Close the database connection pool
    pub async fn close(self) {
        info!("Closing database connection pool...");
        self.pool.close().await;
        info!("Database connection pool closed");
    }

    /// Create a new order in the database
    pub async fn create_order(&self, id: &str, item_name: &str, quantity: i32) -> Result<Order> {
        info!("Creating order in database: {}", id);
        
        let order = sqlx::query_as!(
            Order,
            r#"
            INSERT INTO orders (id, item_name, quantity, status)
            VALUES ($1, $2, $3, 'pending')
            RETURNING id, item_name, quantity, status, created_at, updated_at
            "#,
            id,
            item_name,
            quantity
        )
        .fetch_one(&self.pool)
        .await
        .context("Failed to create order")?;

        info!("Order created successfully: {}", order.id);
        Ok(order)
    }

    /// Get an order by ID
    pub async fn get_order(&self, id: &str) -> Result<Option<Order>> {
        let order = sqlx::query_as!(
            Order,
            "SELECT id, item_name, quantity, status, created_at, updated_at FROM orders WHERE id = $1",
            id
        )
        .fetch_optional(&self.pool)
        .await
        .context("Failed to fetch order")?;

        Ok(order)
    }

    /// List all orders
    pub async fn list_orders(&self) -> Result<Vec<Order>> {
        let orders = sqlx::query_as!(
            Order,
            "SELECT id, item_name, quantity, status, created_at, updated_at FROM orders ORDER BY created_at DESC"
        )
        .fetch_all(&self.pool)
        .await
        .context("Failed to fetch orders")?;

        Ok(orders)
    }
}

/// Order entity representing a warehouse order
#[derive(Debug, Clone, FromRow)]
pub struct Order {
    pub id: String,
    pub item_name: String,
    pub quantity: i32,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Mask sensitive information in database URL for logging
fn mask_database_url(url: &str) -> String {
    if let Ok(parsed) = url::Url::parse(url) {
        let host = parsed.host_str().unwrap_or("unknown");
        let port = parsed.port().map(|p| format!(":{}", p)).unwrap_or_default();
        let path = parsed.path();
        format!("postgresql://***:***@{}{}{}", host, port, path)
    } else {
        "postgresql://***:***@unknown/unknown".to_string()
    }
}

/// Initialize tracing for the database module
pub fn init_logging() {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "wms_db=info,sqlx=warn");
    }
    
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .try_init()
        .ok(); // Ignore error if already initialized
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio_test;

    #[test]
    fn test_database_config_default() {
        let config = DatabaseConfig::default();
        assert_eq!(config.database_url, "postgresql://localhost/wms_dev");
        assert_eq!(config.max_connections, 10);
        assert_eq!(config.min_connections, 1);
    }

    #[test]
    fn test_mask_database_url() {
        let url = "postgresql://user:password@localhost:5432/mydb";
        let masked = mask_database_url(url);
        assert_eq!(masked, "postgresql://***:***@localhost:5432/mydb");
    }

    #[test]
    fn test_mask_invalid_url() {
        let url = "invalid-url";
        let masked = mask_database_url(url);
        assert_eq!(masked, "postgresql://***:***@unknown/unknown");
    }

    // Integration tests - only run if DATABASE_URL is set
    #[tokio::test]
    #[ignore] // Ignored by default, run with --ignored flag
    async fn test_database_connection() {
        init_logging();
        
        // Skip test if DATABASE_URL is not set
        if std::env::var("DATABASE_URL").is_err() {
            eprintln!("Skipping integration test: DATABASE_URL not set");
            return;
        }

        let db = Database::from_env().await.expect("Failed to connect to database");
        db.health_check().await.expect("Health check failed");
        db.close().await;
    }

    #[tokio::test]
    #[ignore] // Ignored by default, run with --ignored flag  
    async fn test_database_migration() {
        init_logging();
        
        // Skip test if DATABASE_URL is not set
        if std::env::var("DATABASE_URL").is_err() {
            eprintln!("Skipping integration test: DATABASE_URL not set");
            return;
        }

        let db = Database::from_env().await.expect("Failed to connect to database");
        db.migrate().await.expect("Migrations failed");
        db.close().await;
    }

    #[tokio::test]
    #[ignore] // Ignored by default, run with --ignored flag
    async fn test_create_order() {
        init_logging();
        
        // Skip test if DATABASE_URL is not set
        if std::env::var("DATABASE_URL").is_err() {
            eprintln!("Skipping integration test: DATABASE_URL not set");
            return;
        }

        let db = Database::from_env().await.expect("Failed to connect to database");
        db.migrate().await.expect("Migrations failed");

        // Create a test order
        let order_id = "ORD-TEST-001";
        let item_name = "Test Item";
        let quantity = 5;

        let order = db.create_order(order_id, item_name, quantity)
            .await
            .expect("Failed to create order");

        assert_eq!(order.id, order_id);
        assert_eq!(order.item_name, item_name);
        assert_eq!(order.quantity, quantity);
        assert_eq!(order.status, "pending");

        // Test get_order
        let retrieved_order = db.get_order(order_id)
            .await
            .expect("Failed to get order")
            .expect("Order not found");

        assert_eq!(retrieved_order.id, order.id);
        assert_eq!(retrieved_order.item_name, order.item_name);
        assert_eq!(retrieved_order.quantity, order.quantity);

        // Clean up - delete the test order
        sqlx::query!("DELETE FROM orders WHERE id = $1", order_id)
            .execute(&db.pool)
            .await
            .expect("Failed to clean up test order");

        db.close().await;
    }

    #[tokio::test]
    #[ignore] // Ignored by default, run with --ignored flag
    async fn test_list_orders() {
        init_logging();
        
        // Skip test if DATABASE_URL is not set
        if std::env::var("DATABASE_URL").is_err() {
            eprintln!("Skipping integration test: DATABASE_URL not set");
            return;
        }

        let db = Database::from_env().await.expect("Failed to connect to database");
        db.migrate().await.expect("Migrations failed");

        // Create test orders
        let order1_id = "ORD-TEST-LIST-001";
        let order2_id = "ORD-TEST-LIST-002";

        let _order1 = db.create_order(order1_id, "Test Item 1", 3)
            .await
            .expect("Failed to create order 1");

        let _order2 = db.create_order(order2_id, "Test Item 2", 7)
            .await
            .expect("Failed to create order 2");

        // Test list_orders
        let orders = db.list_orders()
            .await
            .expect("Failed to list orders");

        assert!(orders.len() >= 2);
        assert!(orders.iter().any(|o| o.id == order1_id));
        assert!(orders.iter().any(|o| o.id == order2_id));

        // Clean up - delete the test orders
        sqlx::query!("DELETE FROM orders WHERE id IN ($1, $2)", order1_id, order2_id)
            .execute(&db.pool)
            .await
            .expect("Failed to clean up test orders");

        db.close().await;
    }

    #[test]
    fn test_order_struct() {
        use chrono::Utc;
        
        let order = Order {
            id: "ORD-123".to_string(),
            item_name: "Test Item".to_string(),
            quantity: 5,
            status: "pending".to_string(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        assert_eq!(order.id, "ORD-123");
        assert_eq!(order.item_name, "Test Item");
        assert_eq!(order.quantity, 5);
        assert_eq!(order.status, "pending");
    }
}
