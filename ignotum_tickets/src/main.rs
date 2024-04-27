/*
* @author Jakob Grätz, Johannes Schießl | @jakobgraetz, @johannesschiessl
* @edition 27/04/2024 DD/MM/YYYY
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

// Creates a new ticket for the user.
// Future: Updates the user ticket / payment count.
// Returns json for a ticket id, and a qr code for the ticket and / or status code.
#[get("/create-ticket")]
fn api_create_ticket() -> &'static str {
    "CREATE TICKET"
}

// Returns json for a ticket with a given id and / or status code.
#[get("/get-ticket")]
fn api_get_ticket() -> &'static str {
    "GET TICKET"
}

// Deletes ticket with given id.
// Returns status code.
#[get("/delete-ticket")]
fn api_delete_ticket() -> &'static str {
    "DELETE TICKET"
}

// Updates ticket with given id.
// Returns status code.
#[get("/update-ticket")]
fn api_update_ticket() -> &'static str {
    "UPDATE TICKET"
}

// Checks if a ticket with id / with qr code is valid.
// Returns bool and / or status code.
#[get("/check-ticket")]
fn api_check_ticket() -> &'static str {
    "CHECK TICKET"
}

#[tokio::main]
async fn main() {
    rocket::build()
        .configure(rocket::Config::figment().merge(("port", 1234)))
        .mount("/", routes![dashboard]) // Mount your routes
        .mount("/api/v1/", routes![api_create_ticket, api_get_ticket, api_delete_ticket, api_update_ticket, api_check_ticket])
        .attach(Template::fairing()) // Attach fairing for templates
        .launch() // Start the Rocket server
        .await; // Await the server to start
}
