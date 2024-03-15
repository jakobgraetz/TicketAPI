/*
* @author Jakob Grätz, Johannes Schießl
* @edition 13/03/2024 DD/MM/YYYY
* @version v0.0.1
* @description Rust file for api utilities.
*/

// DEPENDENCIES

extern crate chrono;

use chrono::{NaiveDate, Local};

use rand::Rng;
use std::collections::HashSet;

// CONSTANTS

const KEY_LENGTH: usize = 64;

// HELPER FUNCTIONS

/*
* @author Johannes Schießl
* @description Checks if the input date string represents a date in the future.
*/
pub fn is_date_in_future(date_str: &str) -> bool {
    match NaiveDate::parse_from_str(date_str, "%Y-%m-%d") {
        Ok(input_date) => {
            let current_date = Local::now().naive_local().into();
            input_date > current_date
        },
        Err(_) => false,
    }
}


// API UTILS

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


/*
* @author Johannes Schießl
* @description Checks the validity of an API request.
*/
pub fn check_api_request(id: String, name: String, date: &str) -> bool {
    if id != "abc123" {
        println!("Error: The provided ID '{}' is invalid. Please ensure the ID is correct.", id);
        return false
    }

    if !is_date_in_future(date) {
        println!("Error: The provided date '{}' is not in the future or has an invalid format. Please ensure the date is in the 'YYYY-MM-DD' format and is a future date.", date);
        return false
    }

    if name != "John Doe" {
        println!("Error: The provided name '{}' is invalid. Please ensure the name is valid.", name);
        return false
    }

    println!("Success: The provided API request is valid.");
    return true
}

// TESTS

#[cfg(test)]
mod tests {
    use super::*;

    // HELPER FUNCTIONS

    // is_date_in_future
    // You may need to mock `is_date_in_future` function to return true for a future date in format
    #[test]
    fn test_is_date_in_future_with_future_date() {
        // Using a date that is guaranteed to be in the future relative to the test writing date
        let future_date = "2100-01-01";
        assert_eq!(is_date_in_future(future_date), true);
    }

    #[test]
    fn test_is_date_in_future_with_past_date() {
        // Using a date that is guaranteed to be in the past
        let past_date = "2000-01-01";
        assert_eq!(is_date_in_future(past_date), false);
    }

    #[test]
    fn test_is_date_in_future_with_invalid_date() {
        // Using an invalid date format
        let invalid_date = "not-a-date";
        assert_eq!(is_date_in_future(invalid_date), false);
    }

    #[test]
    fn test_is_date_in_future_with_today_date() {
        // This test may not be reliably predictable unless you mock the current date
        let today_date = Local::now().format("%Y-%m-%d").to_string();
        assert_eq!(is_date_in_future(&today_date), false);
    }


    // API UTILS

    // check_api_request
    #[test]
    fn test_check_api_request_success() {
        let id = "abc123".to_string();
        let name = "John Doe".to_string();
        let date = "2099-01-01"; // Ensure this is a date in the future
        assert_eq!(check_api_request(id, name, date), true);
    }

    #[test]
    fn test_check_api_request_invalid_id() {
        let id = "wrong_id".to_string();
        let name = "John Doe".to_string();
        let date = "2099-01-01"; // Future date
        assert_eq!(check_api_request(id, name, date), false);
    }

    #[test]
    fn test_check_api_request_invalid_name() {
        let id = "abc123".to_string();
        let name = "Jane Doe".to_string();
        let date = "2099-01-01"; // Future date
        assert_eq!(check_api_request(id, name, date), false);
    }

    #[test]
    fn test_check_api_request_invalid_date_format() {
        let id = "abc123".to_string();
        let name = "John Doe".to_string();
        let date = "01-01-2099"; // Incorrect format
        assert_eq!(check_api_request(id, name, date), false);
    }

    #[test]
    fn test_check_api_request_date_not_in_future() {
        let id = "abc123".to_string();
        let name = "John Doe".to_string();
        let date = "2000-01-01"; // Past date
        assert_eq!(check_api_request(id, name, date), false);
    }

    #[test]
    fn test_api_key_len() {
        assert!(KEY_LENGTH==generate_api_key().len())
    }


}
