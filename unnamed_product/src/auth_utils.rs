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
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString, Salt
    },
    Argon2
};

// TODO: stop it from adding its own options to the hash
pub fn hash_string (data: String) -> Result<(String, SaltString), argon2::password_hash::Error> {
    let salt = SaltString::generate(&mut OsRng);

    let argon2 = Argon2::default();

    let data_hash = argon2.hash_password(data.as_bytes(), &salt)?;
    let hash = data_hash.hash;
    println!("{:?}", hash);
    println!("{:?}", salt);
    Ok((hash.expect("error with hashing").to_string(), salt))
}

pub fn check_hash_string () -> Result<String, argon2::password_hash::Error> {
    let my_hash: String = "8ysWb2FRB+pnE77Iro1oUoaBAqJS6I+/yr1FesA2kqY".to_string();
    let my_salt = SaltString::new("qGgbFKPpXQMYOq7qF5CFDA");

    let password: String = "A quick brown fox jumps over the lazy frog. 123456789!?".to_string();

    let argon2 = Argon2::default();

    let salt = Salt::from(&my_salt);
    let my_hash = argon2.hash_password(password.as_bytes(), salt)?;
    let hash = my_hash.hash;

    println!("{:?}", hash);

    Ok(hash.expect("error").to_string())
}