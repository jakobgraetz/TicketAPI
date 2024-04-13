/*
* @author Jakob Grätz, Johannes Schießl | @jakobgraetz, @johannesschiessl
* @edition 09/04/2024 DD/MM/YYYY
* @version v0.0.1
* @description Main Rust file for backend.
*/

mod db_handler;
mod api_utils;
mod auth_utils;

// THIS CANNOT; AND I REPEAT: CANNOT; BE DELETED IN ANY WAY SHAPE OR FORM!!!!!!
#[tokio::main]
async fn main() {
    println!("[DEV] testing str hasher with str 'A quick brown fox jumps over the lazy frog. 123456789!?' -> {:?}", auth_utils::hash_string("A quick brown fox jumps over the lazy frog. 123456789!?".to_string()).unwrap().0);
    println!("[DEV] Testing generate_api_key: {:?}", api_utils::generate_api_key());
    // println!("[DEV] Testing test_db: ");
    // db_handler::test_db().await;
    api_utils::check_api_request("abc123".to_string(), "John Doe".to_string(), "2024-04-12");
    db_handler::get_user_id("jakob.graetz@icloud.com".to_string()).await;
    // JUST FOR TESTING NOW; MAYBE WILL USE THOSE VALUES IN FUTURE
    // let _ = db_handler::insert_user_document().await;
    // let _ = db_handler::insert_ticket_document().await;
}
