#[macro_use]
extern crate diesel;

pub mod models;
pub mod routes;
pub mod schema;
pub mod utils;

pub type DbPool = diesel::r2d2::Pool<diesel::r2d2::ConnectionManager<diesel::SqliteConnection>>;
