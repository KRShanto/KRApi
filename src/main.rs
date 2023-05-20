use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::web::Data;
use actix_web::{App, HttpServer};
use clap::Parser;
use krapi::cli::{Cli, SubCommand};
use krapi::routes::*;
use krapi::utils::{establish_connection, generate_users, run_migrations};
use log::info;
use std::env;

#[actix_rt::main]
async fn main() {
    dotenv::dotenv().ok();
    env::set_var("RUST_LOG", "krapi");

    // on debug mode, we want to see the logs
    if cfg!(debug_assertions) {
        env::set_var("RUST_LOG", "actix_web=info,error,debug,warn,krapi");
    }
    env_logger::init();

    let cli = Cli::parse();

    match cli.subcmd {
        SubCommand::Start { port } => {
            start_server(port).await.unwrap();
        }
        SubCommand::Generate {
            len,
            users,
            posts,
            todos,
        } => {
            if users {
                println!("Generating {} users", len);

                let result = generate_users(len, establish_connection()).await;
                if result.is_ok() {
                    println!("Generated {} users successfully :)", len);
                }
            } else if posts {
                println!("Generate posts");
            } else if todos {
                println!("Generate todos");
            } else {
                println!("Nothing to generate");
                println!("Use --users, --posts or --todos")
            }
        }
        SubCommand::Docs {
            users,
            posts,
            todos,
        } => {
            println!("Show docs");
            if users {
                println!("Show docs for users");
            }
            if posts {
                println!("Show docs for posts");
            }
            if todos {
                println!("Show docs for todos");
            }
        }
    }
}

async fn start_server(port: u16) -> std::io::Result<()> {
    // Get the connection
    let connection = establish_connection();

    // run the pending migrations
    run_migrations(&mut connection.get().unwrap()).unwrap();

    info!("Starting server at: http://localhost:{}", port);

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
            .app_data(Data::new(connection.clone()))
            // users
            .service(get_users_route)
            .service(create_user_route)
            .service(match_user_route)
            .service(update_password_route)
            .service(update_user_route)
            .service(get_user_route)
            // greet
            .service(greet_route)
    })
    .bind(&format!("localhost:{}", port))?
    .run()
    .await
}
