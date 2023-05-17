use crate::models::users::{User, UserJson, UserNew};
use crate::DbPool;

use actix_web::{post, web, Error, HttpResponse};
use anyhow::Result;
use diesel::dsl::insert_into;
use diesel::prelude::*;
use diesel::RunQueryDsl;

#[post("/create-user")]
pub async fn route(
    pool: web::Data<DbPool>,
    item: web::Json<UserNew>,
) -> Result<HttpResponse, Error> {
    let mut db_connection = pool.get().unwrap();
    let new_user = item.into_inner();

    // Use web::block to offload blocking Diesel code without blocking server thread
    let result = web::block(move || {
        insert_into(crate::schema::users::table)
            .values(&new_user)
            .execute(&mut db_connection)
            .unwrap();

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
            created_at: user.created_at.unwrap(),
        })
    });

    match result.await {
        Ok(user) => Ok(HttpResponse::Ok().json(user.unwrap())),
        Err(_) => Ok(HttpResponse::InternalServerError().into()),
    }
}
