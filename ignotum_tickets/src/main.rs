/*
* @author Jakob Grätz, Johannes Schießl | @jakobgraetz, @johannesschiessl
* @edition 11/05/2024 DD/MM/YYYY
* @version v0.0.1
* @description Main Rust file for backend.
*/

// Imports.
#[macro_use] extern crate rocket;
use rocket_dyn_templates::{Template, context};
use rocket::http::Status;
use rocket::request::{self, Request, FromRequest};
use rocket::request::Outcome;
use rocket::http::CookieJar;
use rocket::fs::FileServer;
use rocket::response::Redirect;
use rocket::http::Cookie;
use rocket::form::Form;
use chrono::prelude::*;
use argon2::password_hash::Error;

// Import local modules.
mod db_handler;
mod api_utils;
mod auth_utils;

// Defines ApiKey struct. Holds the Api Key as a String.
struct ApiKey(String);

// Struct for session management & auth.
struct SessionUser {
    _id: String,
}

// Defines an enum for Api Key errors.
#[derive(Debug)]
enum ApiKeyError {
    BadCount, // Error that indicates an unexpected number of API keys.
    Missing,  // Error that indicates the API key is missing.
    Invalid   // Error that indicates the API key is incorrect.
}

#[derive(FromForm)]
struct SignupForm {
    first_name: String,
    last_name: String,
    email: String,
    password: String,
    phone: String,
    company: String,
    address: String,
}

#[derive(FromForm)]
struct LoginForm {
    email: String,
    password: String,
}

// Implements FromRequest Trait to session user.
#[rocket::async_trait]
impl<'r> FromRequest<'r> for SessionUser {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, ()> {
        let cookies = request.cookies();

        if let Some(cookie) = cookies.get_private("user_id") {
            let user_id = cookie.value().to_string();

            if db_handler::check_user(user_id.clone()).await.unwrap() {
                return Outcome::Success(SessionUser {_id : user_id});
            }

            return Outcome::Forward(Status::BadRequest)
        }

