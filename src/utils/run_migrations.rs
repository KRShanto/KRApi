use diesel::sqlite::Sqlite;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use std::error::Error;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");

/// Run migrations at startup
pub fn run_migrations(
    connection: &mut impl MigrationHarness<Sqlite>,
) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    // This will run the necessary migrations.
    connection.run_pending_migrations(MIGRATIONS)?;

    Ok(())
}
