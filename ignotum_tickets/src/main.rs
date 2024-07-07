// @author Jakob Grätz, Johannes Schießl
// @date 05/07/2024 (DD/MM/YYYY)
// @version v0.0.2

#[macro_use] extern crate rocket;
use rocket::http::Status;
use rocket::request::{self, Request, FromRequest};
use rocket::request::Outcome;
use rocket::serde::{json::Json, Serialize, Deserialize};
use tokio_postgres::{NoTls, Error};
use std::env;
use dotenv::dotenv;

struct ApiKey(String);

#[derive(Serialize, Deserialize)]
struct ErrorResponse {
    status: u16,
    error: &'static str,
    message: &'static str,
    suggestion: &'static str,
}

#[derive(Debug)]
enum ApiKeyError {
    BadCount,
    Missing,
    Invalid
}

#[derive(Debug, Deserialize, Serialize)]
struct Ticket {
    id: Option<i64>,
    event_name: Option<String>,
    event_location: Option<String>,
    event_date: Option<String>,
    status: Option<String>,
    holder_name: Option<String>,
    holder_email: Option<String>,
    notes: Option<String>,
    terms_and_conditions: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct MonthlyUsage {
    date: String,
    usage: i64,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for ApiKey {
    type Error = ApiKeyError;

    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let keys: Vec<_> = req.headers().get("x-api-key").collect();

        match keys.len() {
            0 => Outcome::Error((Status::BadRequest, ApiKeyError::Missing)),
            1 if is_api_key_valid(keys[0]).await.is_ok() => Outcome::Success(ApiKey(keys[0].to_string())),
            1 => Outcome::Error((Status::BadRequest, ApiKeyError::Invalid)),
            _ => Outcome::Error((Status::BadRequest, ApiKeyError::BadCount)),
        }
    }
}

async fn is_api_key_valid(key: &str) -> Result<i64, Error> {
    key.to_string();

    dotenv().ok();
    let database_url = env::var("SUPABASE_URI").expect("SUPABASE_URI must be set");
    let (client, connection) = tokio_postgres::connect(&database_url, NoTls).await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    let query = format!("SELECT id FROM keys WHERE api_key = $1");
    let row = client.query_one(&query, &[&key]).await?;

    let key_id: i64 = row.get(0);

    Ok(key_id)
}

async fn update_usage(key_id: i64) -> Result<(), Error> {
    dotenv().ok();
    let database_url = env::var("SUPABASE_URI").expect("SUPABASE_URI must be set");
    let (client, connection) = tokio_postgres::connect(&database_url, NoTls).await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    let total_query = format!("UPDATE keys SET total_uses = total_uses + 1 WHERE id = $1");
    let _ = client.query_one(&total_query, &[&key_id]).await?;
    
    Ok(())
}

async fn insert_ticket(ticket: Json<Ticket>, key_id: i64) -> Result<i64, Error> {
    let event_name = ticket.event_name.clone().unwrap_or_else(|| "".to_string());
    let event_location = ticket.event_location.clone().unwrap_or_else(|| "".to_string());
    let event_date = ticket.event_date.clone().unwrap_or_else(|| "".to_string());
    let status = ticket.status.clone().unwrap_or_else(|| "".to_string());
    let holder_name = ticket.holder_name.clone().unwrap_or_else(|| "".to_string());
    let holder_email = ticket.holder_email.clone().unwrap_or_else(|| "".to_string());
    let notes = ticket.notes.clone().unwrap_or_else(|| "".to_string());
    let terms_and_conditions = ticket.terms_and_conditions.clone().unwrap_or_else(|| "".to_string());

    dotenv().ok();
    let database_url = env::var("SUPABASE_URI").expect("SUPABASE_URI must be set");
    let (client, connection) = tokio_postgres::connect(&database_url, NoTls).await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    let query = format!("INSERT INTO tickets (event_name, event_location, event_date, status, holder_name, holder_email, notes, terms_and_conditions, key_id) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9) RETURNING id");
    let row = client.query_one(&query, &[&event_name, &event_location, &event_date, &status, &holder_name, &holder_email, &notes, &terms_and_conditions, &key_id]).await?;

    let generated_id: i64 = row.get(0);

    Ok(generated_id)
}

// Routing for ticket API
#[post("/ticket", format = "application/json", data = "<ticket>")]
async fn api_create_ticket(key: ApiKey, ticket: Json<Ticket>) -> String {
    let key_id: i64 = is_api_key_valid(&key.0).await.unwrap();
    let id: i64 = insert_ticket(ticket, key_id.clone()).await.unwrap();
    let _ = update_usage(key_id).await;

    format!("Ticket created successfully: {}", id)
        /*
    "id": i64 SEQUENTIAL NOT NULL,
    "event_name": varchar,
    "event_location": varchar,
    "event_date": timestamptz,
    "key_id": i64 FOREIGN KEY -> keys - NOT NULL,
    "status": varchar,
    "status": "Active",
    "holder_name": varchar,
    "holder_email": varchar,
    "notes": varchar,
    "terms_and_conditions": varchar,
    "created_at": timestamp NOT NULL,
    "updated_at": timestamp
    -----
    id: i64,
    event_name: Option<String>,
    event_location: Option<String>,
    event_date: Option<String>,
    key_id: i64,
    status: Option<String>,
    holder_name: Option<String>,
    holder_email: Option<String>,
    notes: Option<String>,
    terms_and_conditions: Option<String>,
    */
}

#[put("/ticket/<ticket_id>", format = "application/json", data = "<ticket>")]
async fn api_update_ticket(ticket_id: i64, key: ApiKey, ticket: Json<Ticket>) -> String {
    let key_id: i64 = is_api_key_valid(&key.0).await.unwrap();
    //let id: i64 = insert_ticket(ticket, key_id.clone()).await.unwrap();
    let _ = update_usage(key_id).await;

    format!("UPDATE TICKET {ticket_id}")
}

#[get("/ticket/<ticket_id>")]
async fn api_get_ticket(ticket_id: i64, key: ApiKey) -> String {
    let key_id: i64 = is_api_key_valid(&key.0).await.unwrap();
    //let id: i64 = insert_ticket(ticket, key_id.clone()).await.unwrap();
    let _ = update_usage(key_id).await;

    format!("GET TICKET {ticket_id}")
}

async fn delete_ticket(ticket_id: i64, key_id: i64) -> Result<(), Error> {
    dotenv().ok();
    let database_url = env::var("SUPABASE_URI").expect("SUPABASE_URI must be set");
    let (client, connection) = tokio_postgres::connect(&database_url, NoTls).await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    let query = format!("SELECT key_id FROM tickets WHERE id = $1");
    let row = client.query_one(&query, &[&ticket_id]).await?;

    let stored_key_id: i64 = row.get(0);

    if stored_key_id == key_id {
        let delete_query = format!("DELETE from tickets WHERE id = $1");
        let _ = client.query_one(&delete_query, &[&ticket_id]).await?;
        Ok(())
    } else {
        Ok(())   
    }

    
}

#[delete("/ticket/<ticket_id>")]
async fn api_delete_ticket(ticket_id: i64, key: ApiKey) -> String {
    let key_id: i64 = is_api_key_valid(&key.0).await.unwrap();
    let _ = update_usage(key_id).await;

    let _ = delete_ticket(ticket_id, key_id).await;
    format!("Successfully deleted ticket {:?}", ticket_id)
}

// HTTP Error Handlers and Catchers
#[catch(400)]
fn catch_err_400() -> Json<ErrorResponse> {
    Json(ErrorResponse {
        status: 400,
        error: "Bad Request",
        message: "The server could not understand the request due to invalid syntax.",
        suggestion: "Check the request syntax and try again.",
    })
}

#[catch(401)]
fn catch_err_401() -> Json<ErrorResponse> {
    Json(ErrorResponse {
        status: 401,
        error: "Unauthorized",
        message: "You must authenticate yourself to get the requested response.",
        suggestion: "Provide valid authentication credentials.",
    })
}

#[catch(403)]
fn catch_err_403() -> Json<ErrorResponse> {
    Json(ErrorResponse {
        status: 403,
        error: "Forbidden",
        message: "You do not have permission to access the requested resource.",
        suggestion: "Ensure you have the necessary permissions and try again.",
    })
}

#[catch(404)]
fn catch_err_404() -> Json<ErrorResponse> {
    Json(ErrorResponse {
        status: 404,
        error: "Not Found",
        message: "The requested resource could not be found on this server.",
        suggestion: "Check the URL for errors or try searching for the resource.",
    })
}

#[catch(405)]
fn catch_err_405() -> Json<ErrorResponse> {
    Json(ErrorResponse {
        status: 405,
        error: "Method Not Allowed",
        message: "The requested method is not allowed for the resource.",
        suggestion: "Check the allowed HTTP methods for the resource and try again.",
    })
}

#[catch(408)]
fn catch_err_408() -> Json<ErrorResponse> {
    Json(ErrorResponse {
        status: 408,
        error: "Request Timeout",
        message: "The server timed out waiting for the request.",
        suggestion: "Try sending the request again later.",
    })
}

#[catch(429)]
fn catch_err_429() -> Json<ErrorResponse> {
    Json(ErrorResponse {
        status: 429,
        error: "Too Many Requests",
        message: "You have sent too many requests in a given amount of time.",
        suggestion: "Wait for a while before making more requests.",
    })
}

#[catch(500)]
fn catch_err_500() -> Json<ErrorResponse> {
    Json(ErrorResponse {
        status: 500,
        error: "Internal Server Error",
        message: "The server encountered an internal error and could not complete your request.",
        suggestion: "Try again later or contact support if the issue persists.",
    })
}

#[catch(501)]
fn catch_err_501() -> Json<ErrorResponse> {
    Json(ErrorResponse {
        status: 501,
        error: "Not Implemented",
        message: "The server does not support the functionality required to fulfill the request.",
        suggestion: "Check the documentation or contact support.",
    })
}

#[catch(502)]
fn catch_err_502() -> Json<ErrorResponse> {
    Json(ErrorResponse {
        status: 502,
        error: "Bad Gateway",
        message: "The server received an invalid response from the upstream server.",
        suggestion: "Try again later or contact support if the issue persists.",
    })
}

#[catch(503)]
fn catch_err_503() -> Json<ErrorResponse> {
    Json(ErrorResponse {
        status: 503,
        error: "Service Unavailable",
        message: "The server is not ready to handle the request.",
        suggestion: "Try again later or contact support if the issue persists.",
    })
}

#[tokio::main]
async fn main() {
    let _ = rocket::build()
        .configure(rocket::Config::figment().merge(("port", 10000)))
        .register("/", catchers![catch_err_400, catch_err_401, catch_err_403, catch_err_404, catch_err_405, catch_err_408, catch_err_429, catch_err_500, catch_err_501, catch_err_502, catch_err_503])
        .mount("/v1/", routes![api_create_ticket, api_get_ticket, api_delete_ticket, api_update_ticket])
        .launch()
        .await;
}