        Outcome::Forward(Status::BadRequest)
    }
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

// Route "/". Renders the index.html.tera template in the templates folder.
// As of now, the cookie jar is not necessarily needed, but I implemented
// it anyway for future reference.
#[get("/")]
fn index_page(_jar: &CookieJar<'_>) -> Template {
    Template::render("index", context! { field: "value" })
}

// Route "/login". If there is not a SessionUser (Type FromRequest 
// implemented) in the request cookies, login_default() will handle, as
// it has the higher rank. Otherwise (if there is a SessionUser), the 
// user will be redirected to "/dashboard".
// Again, the cookie jar is not necessarily needed, but I implemented
// it anyway for future reference.
#[get("/login", rank = 1)]
fn login(_jar: &CookieJar<'_>, _session_user: SessionUser) -> Redirect {
    Redirect::to("/dashboard")
}

#[post("/login", data = "<login>")]
async fn login_handler(jar: &CookieJar<'_>, login: Form<LoginForm>) -> Redirect {
    async fn determine_login(jar: &CookieJar<'_>, login: Form<LoginForm>) -> Result<(), std::io::Error> {
        let email = &login.email;
        let password = &login.password;

        match db_handler::get_user_id(email).await.map_err(|err| std::io::Error::new(std::io::ErrorKind::Other, err))? {
            Some(user_id) => {
                println!("Exists.");
                let (password_hash, password_salt) = db_handler::get_user_auth_data(email).await.map_err(|err| std::io::Error::new(std::io::ErrorKind::Other, err.to_string()))?;
                println!("Getting user auth data.");
                match auth_utils::check_string(password_salt, password.to_string(), password_hash).map_err(|err| std::io::Error::new(std::io::ErrorKind::Other, err.to_string()))? {
                    true => {
                        println!("Valid Password.");
                        jar.add_private(Cookie::new("user_id", user_id.to_string()));
                    },
                    false => {
                        println!("Invalid Password.");
                    },
                };
            },
            None => {
                println!("Doesn't exist.");
            },
        };
        Ok(())
    }

    if let Err(err) = determine_login(jar, login).await {
        println!("Error in login_handler:{err}");
    }
    Redirect::to("/login")
}

// Route "/login" with rank 2. If there is no SessionUser, one will be
// added to the private cookies. Obviously this needs correct implementation
// of the databases and login functionality (forms).
#[get("/login", rank = 2)]
fn login_default(_jar: &CookieJar<'_>) -> Template {
    Template::render("login", context!{ field: "value" })
}

// Route "/signup". As with the login routes, an already authenticated user
// will be redirected to "/dashboard".
#[get("/signup", rank = 1)]
fn signup(_jar: &CookieJar<'_>, _session_user: SessionUser) -> Redirect {
    Redirect::to("/dashboard")
}

#[post("/signup", data = "<signup>")]
async fn signup_handler(jar: &CookieJar<'_>, signup: Form<SignupForm>) -> Redirect {
    async fn determine_signup(jar: &CookieJar<'_>, signup: Form<SignupForm>) -> Result<(), Error> {
        let first_name = &signup.first_name;
        let last_name = &signup.last_name;
        let email = &signup.email;
        let phone_number = &signup.phone;
        let company = &signup.company;
        let address = &signup.address;
        let local: DateTime<Local> = Local::now();

        let formatted_date_time = local.format("%Y-%m-%d %H:%M:%S").to_string();

        let (password_hash, password_salt) = auth_utils::hash_string(signup.password.to_string())?;
        
        let _ = db_handler::add_user(first_name.to_string(), last_name.to_string(), email.to_string(), "".to_string(), password_hash.to_string(), password_salt.to_string(), phone_number.to_string(), company.to_string(), address.to_string(), 0, formatted_date_time.to_string(), "".to_string()).await;
        match db_handler::get_user_id(email).await.map_err(|err| std::io::Error::new(std::io::ErrorKind::Other, err)) {
            Ok(Some(user_id)) => {
                println!("SignUp User ID: {}", user_id);
                jar.add_private(Cookie::new("user_id", user_id.to_string()));
            },
            Ok(_) => {
                eprintln!("Error!");
            },
            Err(e) => {
                eprintln!("{:?}", e);
            },
        }
        
        Ok(())
    }

    match determine_signup(jar, signup).await {
        Ok(()) => {
            Redirect::to("/dashboard")
        },
        Err(e) => {
            eprintln!("{:?}", e);
            Redirect::to("/signup")
        },
    }
}

// Handles signup. Obviously needs implementation of signup functionality.
#[get("/signup", rank = 2)]
fn signup_default(_jar: &CookieJar<'_>) -> Template {
    Template::render("signup", context!{ field: "value" })
}

// Renders Dashboard template for authenticated users.
#[get("/dashboard", rank = 1)]
fn dashboard(_session_user: SessionUser) -> Template {
    Template::render("dashboard", context!{ field: "value" })
}

// Redirects unauthenticated users to the login page.
#[get("/dashboard", rank = 2)]
fn dashboard_default() -> Redirect {
    Redirect::to("/login")
}

// Redirects user to index page after logout.
#[get("/logout")]
fn logout(jar: &CookieJar<'_>, _session_user: SessionUser) -> Redirect {
    jar.remove_private(Cookie::build("user_id"));
    Redirect::to("/")
}

// Redirects unauthenticated user to login on logout attempt.
#[get("/logout", rank = 2)]
fn logout_default() -> Redirect {
    Redirect::to("/login")
}

#[get("/new-api-key")]
fn new_api_key(jar: &CookieJar<'_>, _session_user: SessionUser) -> String {
    format!("GET TICKET {ticket_id}")
}
// route: /api/v1/
// create-ticket
// get-ticket
// check-ticket
// delete-ticket
// update-ticket
// TODO: format: json

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
    // OPTIONAL; FUTURE:
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
#[post("/ticket")]
fn api_create_ticket(_key: ApiKey) -> &'static str {
    "CREATE TICKET"
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


// Update a ticket by its ID (PUT request)
// Returns status code.
// needs API key
#[put("/ticket/<ticket_id>")]
fn api_update_ticket(ticket_id: &str, _key: ApiKey) -> String {
    format!("UPDATE TICKET {ticket_id}")
}

// Check if a ticket is valid by its ID (GET request)
// Returns bool and / or status code.
// doesn't necessarily need API key, though might be better, idk
// as of may 2nd 2024, 20:21 CEST: will be key protected to avoid "DDOS"
#[get("/ticket/check/<ticket_id>")]
fn api_check_ticket(ticket_id: &str, _key: ApiKey) -> String {
    format!("CHECK TICKET {ticket_id}")
}

// Generate and return a QR code for a ticket by its ID (GET request)
// Useful for example if you want to give your customer a
// document they can provide.
#[get("/ticket/qr/<ticket_id>")]
fn api_get_ticket_qr(ticket_id: &str, _key: ApiKey) -> String {
    format!("GET TICKET QR {ticket_id}")
}

// Ticket Document Route
// FUNCTIONALITY

#[tokio::main]
async fn main() {
    let _ = rocket::build()
        .configure(rocket::Config::figment().merge(("port", 1234)))
        .mount("/", routes![index_page, dashboard, dashboard_default, signup, signup_default, signup_handler, login, login_default, login_handler, logout, logout_default]) // Mounts routes
        .mount("/api/v1/", routes![api_create_ticket, api_get_ticket, api_delete_ticket, api_update_ticket, api_check_ticket, api_get_ticket_qr])
        .mount("/static", FileServer::from("../static"))
        .attach(Template::fairing()) // Attach fairing for templates
        .launch() // Start the Rocket server
        .await; // Await the server to start
}
