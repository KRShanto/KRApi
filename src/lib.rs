#![doc = include_str!("../README.md")]

#[macro_use]
extern crate diesel;

pub const DEFAULT_PORT: usize = 8090;
pub const DEFAULT_PORT_STR: &str = "8090";

pub const DATABASE_PATH: &str = "krapi.sqlite";

pub const DEFAULT_MOCK_DATA_LEN: usize = 10;
pub const DEFAULT_MOCK_DATA_LEN_STR: &str = "10";

pub mod cli;
pub mod models;
pub mod routes;
pub mod schema;
pub mod utils;

pub(crate) use actix_web::{
    get, post,
    web::{self, Data, Json, Path},
    HttpResponse,
};
pub(crate) use diesel::prelude::*;
pub(crate) use diesel::SqliteConnection;
pub(crate) use models::users::*;
pub(crate) use serde::{Deserialize, Serialize};
pub(crate) use utils::response::Response;
pub(crate) use utils::server_error;

pub type DbPool = diesel::r2d2::Pool<diesel::r2d2::ConnectionManager<SqliteConnection>>;
