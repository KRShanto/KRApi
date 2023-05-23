use crate::schema::users;
use crate::*;
use argon2::{
    password_hash::{PasswordHash, PasswordVerifier},
    Argon2,
};

#[derive(Deserialize, Clone)]
pub struct MatchUser {
    pub username: String,
    pub password: String,
}

/// Verify the user's password
///
/// ## Route
///
/// `POST` localhost:8090/verify-user
///
/// ## Body
///
/// ```json
/// {
///     "username": string,
///     "password": string
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
/// const res = await fetch("http://localhost:8090/verify-user", {
///   method: "POST",
///   headers: {
///    "Content-Type": "application/json",
/// },
///   body: JSON.stringify({
///     username: "shanto",
///     password: "admin005"
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
///   "type": "Success",
///   "msg": "Login successful"
/// }
/// ```
#[post("/verify-user")]
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
                return Response::not_found()
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
