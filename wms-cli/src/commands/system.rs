use color_eyre::eyre::Result;
use tracing::{error, info};
use wms_db::Database;

/// Check system health including database connectivity
pub async fn health() -> Result<()> {
    info!("Running system health check...");

    // Initialize database connection
    match Database::from_env().await {
        Ok(db) => {
            info!("✅ Database connection established");

            // Run database health check
            match db.health_check().await {
                Ok(()) => {
                    info!("✅ Database health check passed");
                    info!("🎉 System health check completed successfully");
                }
                Err(e) => {
                    error!("❌ Database health check failed: {}", e);
                    return Err(e.into());
                }
            }

            // Close database connection gracefully
            db.close().await;
        }
        Err(e) => {
            error!("❌ Failed to establish database connection: {}", e);
            error!("💡 Make sure your .env file is configured with DATABASE_URL");
            return Err(e.into());
        }
    }

    Ok(())
}
