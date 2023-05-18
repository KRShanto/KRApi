use crate::models::users::User;
use crate::schema::users;
use crate::utils::response::Response;
use crate::utils::server_error;
use crate::*;

use actix_web::{post, web, HttpResponse};
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use diesel::prelude::*;

#[derive(Deserialize, Clone)]
pub struct Password {
    username: String,
    password: String,
    new_password: String,
}

#[post("/update-password")]
pub async fn route(pool: web::Data<DbPool>, item: web::Json<Password>) -> HttpResponse {
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
                return Response::not_found().msg("User not found").send();
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

    // Update password
    let new_password = user_info.new_password.as_bytes();
    let argon2 = Argon2::default();
    let salt = SaltString::generate(&mut OsRng);

    // hashed password
    let hash = argon2
        .hash_password(new_password, &salt)
        .unwrap()
        .to_string();

    let update_password = web::block({
        let user_info = user_info.clone();
        let mut db_connection = pool.get().unwrap();

        move || {
            diesel::update(users::table)
                .filter(users::username.eq(&user_info.username))
                .set(users::password.eq(&hash))
                .execute(&mut db_connection)
        }
    });

    match update_password.await {
        Ok(_) => (),
        Err(e) => {
            server_error(e);
            return Response::server_error().send();
        }
    }

    Response::success()
        .msg("Password updated successful")
        .send()
}
