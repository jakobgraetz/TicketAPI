/*
* @author Jakob Grätz, Johannes Schießl | @jakobgraetz, @johannesschiessl
* @edition 14/04/2024 DD/MM/YYYY
* @version v0.0.1
* @description Rust file responsible for handling MongoDB (atlas) connection and databases.
* @note The allowed IP in the Atlas Web / DB deployment may needs to be adjusted based on the server or rather, client, IP ... also export MONGODB_URI env var
*/

// TODO: Write actual developer documentation for Rust, mainly MongoDB Rust Driver (for me, internal).
// TODO: Implement actual functionality, or rather more functionality

/*
Some good docs for Rust MongoDB:
https://mongodb.github.io/mongo-rust-driver/manual/reading.html
https://taharmeijs.medium.com/beginners-guide-to-mongodb-and-rust-8d8d3ef17920
*/

// Imports
use mongodb::{Client, options::{ClientOptions, ResolverConfig}, bson::oid::ObjectId};
use std::env;
extern crate serde;
extern crate serde_json;
use rocket::{serde::{Deserialize, Serialize}};
use mongodb::Collection;
use mongodb::results::InsertOneResult;
use bson::doc;

// define the way a db must look here, in the code, as MongoDB doesn't enforce a schema (NoSQL)
// user db - not final in this form
#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct User {
    _id: ObjectId,
    first_name: String,
    last_name: String,
    // Organization / Team features might be great here
    email: String,
    api_key_hash: String,
    user_password_hash: String,
    // More user info: payment, ...
}

// not final
#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct Ticket {
    _id: ObjectId,
    // event_id: ObjectId,
    // This is the id of the user who created the ticket, necessary so we can keep track of who
    // issued what tickets, billing, ...
    user_id: ObjectId,
    title: String,
    description: String,
    status: String,
    creation_date: String,
    update_date: String,
    close_date: String,
    customer_name: String,
    customer_email: String,
    customer_phone: String,
    location: String,
    quantity: usize,
    price: usize,
    // Maybe bool in future.
    payment_status: String,
    payment_date: String,
    payment_method: String,
}

/*
pub async fn test_db() -> Result<(), Box<dyn Error>> {
    // dev, not for production, just so I learn about working with MongoDB.
    // Load the MongoDB connection string from an environment variable:
    let client_uri = env::var("MONGODB_URI").expect("You must set the MONGODB_URI environment var!");
    // A Client is needed to connect to MongoDB:
    // An extra line of code to work around a DNS issue on Windows:
    let options = ClientOptions::parse_with_resolver_config(&client_uri, ResolverConfig::cloudflare())
        .await?;
    let client = Client::with_options(options)?;
    // Print the databases in our MongoDB cluster:
    println!("Databases:");
    for name in client.list_database_names(None, None).await? {
        println!("- {}", name);
    }

    // each collection needs a struct defined here for the <T> -> mongodb::Collection<T>
    // for example: struct Movies -> mongodb::Collection<Movies>
    // Get the 'movies' collection from the 'sample_mflix' database:
    // let movies = client.database("sample_mflix").collection("movies");
    // println!("Testing MongoDB's sample mflix database:");
    // println!("{:?}", movies);
    // Delete the 'sample_mflix' database
    // client.database("sample_mflix").drop(None).await?;
    // println!("Deleted database 'sample_mflix'.");
    Ok(())
}
*/

