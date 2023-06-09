use crate::schema::*;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Queryable, Clone)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub username: String,
    pub email: Option<String>,
    pub img_url: Option<String>,
    pub phone: Option<f64>,
    pub password: String,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, Insertable, Clone)]
#[diesel(table_name = users)]
pub struct UserNew {
    pub name: String,
    pub username: String,
    pub email: Option<String>,
    pub img_url: Option<String>,
    pub phone: Option<f64>,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserJson {
    pub id: i32,
    pub name: String,
    pub username: String,
    pub email: Option<String>,
    pub img_url: Option<String>,
    pub phone: Option<f64>,
    pub created_at: NaiveDateTime,
}
