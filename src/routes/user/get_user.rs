use crate::*;

/// Get a user by id
///
/// ## Route
///
/// `GET` localhost:8090/get-user/{id}
///
/// Here, `{id}` is the id of the user. It is an integer.
///
/// ## Returns
///
/// - If successful, returns [`ResponseType::Success`](crate::utils::response::ResponseType::Success) with the data [`UserJson`].
///
/// - If user does not exist, returns [`ResponseType::NotFound`](crate::utils::response::ResponseType::NotFound).
///
/// - If any error occurs, returns [`ResponseType::ServerError`](crate::utils::response::ResponseType::ServerError).
///
/// ## Example
///
/// Javascript Fetch API
///
/// ```js
/// const res = await fetch("http://localhost:8090/get-user/223");
///
/// const json = await res.json();
/// const data = json.data;
///
/// console.log(data);
/// ```
///
/// ## Example Response
///
/// Suppose the user with id `223` exists.
///
/// ```json
/// {
///     "type": "Success",
///     "msg": null,
///     "data": {
///       "name": "Shanto Islam",
///       "username": "shanto",
///       "email": "shanto@gmail.com",
///       "created_at": "2023-05-21T07:30:48",
///       "id": 223,
///       "img_url": null,
///       "phone": null
///     }
/// }
/// ```
#[get("/get-user/{id}")]
pub async fn route(pool: web::Data<DbPool>, path: Path<i32>) -> HttpResponse {
    let mut db_connection = pool.get().unwrap();
    let id = path.into_inner();

    // Get user by id
    let result = web::block(move || {
        crate::schema::users::table
            .filter(crate::schema::users::id.eq(id))
            .load::<User>(&mut db_connection)
    });

    match result.await {
        Ok(users_result) => match users_result {
            Ok(users) => {
                // if users is empty, return notfound
                if users.is_empty() {
                    return Response::not_found().send();
                }

                let user = &users[0];
                let user_json = UserJson {
                    id: user.id,
                    name: user.name.clone(),
                    username: user.username.clone(),
                    email: user.email.clone(),
                    img_url: user.img_url.clone(),
                    phone: user.phone,
                    created_at: user.created_at,
                };

                Response::success().data(user_json).send()
            }
            Err(e) => {
                server_error(e);
                Response::server_error().send()
            }
        },
        Err(e) => {
            server_error(e);
            Response::server_error().send()
        }
    }
}
