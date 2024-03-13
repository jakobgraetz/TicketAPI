/*
* @author Jakob Grätz, Johannes Schießl
* @edition 13/03/2024 DD/MM/YYYY
* @version v0.0.1
* @description Rust file for api utilities.
*/

use rand::Rng;
use std::collections::HashSet;

const KEY_LENGTH: usize = 64;

/*
* @author Jakob Grätz
* @description Checks if a given API key is a valid API key.
*/
pub fn check_api_key(api_key: String) -> bool {
    if api_key == "abc123" {
        return true;
    } else {
        return false;
    }
}

/*
* @author Jakob Grätz
* @description Generates a new API key (String).
*/
// to avoid giving out the same API key more than once, API keys will 
// need to be stored and the API key generator needs to check if a key 
// already exists before returning it.
pub fn generate_api_key() -> String {
    let mut rng = rand::thread_rng();

    let charset: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789.!+-#![]|{}?*'^<>()/&%$";
    let mut key = String::with_capacity(KEY_LENGTH);

    let mut unique_chars = HashSet::with_capacity(KEY_LENGTH);
    while key.len() < KEY_LENGTH {
        // random index within charset range
        let random_index = rng.gen_range(0..charset.len());
        // random character from charset
        let random_char = charset[random_index] as char;
        if unique_chars.insert(random_char) {
            key.push(random_char);
        }
    }
    return key;
}