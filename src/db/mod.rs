use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

pub mod model;
pub mod schema;


pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();
    let database_url = "db/db";
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}