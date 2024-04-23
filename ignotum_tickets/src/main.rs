/*
* @author Jakob Grätz, Johannes Schießl | @jakobgraetz, @johannesschiessl
* @edition 14/04/2024 DD/MM/YYYY
* @version v0.0.1
* @description Main Rust file for backend.
*/

#[macro_use] extern crate rocket;
use rocket_dyn_templates::{Template, context};
mod db_handler;
mod api_utils;
mod auth_utils;

#[get("/dashboard")]
fn dashboard() -> Template {
    Template::render("dashboard", context!{ field: "value" })
}

#[tokio::main]
async fn main() {
    /*
    let _ = db_handler::delete_user("jane.doe@example.com".to_string()).await;
    println!("{:?}", db_handler::get_user_data("john.doe@example.com".to_string()).await);
    let hash_tuple = auth_utils::hash_string("A quick brown fox jumps over the lazy frog. 123456789!?".to_string()).unwrap();
    println!("[DEV] testing str hasher with str 'A quick brown fox jumps over the lazy frog. 123456789!?' -> {:?} with salt: {:?}",
        hash_tuple.0,
        hash_tuple.1,
    );
    println!("Checking with correct plain, hash and salt:");
    let _ = auth_utils::check_string("faozt5o1Fbep6T2v+wmwyg".to_string(), 
        "A quick brown fox jumps over the lazy frog. 123456789!?".to_string(),
        "bsNljGi1cQDH/G6V+OJqbA/XUJLAFlDXMrnnusGnuqQ".to_string()
    );
    println!("[DEV] Testing generate_api_key: {:?}", auth_utils::generate_api_key());
    // println!("[DEV] Testing test_db: ");
    // db_handler::test_db().await;
    // JUST FOR TESTING NOW; MAYBE WILL USE THOSE VALUES IN FUTURE
    // let _ = db_handler::insert_user_document().await;
    // let _ = db_handler::insert_ticket_document().await;
    */
    rocket::build()
        .configure(rocket::Config::figment().merge(("port", 1234)))
        .mount("/", routes![dashboard]) // Mount your routes
        .attach(Template::fairing()) // Attach fairing for templates
        .launch() // Start the Rocket server
        .await; // Await the server to start
}
