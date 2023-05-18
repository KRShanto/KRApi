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

// Update the user
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
            Ok(_) => Response::success().send(),
            Err(e) => {
                server_error(e);
                Response::server_error().send()
            }
        },
        Err(_) => Response::server_error().send(),
    }
}
