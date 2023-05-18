#[macro_use]
extern crate diesel;

pub mod models;
pub mod routes;
pub mod schema;
pub mod utils;

pub use actix_web::{
    get, post,
    web::{self, Data, Json},
    HttpResponse,
};
pub use diesel::prelude::*;
pub use serde::{Deserialize, Serialize};
pub use utils::response::Response;
pub use utils::server_error;

pub use models::users::*;

pub type DbPool = diesel::r2d2::Pool<diesel::r2d2::ConnectionManager<diesel::SqliteConnection>>;
