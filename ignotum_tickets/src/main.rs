/*
* @author Jakob Grätz, Johannes Schießl | @jakobgraetz, @johannesschiessl
* @edition 11/05/2024 DD/MM/YYYY
* @version v0.0.1
* @description Main Rust file for backend.
*/

// Imports.
#[macro_use] extern crate rocket;
use bson::oid::ObjectId;
use rocket::http::Status;
use rocket::request::{self, Request, FromRequest};
use rocket::request::Outcome;
use rocket::serde::{json::Json, Deserialize};
use chrono::prelude::*;

// Import local modules.
mod db_handler;
mod api_utils;
mod auth_utils;

// Defines ApiKey struct. Holds the Api Key as a String.
struct ApiKey(String);

// Defines an enum for Api Key errors.
#[derive(Debug)]
enum ApiKeyError {
    BadCount, // Error that indicates an unexpected number of API keys.
    Missing,  // Error that indicates the API key is missing.
    Invalid   // Error that indicates the API key is incorrect.
}

#[derive(Debug, Deserialize)]
struct Ticket {
    #[serde(rename = "title")]
    title: String,
    #[serde(rename = "close_date")]
    close_date: String,
    #[serde(rename = "customer_first_name")]
    customer_first_name: String,
    #[serde(rename = "customer_last_name")]
    customer_last_name: String,
    #[serde(rename = "customer_email")]
    customer_email: String,
}

// Implement the conversion trait `FromRequest` for `ApiKey`.
#[rocket::async_trait]
impl<'r> FromRequest<'r> for ApiKey {
    // Specify the associated error type for the conversion trait.
    type Error = ApiKeyError;

    // Implement the conversion function for obtaining an `ApiKey` from a request.
    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        // Retrieve the API keys from the request headers.
        let keys: Vec<_> = req.headers().get("x-api-key").collect();

        // For non-Rustaceans, a match statement is basically the same as a Switch - Case statement.
        // Matches the number of keys in the request header to the corresponding actions.
        match keys.len() {
            0 => Outcome::Error((Status::BadRequest, ApiKeyError::Missing)),
            1 if is_api_key_valid(keys[0]) => Outcome::Success(ApiKey(keys[0].to_string())),
            1 => Outcome::Error((Status::BadRequest, ApiKeyError::Invalid)),
            _ => Outcome::Error((Status::BadRequest, ApiKeyError::BadCount)),
        }
    }
}

fn is_api_key_valid(key: &str) -> bool {
    key == "valid_api_key"
}

// route: /api/v1/
// create-ticket
// get-ticket
// check-ticket
// delete-ticket
// update-ticket

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
    customer_first_name: String,
    customer_last_name: String,
    customer_email: String
}
*/

// Creates a new ticket for the user.
// Future: Updates the user ticket / payment count.
// Returns json for a ticket id, and a qr code for the ticket and / or status code.
// needs API key

// Create a ticket (POST request)
#[post("/ticket", format = "application/json", data = "<ticket>")]
fn api_create_ticket(_key: ApiKey, ticket: Json<Ticket>) -> String {
    let id = ObjectId::new().to_string();

    println!(
        "Received ticket: {:?}, {:?}, {:?}, {:?}, {:?}, {:?}",
        id, ticket.title, ticket.close_date, ticket.customer_first_name,
        ticket.customer_last_name, ticket.customer_email
    );
    /*
    pub struct Ticket {
        user_id: ObjectId,
        title: String,
        close_date: String,
        customer_first_name: String,
        customer_last_name: String,
        customer_email: String
    }
    */
    format!("ticket created: {id}")
}

// Update a ticket by its ID (PUT request)
// Returns status code.
// needs API key
#[put("/ticket/<ticket_id>", format = "application/json", data = "<ticket>")]
fn api_update_ticket(ticket_id: &str, _key: ApiKey, ticket: Json<Ticket>) -> String {
    format!("UPDATE TICKET {ticket_id}")
}

// Retrieve a ticket by its ID (GET request)
// Returns json for a ticket with a given id and / or status code.
#[get("/ticket/<ticket_id>")]
fn api_get_ticket(ticket_id: &str, _key: ApiKey) -> String {
    format!("GET TICKET {ticket_id}")
}

// Delete a ticket by its ID (DELETE request)
// Returns status code.
// needs API key
#[delete("/ticket/<ticket_id>")]
fn api_delete_ticket(ticket_id: &str, _key: ApiKey) -> String {
    format!("DELETE TICKET {ticket_id}")
}




// Check if a ticket is valid by its ID (GET request)
// Returns bool and / or status code.
// doesn't necessarily need API key, though might be better, idk
// as of may 2nd 2024, 20:21 CEST: will be key protected to avoid "DDOS"
#[get("/ticket/check/<ticket_id>")]
fn api_check_ticket(ticket_id: &str, _key: ApiKey) -> String {
    format!("CHECK TICKET {ticket_id}")
}

#[tokio::main]
async fn main() {
    let _ = rocket::build()
        .configure(rocket::Config::figment().merge(("port", 10000)))
        .mount("/v1/", routes![api_create_ticket, api_get_ticket, api_delete_ticket, api_update_ticket, api_check_ticket])
        .launch() // Start the Rocket server
        .await; // Await the server to start
}