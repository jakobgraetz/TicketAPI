// @author Jakob Grätz, Johannes Schießl
// @date 05/07/2024 (DD/MM/YYYY)
// @version v0.0.2

#[macro_use] extern crate rocket;
use rocket::http::Status;
use rocket::request::{self, Request, FromRequest};
use rocket::request::Outcome;
use rocket::serde::{json::Json, Serialize, Deserialize};

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

#[derive(Debug, Deserialize)]
struct Ticket {
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for ApiKey {
    type Error = ApiKeyError;

    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let keys: Vec<_> = req.headers().get("x-api-key").collect();

        match keys.len() {
            0 => Outcome::Error((Status::BadRequest, ApiKeyError::Missing)),
            1 if is_api_key_valid(keys[0]) => Outcome::Success(ApiKey(keys[0].to_string())),
            1 => Outcome::Error((Status::BadRequest, ApiKeyError::Invalid)),
            _ => Outcome::Error((Status::BadRequest, ApiKeyError::BadCount)),
        }
    }
}

fn is_api_key_valid(key: &str) -> bool {
    update_usage();
    key == "valid_api_key"
}

fn update_usage() {
    println!("Updating Usage for user.")
}


// Routing for ticket API
#[post("/ticket", format = "application/json", data = "<ticket>")]
async fn api_create_ticket(_key: ApiKey, ticket: Json<Ticket>) -> String {
    format!("CREATE TICKET")
}

#[put("/ticket/<ticket_id>", format = "application/json", data = "<ticket>")]
fn api_update_ticket(ticket_id: &str, _key: ApiKey, ticket: Json<Ticket>) -> String {
    format!("UPDATE TICKET {ticket_id}")
}

#[get("/ticket/<ticket_id>")]
fn api_get_ticket(ticket_id: &str, _key: ApiKey) -> String {
    format!("GET TICKET {ticket_id}")
}

#[delete("/ticket/<ticket_id>")]
fn api_delete_ticket(ticket_id: &str, _key: ApiKey) -> String {
    format!("DELETE TICKET {ticket_id}")
}

#[get("/ticket/check/<ticket_id>")]
fn api_check_ticket(ticket_id: &str, _key: ApiKey) -> String {
    format!("CHECK TICKET {ticket_id}")
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
        .mount("/v1/", routes![api_create_ticket, api_get_ticket, api_delete_ticket, api_update_ticket, api_check_ticket])
        .launch()
        .await;
}