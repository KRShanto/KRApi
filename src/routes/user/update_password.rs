use crate::schema::users;
use crate::*;
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};

#[derive(Deserialize, Clone)]
pub struct Password {
    username: String,
    password: String,
    new_password: String,
}

/// Update the user's password
///
/// ## Route
///
/// `POST` localhost:8090/update-password
///
/// ## Body
///
/// ```json
/// {
///    "username": string,
///    "password": string,
///    "new_password": string
/// }
/// ```
///
/// ## Returns
///
/// - If successful, returns [`ResponseType::Success`](crate::utils::response::ResponseType::Success).
///
/// - If user not found, returns [`ResponseType::NotFound`](crate::utils::response::ResponseType::NotFound).
///
/// - If password is incorrect, returns [`ResponseType::IncorrectPassword`](crate::utils::response::ResponseType::IncorrectPassword).
///
/// - If any error occurs, returns [`ResponseType::ServerError`](crate::utils::response::ResponseType::ServerError).
///
/// ## Example
///
/// First you need to create a user. See [`create_user`](crate::routes::create_user_route) route.
///
/// Lets say you have a user with username `shanto` and password `admin005`.
///
/// Javascript Fetch API
///
/// ```js
/// const res = await fetch("http://localhost:8090/update-password", {
///   method: "POST",
///   headers: {
///     "Content-Type": "application/json",
/// },
///   body: JSON.stringify({
///      username: "shanto",
///      password: "admin005",
///      new_password: "admin006"
///  }),
/// });
///
/// const json = await res.json();
/// const data = json.data;
///
/// console.log(data);
/// ```
///
/// ## Example Response
///
/// ```json
/// {
///    "type": "Success",
///    "msg": "Password updated successfully"
/// }
/// ```
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
                // user not found
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
