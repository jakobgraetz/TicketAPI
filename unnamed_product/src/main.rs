/*
* @author Jakob Grätz
* @edition 12/03/2024 DD/MM/YYYY
* @version v0.0.1
* @description Main Rust file for backend.
*/

mod db_handler;
mod api_utils;

fn main() {
    api_utils::handle_api_utils_mod_test();
    db_handler::handle_db_mod_test();
    println!("Hello, world!");
}
