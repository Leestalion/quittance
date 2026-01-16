use sqlx::{postgres::PgPoolOptions, PgPool};
use std::env;
use std::time::Duration;

#[derive(Clone)]
pub struct Database {
    pub pool: PgPool,
}

impl Database {
    pub async fn new() -> Result<Self, sqlx::Error> {
        let database_url = env::var("DATABASE_URL")
            .expect("DATABASE_URL must be set in .env file");

        // Retry configuration
        let max_retries = 10;
        let initial_delay = Duration::from_secs(2);
        let max_delay = Duration::from_secs(30);

        let mut retry_count = 0;
        let mut delay = initial_delay;

        loop {
            tracing::info!(
                "🔌 Attempting to connect to database (attempt {}/{})",
                retry_count + 1,
                max_retries
            );

            match PgPoolOptions::new()
                .max_connections(10)
                .min_connections(0) // Don't keep idle connections
                .acquire_timeout(Duration::from_secs(10))
                .idle_timeout(Duration::from_secs(60)) // Close idle connections after 1 min
                .max_lifetime(Duration::from_secs(1800)) // Recycle connections after 30 min
                .test_before_acquire(true) // Test connections before using them (enables auto-reconnect)
                .connect(&database_url)
                .await
            {
                Ok(pool) => {
                    tracing::info!("✅ Successfully connected to database");
                    return Ok(Database { pool });
                }
                Err(e) => {
                    retry_count += 1;
                    
                    if retry_count >= max_retries {
                        tracing::error!(
                            "❌ Failed to connect to database after {} attempts: {}",
                            max_retries,
                            e
                        );
                        return Err(e);
                    }

                    tracing::warn!(
                        "⚠️  Database connection failed: {}. Retrying in {:?}...",
                        e,
                        delay
                    );

                    tokio::time::sleep(delay).await;

                    // Exponential backoff with cap
                    delay = std::cmp::min(delay * 2, max_delay);
                }
            }
        }
    }

    pub async fn run_migrations(&self) -> Result<(), sqlx::Error> {
        sqlx::migrate!("./migrations")
            .run(&self.pool)
            .await?;
        
        tracing::info!("✅ Database migrations completed");
        Ok(())
    }
}
