#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod schema;
pub mod models;
pub mod operations;
mod errors;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;
use rand::{distributions::Alphanumeric, Rng};

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let db_url = env::var("DATABASE_URI")
        .unwrap();
    PgConnection::establish(&db_url)
        .expect("Error connecting to Postgres Database")
}

pub fn rand(length: usize) -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect()
}