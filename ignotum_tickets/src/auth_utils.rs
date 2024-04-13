/*
* @author Jakob Grätz, Johannes Schießl | @jakobgraetz, @johannesschiessl
* @edition 13/04/2024 DD/MM/YYYY
* @version v0.0.1
* @description Rust file for authentication utilities.
*/

extern crate argon2;
use argon2::{
    password_hash::{
        rand_core::OsRng, 
        // Error,
        /*PasswordHash,*/ PasswordHasher, /*PasswordVerifier,*/ SaltString, /*Salt*/
        
    },
    Argon2
};
use rand::Rng;
use std::collections::HashSet;

pub fn hash_string (data: String) -> Result<(String, String), argon2::password_hash::Error> {
    let salt = SaltString::generate(&mut OsRng);

    let argon2 = Argon2::default();

    let data_hash = argon2.hash_password(data.as_bytes(), &salt)?;
    let hash = match data_hash.hash {
        Some(h) => h.to_string(),
        None => "error".to_string(), //TODO: error handling wizardry
    };

    println!("{:?}", hash);
    println!("{:?}", salt);

    Ok((hash, salt.to_string()))
}

// @author Jakob Grätz
// Hashes plain text string and compares it to a given hash
pub fn check_string (salt: String, plain: String, hash: String) -> Result<bool, argon2::password_hash::Error> {
    let new_salt = match SaltString::from_b64(&salt) {
        Ok(salt) => salt,
        Err(err) => panic!("Unexpected behavior when generating SaltString"),
    };
    
    let argon2 = Argon2::default();

    let new_hash = argon2.hash_password(plain.as_bytes(), &new_salt)?;
    let hash_content = match new_hash.hash {
        Some(h) => h.to_string(),
        None => "error".to_string(),
    };

    if hash == hash_content {
        println!("{:?} = {:?}", hash_content, hash);
        Ok(true)
    } else {
        println!("{:?} != {:?}", hash_content, hash);
        Ok(false)
    }
}

pub fn generate_api_key() -> String {
    const KEY_LENGTH: usize = 64;
    
    let mut rng = rand::thread_rng();

    let charset: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789.!+-#![]|{}?*'^<>()/&%$";
    let mut key = String::with_capacity(KEY_LENGTH);
    key.push_str("sk-");

    let mut unique_chars = HashSet::with_capacity(KEY_LENGTH);
    while key.len() < KEY_LENGTH + 3 {
        // random index within charset range
        // might sometime update this to not only include unique characters in each strings, as that limits
        // the number of API keys.
        let random_index = rng.gen_range(0..charset.len());
        // random character from charset
        let random_char = charset[random_index] as char;
        if unique_chars.insert(random_char) {
            key.push(random_char);
        }
    }
    return key;
}