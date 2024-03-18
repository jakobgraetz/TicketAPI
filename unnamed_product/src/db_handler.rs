/*
* @author Jakob Grätz, Johannes Schießl | @jakobgraetz, @johannesschiessl
* @edition 17/03/2024 DD/MM/YYYY
* @version v0.0.1
* @description Rust file responsible for handling MongoDB (atlas) connection and databases.
* @note The allowed IP in the Atlas Web / DB deployment may needs to be adjusted based on the server IP ...
*/

use mongodb::{Client, options::{ClientOptions, ResolverConfig}, bson::oid::ObjectId};
use std::env;
use std::error::Error;

// define the way a db must look here, in the code, as MongoDB doesn't enforce a schema (NoSQL)
// user db - not final in this form
struct User {
    _id: ObjectId,
    first_name: String,
    last_name: String,
    // Organization / Team features might be great here
    email: String,
    api_key_hash: String,
    user_password_hash: String
}
#[tokio::main]
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
    println!("{:?}", movies);*/
    // Delete the 'sample_mflix' database
   /*client.database("sample_mflix").drop(None).await?;
    println!("Deleted database 'sample_mflix'."); */
    Ok(())
}

// All functions with the purpose of "write-access", for example: inserting user into db





// All functions with the purpose of "read-access", for example: check if a given API key is in db