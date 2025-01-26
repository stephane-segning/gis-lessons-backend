use crate::modules::utils::err::app_panic;
use anyhow::Result;
use diesel::Connection;
use diesel_async::async_connection_wrapper::AsyncConnectionWrapper;
use diesel_async::AsyncPgConnection;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

#[inline]
pub async fn run_migrations(database_url: String) -> Result<()> {
    tokio::task::spawn_blocking(move || {
        let mut conn = AsyncConnectionWrapper::<AsyncPgConnection>::establish(&database_url)
            .unwrap_or_else(|e| app_panic(format!("Failed to connect to db: {}", e)));
        conn.run_pending_migrations(MIGRATIONS)
            .unwrap_or_else(|e| app_panic(format!("Failed to run migrations: {}", e)));
    })
    .await?;
    Ok(())
}
