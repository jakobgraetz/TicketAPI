/*
* @author Jakob Grätz, Johannes Schießl | @jakobgraetz, @johannesschiessl
* @edition 26/04/2024 DD/MM/YYYY
* @version v0.0.1
* @description Main Rust file for backend.
*/

#[macro_use] extern crate rocket;
use rocket_dyn_templates::{Template, context};
use rocket::Outcome;
use rocket::http::Status;
use rocket::request::{self, Request, FromRequest};

mod db_handler;
mod api_utils;
mod auth_utils;

#[get("/dashboard")]
fn dashboard() -> Template {
    Template::render("dashboard", context!{ field: "value" })
}

/*
#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct User {
    _id: ObjectId,
    first_name: String,
    last_name: String,
    email: String,
    api_key_hash: String,
    user_password_hash: String,
    salt:  String,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Ticket {
    _id: ObjectId,
    user_id: ObjectId,
    title: String,
    status: String,
    creation_date: String,
    update_date: String,
    close_date: String,
}
*/

// route: /api/v1/
// create-ticket
// get-ticket
// check-ticket
// delete-ticket
// update-ticket

#[get("/create-ticket")]
fn api_create_ticket() -> &'static str {
    "CREATE TICKET"
}

#[get("/get-ticket")]
fn api_get_ticket() -> &'static str {
    "GET TICKET"
}

#[get("/delete-ticket")]
fn api_delete_ticket() -> &'static str {
    "DELETE TICKET"
}

#[get("/update-ticket")]
fn api_update_ticket() -> &'static str {
    "UPDATE TICKET"
}

#[get("/check-ticket")]
fn api_check_ticket() -> &'static str {
    "CHECK TICKET"
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
        .mount("/api/v1/", routes![api_create_ticket, api_get_ticket, api_delete_ticket, api_update_ticket, api_check_ticket])
        .attach(Template::fairing()) // Attach fairing for templates
        .launch() // Start the Rocket server
        .await; // Await the server to start
}
