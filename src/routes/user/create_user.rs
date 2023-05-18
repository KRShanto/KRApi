use crate::schema::users;
use crate::*;
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2,
};

#[post("/create-user")]
pub async fn route(pool: web::Data<DbPool>, item: web::Json<UserNew>) -> HttpResponse {
    let mut db_connection = pool.get().unwrap();
    let new_user = item.into_inner();

    // Check if username or email already exists
    let user_exists_result = web::block({
        let new_user = new_user.clone();
        let mut db_connection = pool.get().unwrap();

        move || {
            users::table
                .filter(users::username.eq(&new_user.username))
                .or_filter(users::email.eq(&new_user.email))
                .first::<User>(&mut db_connection)
        }
    });

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

    let argon2 = Argon2::default();
    let salt = SaltString::generate(&mut OsRng);

    // hashed password
    let hash = argon2
        .hash_password(new_user.password.as_bytes(), &salt)
        .unwrap()
        .to_string();

    let new_user = UserNew {
        password: hash,
        ..new_user
    };

    let result = web::block(move || {
        let insert_result = diesel::insert_into(crate::schema::users::table)
            .values(&new_user)
            .execute(&mut db_connection);

        match insert_result {
            Ok(_) => (),
            Err(e) => {
                server_error(e);
            }
        }

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
