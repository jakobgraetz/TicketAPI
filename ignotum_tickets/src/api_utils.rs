/*
* @author Jakob Grätz, Johannes Schießl | @jakobgraetz, @johannesschiessl
* @edition 23/04/2024 DD/MM/YYYY
* @version v0.0.1
* @description Rust file for api utilities.
*/
/* 
// DEPENDENCIES
extern crate chrono;

use chrono::{NaiveDate, Local};

// HELPER FUNCTIONS
/*
* @author Johannes Schießl
* @description Checks if the input date string represents a date in the future.
*/
pub fn is_date_in_future(date_str: &str) -> bool {
    match NaiveDate::parse_from_str(date_str, "%Y-%m-%d") {
        Ok(input_date) => {
            let current_date = Local::now().naive_local().into();
            input_date > current_date
        },
        Err(_) => false,
    }
}
*/