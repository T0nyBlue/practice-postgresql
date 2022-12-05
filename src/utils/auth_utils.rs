use argon2::{self, Config};
use lazy_static::lazy_static;

lazy_static! {
    static ref ARGON_SECRET_KEY: String = std::env::var("ARGON_SECRET_KEY").unwrap_or_else(|_| "0123".repeat(8));
    static ref SALT: String = std::env::var("SALT").unwrap_or_else(|_| "Str0ng!Passw0rd".to_string());
}

pub fn hash_password(password: &str) -> Result<String, argon2::Error> {
    let config = Config {
        secret: ARGON_SECRET_KEY.as_bytes(),
        ..Default::default()
    };
    let hash = argon2::hash_encoded(password.as_bytes(), SALT.as_bytes(), &config).map_err(|err| {
        println!("Error hashing password: {}", err);
        err
    })?;
    Ok(hash)
}