use crate::*;

#[get("/get-user/{id}")]
pub async fn route(pool: web::Data<DbPool>, path: Path<i32>) -> HttpResponse {
    let mut db_connection = pool.get().unwrap();
    let id = path.into_inner();

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
        Err(_) => Response::server_error().send(),
    }
}
