/*
* @author Jakob Grätz, Johannes Schießl
* @edition 12/03/2024 DD/MM/YYYY
* @version v0.0.1
* @description Main Rust file for backend.
*/

mod db_handler;
mod api_utils;

fn main() {
    print!("[DEV] Testing with 'correct' API key: {:?} \n", api_utils::check_api_key("abc123".to_string()));
    print!("[DEV] Testing with 'incorrect' API key: {:?} \n", api_utils::check_api_key("def456".to_string()));
    print!("[DEV] Testing generate_api_key: {:?} \n", api_utils::generate_api_key());
    api_utils::check_api_request("abc123".to_string(), "John Doe".to_string(), "2024-04-12");
}
