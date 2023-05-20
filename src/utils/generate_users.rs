use crate::*;
use fake::faker::internet::raw::*;
use fake::faker::name::raw::*;
// use fake::faker::phone_number::en::*;
use fake::locales::EN;
use fake::Fake;

use super::hash::hash_password;

pub async fn generate_users(len: u32, conn: DbPool) -> Result<Vec<User>, ()> {
    let generated_users = (0..len)
        .map(|_| UserNew {
            name: Name(EN).fake(),
            username: Username(EN).fake(),
            email: FreeEmail(EN).fake(),
            password: hash_password(Password(EN, 8..16).fake::<String>()),
            phone: None, // TODO: Add phone number
            img_url: None,
        })
        .collect::<Vec<UserNew>>();

    let mut db_connection = conn.get().unwrap();

    let result = web::block(move || {
        let insert_result = diesel::insert_into(crate::schema::users::table)
            .values(&generated_users)
            .execute(&mut db_connection);

        match insert_result {
            Ok(_) => (),
            Err(e) => {
                server_error(e);
            }
        }

        crate::schema::users::table
            .order(crate::schema::users::id.desc())
            .limit(len as i64)
            .load::<User>(&mut db_connection)
    });

    match result.await {
        Ok(users_result) => match users_result {
            Ok(users) => Ok(users),
            Err(e) => {
                server_error(e);
                Err(())
            }
        },
        Err(e) => {
            server_error(e);
            Err(())
        }
    }
}
