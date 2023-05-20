use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2,
};

pub fn hash_password(password: impl AsRef<str>) -> String {
    let argon2 = Argon2::default();
    let salt = SaltString::generate(&mut OsRng);

    // hashed password
    argon2
        .hash_password(password.as_ref().as_bytes(), &salt)
        .unwrap()
        .to_string()
}
