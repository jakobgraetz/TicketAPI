/*
* @author Jakob Grätz, Johannes Schießl | @jakobgraetz, @johannesschiessl
* @edition 30/03/2024 DD/MM/YYYY
* @version v0.0.1
* @description Rust file responsible for handling MongoDB (atlas) connection and databases.
* @note The allowed IP in the Atlas Web / DB deployment may needs to be adjusted based on the server IP ... also export MONGODB_URI env var
*/

use mongodb::{Client, options::{ClientOptions, ResolverConfig}, bson::oid::ObjectId};
use std::env;
use std::error::Error;
extern crate serde;
extern crate serde_json;
use rocket::{serde::{Deserialize, Serialize}};
use mongodb::Collection;
use mongodb::results::InsertOneResult;

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
    user_password_hash: String
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct Ticket {
    id: ObjectId,
    event_id: ObjectId,
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
    payment_status: String,
    payment_date: String,
    payment_method: String,
    comments: String,
}

pub async fn test_db() -> Result<(), Box<dyn Error>> {
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

    // TODO: dev, not for production, just so I learn about working with MongoDB.
    // Hypothesis: each collection needs a struct defined here for the <T> -> mongodb::Collection<T>
    // for example: struct Movies -> mongodb::Collection<Movies>
    // Get the 'movies' collection from the 'sample_mflix' database:
  /*let movies = client.database("sample_mflix").collection("movies");
    println!("Testing MongoDB's sample mflix database:");
    println!("{:?}", movies);
    // Delete the 'sample_mflix' database
    client.database("sample_mflix").drop(None).await?;
    println!("Deleted database 'sample_mflix'."); */
    Ok(())
}

// All functions with the purpose of "write-access", for example: inserting user into db
pub async fn insert_user_document() -> Result<InsertOneResult, mongodb::error::Error> {
    // Load the MongoDB connection string from an environment variable:
    let client_uri = env::var("MONGODB_URI").expect("You must set the MONGODB_URI environment var!");
    // A Client is needed to connect to MongoDB:
    // An extra line of code to work around a DNS issue on Windows:
    let options = ClientOptions::parse_with_resolver_config(&client_uri, ResolverConfig::cloudflare())
        .await?;
    let client = Client::with_options(options)?;
    let user_collection: Collection<User> = client.database("users").collection("ignotum-users");
    let user_document = User {_id: ObjectId::new(), first_name: "Jakob".to_string(), last_name: "Grätz".to_string(), email: "jakob.graetz@icloud.com".to_string(), api_key_hash: "my-fake-secret-key".to_string(), user_password_hash: "my-fake-password".to_string()};

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

pub async fn insert_ticket_document() {
    // Load the MongoDB connection string from an environment variable:
    let client_uri = env::var("MONGODB_URI").expect("You must set the MONGODB_URI environment var!");
    // A Client is needed to connect to MongoDB:
    // An extra line of code to work around a DNS issue on Windows:
    let options = ClientOptions::parse_with_resolver_config(&client_uri, ResolverConfig::cloudflare())
        .await?;
    let client = Client::with_options(options)?;
    let user_collection: Collection<Ticket> = client.database("tickets").collection("ignotum-tickets");
    let user_document = Ticket {_id: ObjectId::new(), first_name: "Jakob".to_string(), last_name: "Grätz".to_string(), email: "jakob.graetz@icloud.com".to_string(), api_key_hash: "my-fake-secret-key".to_string(), user_password_hash: "my-fake-password".to_string()};

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



// All functions with the purpose of "read-access", for example: check if a given API key is in db