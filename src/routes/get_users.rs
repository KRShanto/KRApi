use crate::models::users::{User, UserJson};
use crate::DbPool;

use actix_web::get;
use actix_web::{web, Error, HttpResponse};
use anyhow::Result;
use diesel::prelude::*;
use diesel::RunQueryDsl;

#[get("/get-users")]
pub async fn route(pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    let mut db_connection = pool.get().unwrap();

    // Use web::block to offload blocking Diesel code without blocking server thread
    let result = web::block(move || {
        crate::schema::users::table
            .order(crate::schema::users::id.desc())
            .load::<User>(&mut db_connection)
    });

    match result.await {
        Ok(users) => {
            let users_json: Vec<UserJson> = users
                .unwrap()
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
                .collect();

            Ok(HttpResponse::Ok().json(users_json))
        }
        Err(_) => Ok(HttpResponse::InternalServerError().into()),
    }
}
