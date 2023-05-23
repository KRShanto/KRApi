use crate::schema::users;
use crate::*;

#[derive(Deserialize, Clone)]
pub struct UserUpdate {
    username: String, // For querying
    name: Option<String>,
    email: Option<String>,
    phone: Option<f64>,
    img_url: Option<String>,
}

/// Update the user
///
/// This route will update these fields:
///
/// - `name`
/// - `email`
/// - `phone`
/// - `img_url`
///
/// For updating the password, see [`update_password`](crate::routes::update_password_route) route.
///
/// ## Route
///
/// `POST` localhost:8090/update-user
///
/// ## Body
///
/// ```json
/// {
///    "username": string,
///    "name": string (optional),
///    "email": string (optional),
///    "phone": number (optional),
///    "img_url": string (optional)
/// }
/// ```
///
/// The `username` field is required. This is used to query the user. Whichever field you want to update, just add it to the body.
///
/// ## Returns
///
/// - If successful, returns [`ResponseType::Success`](crate::utils::response::ResponseType::Success).
///
/// - If user not found, returns [`ResponseType::NotFound`](crate::utils::response::ResponseType::NotFound).
///
/// - If any error occurs, returns [`ResponseType::ServerError`](crate::utils::response::ResponseType::ServerError).
///
/// ## Example
///
/// First you need to create a user. See [`create_user`](crate::routes::create_user_route) route.
///
/// Lets say you have a user with username `shanto` and password `admin005` and email `shanto@xyz.com`.
///
/// Javascript Fetch API
///
/// ```js
/// const res = await fetch("http://localhost:8090/update-user", {
///   method: "POST",
///   headers: {
///     "Content-Type": "application/json",
///   },
///   body: JSON.stringify({
///     username: "shanto",
///     email: "shanto@abc.com",
///  }),
/// });
///
/// const json = await res.json();
/// const data = json.data;
///
/// console.log(data);
/// ```
/// ## Example Response
///
/// ```json
/// {
///    "type": "Success",
///    "msg": "Update successful"
/// }
/// ```
///
/// Now if you query the user using [`get_user`](crate::routes::get_user_route) route, you will see that the email has been updated.
#[post("/update-user")]
pub async fn route(pool: Data<DbPool>, item: Json<UserUpdate>) -> HttpResponse {
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

    // Update the user
    let user_update = web::block({
        let user_info = user_info.clone();
        let mut db_connection = pool.get().unwrap();

        move || {
            diesel::update(users::table.find(user.id))
                .set((
                    users::name.eq(user_info.name.unwrap_or(user.name)),
                    users::email.eq(if user_info.email.is_some() {
                        user_info.email
                    } else {
                        user.email
                    }),
                    users::phone.eq(if user_info.phone.is_some() {
                        user_info.phone
                    } else {
                        user.phone
                    }),
                    users::img_url.eq(if user_info.img_url.is_some() {
                        user_info.img_url
                    } else {
                        user.img_url
                    }),
                ))
                .execute(&mut db_connection)
        }
    });

    match user_update.await {
        Ok(user_result) => match user_result {
            Ok(_) => Response::success().msg("Update successful").send(),
            Err(e) => {
                server_error(e);
                Response::server_error().send()
            }
        },
        Err(_) => Response::server_error().send(),
    }
}
