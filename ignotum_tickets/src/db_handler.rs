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

// define the way a db must look here, in the code, as MongoDB doesn't enforce a schema (NoSQL)
// Organization / Team features might be great here
// user db - not final in this form
// does not need credit card info, we use stripe
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

// not final
#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Ticket {
    _id: ObjectId,
    user_id: ObjectId,
    title: String,
    ticket_type: String,
    ticket_use_count: i32,
    max_ticket_uses: i32,
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

// Calls to this fn will provide empty strings for optional fields!
pub async fn add_user(first_name: String, last_name: String, email: String, encrypted_api_key: String, password_hash: String, password_salt: String, phone_number: String, company: String, address: String, api_limit: i32, date_time_joined: String, subscription_type: String) -> Result<String, mongodb::error::Error> {
    /*
        _id: ObjectId,
        first_name: String,
        last_name: String,
        email: String,
        api_key_hash: String,
        api_key_salt: String,
        password_hash: String,
        password_salt:  String,
        month_requests_count: i128,
        phone_number: String, //optional
        company: String,      //optional
        address: String,      //optional
        api_limit: i128,
        date_time_joined: String,
        subscription_type: String,
    */

    // Load the MongoDB connection string from an environment variable:
    let client_uri = env::var("MONGODB_URI").expect("You must set the MONGODB_URI environment var!");
    // A Client is needed to connect to MongoDB:
    // An extra line of code to work around a DNS issue on Windows:
    let options = ClientOptions::parse_with_resolver_config(&client_uri, ResolverConfig::cloudflare()).await?;
    let client = Client::with_options(options)?;
    let user_collection: Collection<User> = client.database("users").collection("ignotum-users");

    let user_document = User {
        _id: ObjectId::new(),
        first_name, 
        last_name, 
        email,
        encrypted_api_key,
        password_hash,
        password_salt,
        month_requests_count: 0,
        phone_number, //optional - calls to this fn will provide empty strings if not needed
        company,           //optional - calls to this fn will provide empty strings if not needed
        address,           //optional - calls to this fn will provide empty strings if not needed
        api_limit,
        date_time_joined,
        subscription_type,
    };

    match user_collection.insert_one(user_document, None).await {
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

// Returns all data for user with given email.
pub async fn get_user_data(email: String) -> Result<Option<User>, mongodb::error::Error> {
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
    let result: Result<Option<_>, mongodb::error::Error> = user_collection.find_one(filter, None).await;
    
    match result {
        Ok(Some(ref document)) => {
            let user =  User {
                _id: document._id, 
                first_name: document.first_name.clone(), 
                last_name: document.last_name.clone(), 
                email: document.email.clone(),
                encrypted_api_key: document.encrypted_api_key.clone(),
                password_hash: document.password_hash.clone(),
                password_salt: document.password_salt.clone(),
                month_requests_count: document.month_requests_count,
                phone_number: document.phone_number.clone(), //optional - calls to this fn will provide empty strings if not needed
                company: document.company.clone(),           //optional - calls to this fn will provide empty strings if not needed
                address: document.address.clone(),           //optional - calls to this fn will provide empty strings if not needed
                api_limit: document.api_limit,
                date_time_joined: document.date_time_joined.clone(),
                subscription_type: document.subscription_type.clone(),
            };
            
            Ok(Some(user))
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

pub async fn get_user_id(email: &String) -> Result<Option<String>, mongodb::error::Error> {
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
            println!("Found a match in collection.");
            Ok(Some(document._id.to_hex()))
        },
        Err(e) => {
            println!("Error: {:?}", e); // Handle the error case
            Err(e)
        },
        _ => {
            Ok(None)
        },
    }
}

pub async fn check_user(id: String) -> Result<bool, mongodb::error::Error> {
    // Load the MongoDB connection string from an environment variable:
    let client_uri = env::var("MONGODB_URI").expect("You must set the MONGODB_URI environment var!");
    // A Client is needed to connect to MongoDB:
    // An extra line of code to work around a DNS issue on Windows:
    let options = ClientOptions::parse_with_resolver_config(&client_uri, ResolverConfig::cloudflare())
        .await?;
    let client = Client::with_options(options)?;
    let user_collection: Collection<User> = client.database("users").collection("ignotum-users");

    // This is how an ObjectId is "created" from a String.
    let oid = match ObjectId::parse_str(&id) {
        Ok(oid) => oid,
        Err(e) => {
            println!("Error parsing ObjectId: {:?}", e);
            return Ok(false);
        }
    };

    let filter = doc! { "_id": oid };
    // There is also find() that returns all records / documents
    let result = user_collection.find_one(filter, None).await;

    match result {
        Ok(Some(ref _document)) => {
            println!("Found a match in collection.");
            Ok(true)
        },
        Err(e) => {
            println!("Error: {:?}", e); // Handle the error case
            Err(e)
        },
        _ => {
            Ok(false)
        },
    }
}

pub async fn get_user_auth_data(email: &String) -> Result<(String, String), mongodb::error::Error> {
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
            println!("Found a match in collection.");
            Ok((document.password_hash.to_string(), document.password_salt.to_string()))
        },
        Err(e) => {
            println!("Error: {:?}", e); // Handle the error case
            Err(e)
        },
        _ => {
            Ok(("error".to_string(), "error".to_string()))
        },
    }
}

pub async fn delete_user(email: String) -> Result<(), mongodb::error::Error> {
    // Load the MongoDB connection string from an environment variable:
    let client_uri = env::var("MONGODB_URI").expect("You must set the MONGODB_URI environment var!");
    // A Client is needed to connect to MongoDB:
    // An extra line of code to work around a DNS issue on Windows:
    let options = ClientOptions::parse_with_resolver_config(&client_uri, ResolverConfig::cloudflare())
        .await?;
    let client = Client::with_options(options)?;
    let user_collection: Collection<User> = client.database("users").collection("ignotum-users");

    let filter = doc! { "email": email };

    let result = user_collection.delete_one(filter, None).await?;
    println!("Deleted {:?} documents.", result.deleted_count);
    Ok(())
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
/*
pub async fn insert_ticket_document(user_id: ObjectId, title: String, ticket_type: String, ticket_use_count: i128, max_ticket_uses: i128, description: String, status: String, creation_date: String, update_date: String, close_date: String, ticket_holder_first_name: String, ticket_holder_last_name: String, ticket_holder_email: String,) -> Result<String, mongodb::error::Error> {
    /*
    // not final
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
    */
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
        user_id: user_id,
        title: title,
        ticket_type: ticket_type,
        ticket_use_count: ticket_use_count,
        max_ticket_uses: max_ticket_uses,
        description: description,
        status: status,
        creation_date: creation_date,
        update_date: update_date,
        close_date: close_date,
        ticket_holder_first_name: ticket_holder_first_name,
        ticket_holder_last_name: ticket_holder_last_name,
        ticket_holder_email: ticket_holder_email,
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