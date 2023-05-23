use crate::*;

/// Get all users
///
/// ## Route
///
/// `GET` localhost:8090/get-users
///
/// ## Returns
///
/// - If successful, returns [`ResponseType::Success`](crate::utils::response::ResponseType::Success) with the data [`Vec<UserJson>`].
///
/// - If any error occurs, returns [`ResponseType::ServerError`](crate::utils::response::ResponseType::ServerError).
///
/// ## Example
///
/// Javascript Fetch API
///
/// ```js
/// const res = await fetch("http://localhost:8090/get-users");
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
///     "type": "Success",
///     "msg": null,
///     "data": [
///         {
///           "created_at": "2023-05-21T07:30:48",
///           "email": "shanto@gmail.com",
///           "id": 223,
///           "img_url": null,
///           "name": "Shanto Islam",
///           "phone": null,
///           "username": "shanto"
///         },
///         {
///           "created_at": "2023-05-20T16:22:17",
///           "email": "oleta_iste@hotmail.com",
///           "id": 222,
///           "img_url": null,
///           "name": "Gunner Hettinger",
///           "phone": null,
///           "username": "otis_quaerat"
///         },
///         {
///           "created_at": "2023-05-20T16:22:17",
///           "email": null,
///           "id": 221,
///           "img_url": null,
///           "name": "Ursula Ruecker",
///           "phone": null,
///           "username": "dianna_sed"
///         }
///      ]
/// }
/// ```
#[get("/get-users")]
pub async fn route(pool: web::Data<DbPool>) -> HttpResponse {
    let mut db_connection = pool.get().unwrap();

    // Get all users
    let result = web::block(move || {
        crate::schema::users::table
            .order(crate::schema::users::id.desc())
            .load::<User>(&mut db_connection)
    });

    match result.await {
        Ok(users_result) => match users_result {
            Ok(users) => {
                let users_json = users
                    .into_iter()
                    .map(|user| UserJson {
                        id: user.id,
                        name: user.name,
                        username: user.username,
                        email: user.email,
                        img_url: user.img_url,
                        phone: user.phone,
                        created_at: user.created_at,
                    })
                    .collect::<Vec<UserJson>>();

                Response::success().data(users_json).send()
            }
            Err(e) => {
                server_error(e);
                Response::server_error().send()
            }
        },
        Err(_) => Response::server_error().send(),
    }
}
