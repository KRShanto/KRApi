use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::web::Data;
use actix_web::{App, HttpServer};
use krapi::routes::*;
use krapi::utils::get_pool;

// pub type Pool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    let pool = get_pool();

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_header()
                    .allow_any_method()
                    .supports_credentials(),
            )
            .app_data(Data::new(pool.clone()))
            .service(get_users_route)
            .service(create_user_route)
            .service(match_user_route)
            .service(update_password_route)
    })
    .bind("localhost:8080")?
    .run()
    .await
}
