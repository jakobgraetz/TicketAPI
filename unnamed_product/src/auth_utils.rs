/*
* @author Jakob Grätz, Johannes Schießl | @jakobgraetz, @johannesschiessl
* @edition 10/04/2024 DD/MM/YYYY
* @version v0.0.1
* @description Rust file for authentication utilities.
*/

extern crate argon2;
use argon2::{
    password_hash::{
        rand_core::OsRng,
        /*PasswordHash,*/ PasswordHasher, /*PasswordVerifier,*/ SaltString, /*Salt*/
        // Error
    },
    Argon2
};

// TODO: stop it from adding its own options to the hash
pub fn hash_string (data: String) -> Result<(String, SaltString), argon2::password_hash::Error> {
    let salt = SaltString::generate(&mut OsRng);

    let argon2 = Argon2::default();

    let data_hash = argon2.hash_password(data.as_bytes(), &salt)?;
    let hash = match data_hash.hash {
        Some(h) => h.to_string(),
        None => "exception".to_string(), //TODO: error handling wizardry
    };
    println!("{:?}", hash);
    println!("{:?}", salt);
    Ok((hash, salt))
}