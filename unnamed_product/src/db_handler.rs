/*
* @author Jakob Grätz, Johannes Schießl | @jakobgraetz, @johannesschiessl
* @edition 17/03/2024 DD/MM/YYYY
* @version v0.0.1
* @description Rust file responsible for handling MongoDB (atlas) connection and databases.
* @note The allowed IP in the Atlas Web / DB deployment may needs to be adjusted based on the server IP ...
*/

use mongodb::{Client, options::{ClientOptions, ResolverConfig}};
use std::env;
use std::error::Error;

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
    Ok(())
}