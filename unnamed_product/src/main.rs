/*
* @author Jakob Grätz, Johannes Schießl | @jakobgraetz, @johannesschiessl
* @edition 12/03/2024 DD/MM/YYYY
* @version v0.0.1
* @description Main Rust file for backend.
*/

mod db_handler;
mod api_utils;

fn main() {
    print!("[DEV] Testing generate_api_key: {:?} \n", api_utils::generate_api_key());
    print!("[DEV] Testing test_db: {:?} \n", db_handler::test_db());
    api_utils::check_api_request("abc123".to_string(), "John Doe".to_string(), "2024-04-12");
}