pub async fn insert_user_document(first_name: String, last_name: String, email: String, api_key_hash: String, user_password_hash: String) -> Result<InsertOneResult, mongodb::error::Error> {
    // Load the MongoDB connection string from an environment variable:
    let client_uri = env::var("MONGODB_URI").expect("You must set the MONGODB_URI environment var!");
    // A Client is needed to connect to MongoDB:
    // An extra line of code to work around a DNS issue on Windows:
    let options = ClientOptions::parse_with_resolver_config(&client_uri, ResolverConfig::cloudflare())
        .await?;
    let client = Client::with_options(options)?;
    let user_collection: Collection<User> = client.database("users").collection("ignotum-users");

    let user_document = User {
        _id: ObjectId::new(), 
        first_name: first_name, 
        last_name: last_name, 
        email: email, 
        api_key_hash: api_key_hash, 
        user_password_hash: user_password_hash
    };

    match user_collection.insert_one(user_document, None).await {
        Ok(insert_one_result) => {
            println!("Inserted doc with id: {}", insert_one_result.inserted_id);
            Ok(insert_one_result)
        },
        Err(e) => {
            println!("Error inserting document: {}", e);
            Err(e)
        }
    }
}

pub async fn insert_ticket_document(title: String, description: String, status: String, creation_date: String, update_date: String, close_date: String, customer_name: String, customer_email: String, customer_phone: String, location: String, quantity: usize, price: usize, payment_status: String, payment_date: String, payment_method: String) -> Result<InsertOneResult, mongodb::error::Error> {
    // Load the MongoDB connection string from an environment variable:
    let client_uri = env::var("MONGODB_URI").expect("You must set the MONGODB_URI environment var!");
    // A Client is needed to connect to MongoDB:
    // An extra line of code to work around a DNS issue on Windows:
    let options = ClientOptions::parse_with_resolver_config(&client_uri, ResolverConfig::cloudflare())
        .await?;
    let client = Client::with_options(options)?;
    let ticket_collection: Collection<Ticket> = client.database("tickets").collection("ignotum-tickets");

    let ticket_document = Ticket {
        _id: ObjectId::new(),
        // event_id: ObjectId::new(),
        // Will not generate a new ID, but insert the ID of the user who created the ticket!!!
        user_id: ObjectId::new(), 
        title: title, 
        description: description, 
        status: status, 
        creation_date: creation_date, 
        update_date: update_date,
        close_date: close_date,
        customer_name: customer_name,
        customer_email: customer_email,
        customer_phone: customer_phone,
        location: location,
        quantity: quantity,
        price: price,
        payment_status: payment_status,
        payment_date: payment_date,
        payment_method: payment_method
    };

    match ticket_collection.insert_one(ticket_document, None).await {
        Ok(insert_one_result) => {
            println!("Inserted doc with id: {}", insert_one_result.inserted_id);
            Ok(insert_one_result)
        },
        Err(e) => {
            println!("Error inserting document: {}", e);
            Err(e)
        }
    }
}
/*
pub async fn delete_user() {

}

pub async fn delete_ticket() {

}

pub async fn check_api_key() {

}

pub async fn check_password() {

}
*/
pub async fn get_user_id(email: String) -> Result<Option<ObjectId>, mongodb::error::Error> {
    // Load the MongoDB connection string from an environment variable:
    let client_uri = env::var("MONGODB_URI").expect("You must set the MONGODB_URI environment var!");
    // A Client is needed to connect to MongoDB:
    // An extra line of code to work around a DNS issue on Windows:
    let options = ClientOptions::parse_with_resolver_config(&client_uri, ResolverConfig::cloudflare())
        .await?;
    let client = Client::with_options(options)?;
    let user_collection: Collection<User> = client.database("users").collection("ignotum-users");

    let filter = doc! { "email": email };
    // There is also find() that returns all records / documents
    let result = user_collection.find_one(filter, None).await;
    
    match result {
        Ok(Some(ref document)) => {
            let user_id =  document._id.clone(); // Selecting user_id from the document
            println!("user id {:?}", user_id);
            Ok(Some(user_id))
        },
        Ok(None) => {
            println!("Unable to find a match in collection.");
            Ok(None)
        },
        Err(e) => {
            println!("Error: {:?}", e); // Handle the error case
            Err(e)
        }
    }
}

/* 
pub async fn get_user_data() {

}

pub async fn get_ticket_data() {

}

pub async fn check_ticket() {

}

pub async fn check_user() {

}
*/