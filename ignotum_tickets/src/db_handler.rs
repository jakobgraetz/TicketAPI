/*
* @author Jakob Grätz, Johannes Schießl | @jakobgraetz, @johannesschiessl
* @edition 23/04/2024 DD/MM/YYYY
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
use bson::doc;
use chrono::prelude::*;

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct User {
    _id: ObjectId,
    first_name: String,
    last_name: String,
    email: String,
    encrypted_api_key: String,
    password_hash: String,
    password_salt:  String,
    month_requests_count: i32,
    phone_number: String, //optional
    company: String,      //optional
    address: String,      //optional
    api_limit: i32,    
    date_time_joined: String,
    subscription_type: String,
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
    ticket_holder_first_name: String,
    ticket_holder_last_name: String,
    ticket_holder_email: String,
}
/*
pub async fn get_user_id(api_key: String) -> Result<String, mongodb::error::Error> {
    let client_uri = env::var("MONGODB_URI").expect("You must set the MONGODB_URI environment var!");
    let options = ClientOptions::parse_with_resolver_config(&client_uri, ResolverConfig::cloudflare()).await?;

    let client = Client::with_options(options)?;
    let ticket_collection: Collection<Ticket> = client.database("users").collection("ignotum-users");
}

pub async fn update_request_count(api_key: String) -> Result<(), mongodb::error::Error> {
    let client_uri = env::var("MONGODB_URI").expect("You must set the MONGODB_URI environment var!");
    let options = ClientOptions::parse_with_resolver_config(&client_uri, ResolverConfig::cloudflare()).await?;

    let client = Client::with_options(options)?;
    let ticket_collection: Collection<Ticket> = client.database("users").collection("ignotum-users");
    
}
*/
pub async fn insert_ticket_doc(user_id: ObjectId, title: String, close_date: String, customer_first_name: String, customer_last_name: String, customer_email: String) -> Result<String, mongodb::error::Error> {
    let client_uri = env::var("MONGODB_URI").expect("You must set the MONGODB_URI environment var!");
    let options = ClientOptions::parse_with_resolver_config(&client_uri, ResolverConfig::cloudflare()).await?;

    let client = Client::with_options(options)?;
    let ticket_collection: Collection<Ticket> = client.database("tickets").collection("ignotum-tickets");

    let ticket_document = Ticket {
        _id: ObjectId::new(),
        user_id: user_id,
        title: title,
        status: "unchecked".to_string(),
        creation_date: Utc::now().to_string(),
        update_date: Utc::now().to_string(),
        close_date: close_date,
        ticket_holder_first_name: customer_first_name,
        ticket_holder_last_name: customer_last_name,
        ticket_holder_email: customer_email,
    };

    match ticket_collection.insert_one(ticket_document, None).await {
        Ok(insert_one_result) => {
            println!("Inserted doc with id: {}", insert_one_result.inserted_id);
            Ok(insert_one_result.inserted_id.to_string())
        },
        Err(e) => {
            println!("Error inserting document: {}", e);
            Err(e)
        }
    }
}

/*
pub async fn insert_ticket_document(user_id: ObjectId, title: String, ticket_type: String, ticket_use_count: i128, max_ticket_uses: i128, description: String, status: String, creation_date: String, update_date: String, close_date: String, ticket_holder_first_name: String, ticket_holder_last_name: String, ticket_holder_email: String,)  {
    #[derive(Serialize, Deserialize)]
    #[serde(crate = "rocket::serde")]
    pub struct Ticket {
        _id: ObjectId,
        user_id: ObjectId,
        title: String,
        ticket_type: String,
        ticket_use_count: i128,
        max_ticket_uses: i128,
        description: String,
        status: String,
        creation_date: String,
        update_date: String,
        close_date: String,

        // optional
        ticket_holder_first_name: String,
        ticket_holder_last_name: String,
        ticket_holder_email: String,
    }
    // Load the MongoDB connection string from an environment variable:
    
    // A Client is needed to connect to MongoDB:
    // An extra line of code to work around a DNS issue on Windows:
    
    

    

    
}



// Will delete ticket with unique id.
pub async fn delete_ticket(ticket_id: ObjectId) -> Result<(), mongodb::error::Error> {
    // Load the MongoDB connection string from an environment variable:
    let client_uri = env::var("MONGODB_URI").expect("You must set the MONGODB_URI environment var!");
    // A Client is needed to connect to MongoDB:
    // An extra line of code to work around a DNS issue on Windows:
    let options = ClientOptions::parse_with_resolver_config(&client_uri, ResolverConfig::cloudflare())
        .await?;
    let client = Client::with_options(options)?;
    let ticket_collection: Collection<Ticket> = client.database("tickets").collection("ignotum-tickets");

    let filter = doc! { "_id": ticket_id };

    let result = ticket_collection.delete_one(filter, None).await?;
    println!("Deleted {:?} documents.", result.deleted_count);
    Ok(())
}

/*
- handling all getting operations in one function
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
*/





// If the user owns a ticket (though that is not a problem, as each ticket has a unique id) it will return all the ticket data
pub async fn get_ticket_data(user_id: ObjectId, ticket_id: ObjectId) -> Result<Option<Ticket>, mongodb::error::Error> {
    // Load the MongoDB connection string from an environment variable:
    let client_uri = env::var("MONGODB_URI").expect("You must set the MONGODB_URI environment var!");
    // A Client is needed to connect to MongoDB:
    // An extra line of code to work around a DNS issue on Windows:
    let options = ClientOptions::parse_with_resolver_config(&client_uri, ResolverConfig::cloudflare())
        .await?;
    let client = Client::with_options(options)?;
    let ticket_collection: Collection<Ticket> = client.database("tickets").collection("ignotum-tickets");

    let filter = doc! { "_id": ticket_id, "user_id": user_id };
    // There is also find() that returns all records / documents
    let result = ticket_collection.find_one(filter, None).await;
    
    match result {
        Ok(Some(ref document)) => {
            // TICKET
            let ticket = Ticket {
                _id: document._id.clone(),
                user_id: document.user_id.clone(),
                title: document.title.clone(),
                // description: document.description.clone(),
                status: document.status.clone(),
                creation_date: document.creation_date.clone(),
                update_date: document.update_date.clone(),
                close_date: document.close_date.clone(),
                // customer_name: document.customer_name.clone(),
                // customer_email: document.customer_email.clone(),
                // customer_phone: document.customer_phone.clone(),
                // location: document.location.clone(),
                // quantity: document.quantity.clone(),
                // price: document.price.clone(),
                // payment_status: document.payment_status.clone(),
                // payment_date: document.payment_date.clone(),
                // payment_method: document.payment_method.clone(),
            };
            
            Ok(Some(ticket))
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
*/