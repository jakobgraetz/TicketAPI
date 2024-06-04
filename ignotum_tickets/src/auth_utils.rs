/*
* @author Jakob Grätz, Johannes Schießl | @jakobgraetz, @johannesschiessl
* @edition 23/04/2024 DD/MM/YYYY
* @version v0.0.1
* @description Rust file for authentication utilities, like hashing, key gen, ...
*/

use rand::Rng;
use std::collections::HashSet;

pub fn generate_api_key(user_id: String) -> String {
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
    
    key.push_str(&user_id);

    key
}