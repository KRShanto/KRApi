use crate::*;
use diesel::r2d2::{self, ConnectionManager};
use home::home_dir;

pub fn establish_connection() -> DbPool {
    let db_path = if cfg!(debug_assertions) {
        // On debug, use the working directory
        format!(
            "{}/{}",
            std::env::current_dir().unwrap().to_str().unwrap(),
            DATABASE_PATH
        )
    } else {
        // On release, use user's home directory
        format!(
            "{}/.config/{}",
            home_dir().unwrap().to_str().unwrap(),
            DATABASE_PATH
        )
    };

    // Sqlite connection manager
    let manager = ConnectionManager::<SqliteConnection>::new(db_path);

    let pool: DbPool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    pool
}
