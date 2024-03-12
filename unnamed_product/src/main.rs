/*
* @author Jakob Grätz
* @edition 12/03/2024 DD/MM/YYYY
* @version v0.0.1
* @description Main Rust file for backend.
*/

mod db_handler;
mod api_utils;

fn main() {
    print!("Testing with 'correct' API key: {:?} \n", api_utils::check_api_key("abc123".to_string()));
    print!("Testing with 'incorrect' API key: {:?} \n", api_utils::check_api_key("def456".to_string()));
    db_handler::handle_db_mod_test();
    println!("Hello, world!");
}
