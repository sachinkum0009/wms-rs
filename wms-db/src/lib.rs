use color_eyre::eyre::{Context, Result};
use dotenv::dotenv;
use sqlx::{postgres::PgPoolOptions, PgPool, Pool, Postgres, Row};
use std::env;
use std::time::Duration;
use tracing::{info, error, warn};
use tracing_subscriber::filter::EnvFilter;

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
                    color_eyre::eyre::bail!("Database health check failed: unexpected result")
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
        .with_env_filter(EnvFilter::from_default_env())
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
}
