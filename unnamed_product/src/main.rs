/*
* @author Jakob Grätz, Johannes Schießl | @jakobgraetz, @johannesschiessl
* @edition 01/04/2024 DD/MM/YYYY
* @version v0.0.1
* @description Main Rust file for backend.
*/

/* --------------------- IMPORTS --------------------- */

#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
extern crate serde;
extern crate serde_json;

use rocket::{serde::{json::Json, Deserialize, Serialize}, http::Status};
use rocket::response::status;

mod db_handler;
mod api_utils;

/* --------------------- STRUCTS --------------------- */

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct Ticket {
    id: usize,
    title: String,
    description: String,
}

/* --------------------- ROUTES --------------------- */

#[post("/tickets", format = "json", data = "<ticket>")]
fn create_ticket(ticket: Json<Ticket>) -> Result<Json<Ticket>, status::Custom<Json<&'static str>>> {
    // Simulating a database insert success
    if ticket.title != "Error" {
        Ok(ticket)
    } else {
        // Simulating a database insert failure
        Err(status::Custom(Status::BadRequest, Json("Error inserting ticket into database")))
    }
}

#[get("/tickets/<id>")]
fn get_ticket(id: usize) -> Result<Json<Ticket>, status::Custom<Json<&'static str>>> {
    // Simulating fetching a ticket successfully
    if id != 0 {
        Ok(Json(Ticket { id, title: "Example Ticket".into(), description: "This is a ticket description.".into() }))
    } else {
        // Simulating a "not found" situation
        Err(status::Custom(Status::NotFound, Json("Ticket not found")))
    }
}

#[put("/tickets/<id>", format = "json", data = "<ticket>")]
fn update_ticket(id: usize, ticket: Json<Ticket>) -> Result<Json<Ticket>, status::Custom<Json<&'static str>>> {
    // Simulating an update operation success
    if ticket.title != "Error" {
        Ok(ticket)
    } else {
        // Simulating an update operation failure
        Err(status::Custom(Status::BadRequest, Json("Error updating ticket")))
    }
}

#[delete("/tickets/<id>")]
fn delete_ticket(id: usize) -> Result<status::NoContent, status::Custom<Json<&'static str>>> {
    // Simulating a successful delete operation
    if id != 0 {
        Ok(status::NoContent)
    } else {
        // Simulating a delete operation failure (e.g., ticket not found)
        Err(status::Custom(Status::NotFound, Json("Ticket not found")))
    }
}

/* --------------------- MAIN --------------------- */

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![create_ticket, get_ticket, update_ticket, delete_ticket])
}

fn main() {
    println!("[DEV] Testing generate_api_key: {:?}", api_utils::generate_api_key());
    println!("[DEV] Testing test_db: {:?}", db_handler::test_db());
    api_utils::check_api_request("abc123".to_string(), "John Doe".to_string(), "2024-04-12");
}
