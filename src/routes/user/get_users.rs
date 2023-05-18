use crate::models::users::{User, UserJson};
use crate::utils::response::Response;
use crate::utils::server_error;
use crate::DbPool;

use actix_web::get;
use actix_web::{web, HttpResponse};
use diesel::prelude::*;
use diesel::RunQueryDsl;

#[get("/get-users")]
pub async fn route(pool: web::Data<DbPool>) -> HttpResponse {
    let mut db_connection = pool.get().unwrap();

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
                        created_at: user.created_at.unwrap(),
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
