use crate::schema::users;
use crate::utils::hash::hash_password;
use crate::*;

/// Create a new user
///
/// ## Route
///
/// `POST` localhost:8090/create-user
///
/// ## Body
///
/// ```json
/// {
///     "name": string,
///     "username": string,
///     "password": string,
///     "email": string (optional),
///     "img_url": string (optional),
///     "phone": number (optional)
/// }
/// ```
/// Required fields: `name`, `username` and `password`
///
/// ## Returns
///
/// - If successful, returns [`ResponseType::Success`](crate::utils::response::ResponseType::Success) with the data [`UserJson`].
///
/// - If user already exists (matched by `username` and `email`) then returns [`ResponseType::AlreadyExists`](crate::utils::response::ResponseType::AlreadyExists).
///
/// - If any error occurs, returns [`ResponseType::ServerError`](crate::utils::response::ResponseType::ServerError).
///
/// ## Example
///
/// Javascript Fetch API
///
/// ```js
/// const res = await fetch("http://localhost:8090/create-user", {
///    method: "POST",
///    headers: {
///     "Content-Type": "application/json",
///   },
///    body: JSON.stringify({
///      name: "Shanto Islam",
///      username: "shanto",
///      email: "shanto@gmail.com",
///      password: "admin005"
///    }),
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
#[post("/create-user")]
pub async fn route(pool: web::Data<DbPool>, item: web::Json<UserNew>) -> HttpResponse {
    let mut db_connection = pool.get().unwrap();
    let new_user = item.into_inner();

    // Check if username or email already exists
    let user_exists_result = web::block({
        let new_user = new_user.clone();
        let mut db_connection = pool.get().unwrap();

        // match by username and email
        move || {
            users::table
                .filter(users::username.eq(&new_user.username))
                .or_filter(users::email.eq(&new_user.email))
                .first::<User>(&mut db_connection)
        }
    });

    // If user exists, return error
    match user_exists_result.await {
        Ok(user_exists) => {
            if user_exists.is_ok() {
                return Response::already_exists()
                    .msg("Username or email already exists man")
                    .send();
            }
        }
        Err(e) => {
            server_error(e);
        }
    }

    // Hash password
    let hash = hash_password(&new_user.password);

    let new_user = UserNew {
        password: hash,
        ..new_user
    };

    // Insert user into database
    let result = web::block(move || {
        let insert_result = diesel::insert_into(crate::schema::users::table)
            .values(&new_user)
            .execute(&mut db_connection);

        if let Err(e) = insert_result {
            server_error(e);
        }

        // Get the user
        let user = crate::schema::users::table
            .order(crate::schema::users::id.desc())
            .first::<User>(&mut db_connection);

        user.map(|user| UserJson {
            id: user.id,
            name: user.name,
            username: user.username,
            email: user.email,
            img_url: user.img_url,
            phone: user.phone,
            created_at: user.created_at,
        })
    });

    // Return response
    match result.await {
        Ok(user_result) => match user_result {
            Ok(user) => Response::success().data(user).send(),
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
