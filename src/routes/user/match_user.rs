use crate::models::users::User;
use crate::schema::users;
use crate::utils::response::Response;
use crate::utils::server_error;
use crate::*;

use actix_web::{post, web, HttpResponse};
use argon2::{
    password_hash::{PasswordHash, PasswordVerifier},
    Argon2,
};
use diesel::prelude::*;

#[derive(Deserialize, Clone)]
pub struct MatchUser {
    pub username: String,
    pub password: String,
}

#[post("/match-user")]
pub async fn route(pool: web::Data<DbPool>, item: web::Json<MatchUser>) -> HttpResponse {
    let user_info = item.into_inner();

    // Check if the user exists
    let user_exists = web::block({
        let user_info = user_info.clone();
        let mut db_connection = pool.get().unwrap();

        move || {
            users::table
                .filter(users::username.eq(&user_info.username))
                .first::<User>(&mut db_connection)
        }
    });

    let user: User = match user_exists.await {
        Ok(user_result) => match user_result {
            Ok(user) => user,
            Err(_) => {
                return Response::incorrect_password()
                    .msg("Username or password is incorrect")
                    .send();
            }
        },
        Err(e) => {
            server_error(e);
            return Response::server_error().send();
        }
    };

    // Verify password

    let password = user_info.password.as_bytes();
    let hash = PasswordHash::new(&user.password).unwrap();

    let is_valid = Argon2::default().verify_password(password, &hash).is_ok();

    if !is_valid {
        return Response::incorrect_password()
            .msg("Username or password is incorrect")
            .send();
    }

    Response::success().msg("Login successful").send()
}
