/*
* @author Jakob Grätz, Johannes Schießl | @jakobgraetz, @johannesschiessl
* @edition 30/03/2024 DD/MM/YYYY
* @version v0.0.1
* @description Main Rust file for backend.
*/

mod db_handler;
mod api_utils;

#[tokio::main]
async fn main() {
    println!("[DEV] Testing generate_api_key: {:?}", api_utils::generate_api_key());
    println!("[DEV] Testing test_db: \n");
    db_handler::test_db().await;
    api_utils::check_api_request("abc123".to_string(), "John Doe".to_string(), "2024-04-12");
    db_handler::insert_user_document().await;
    db_handler::insert_ticket_document().await;
}
