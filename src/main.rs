#[macro_use]
extern crate diesel;
extern crate dotenv;
pub mod database;

use diesel::pg::PgConnection;
use diesel::prelude::*;

use dotenv::dotenv;
use std::env;

pub mod models;
pub mod schema;

use database::Database;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}
fn main() {
    let conn: PgConnection = establish_connection();
    let db: Database = Database::new(conn);

    let message = models::NewMessage {
        content: String::from("FeelsDankMan FeelsDankMan"),
        sender_login: String::from("daumenloser"),
        channel: String::from("xqcow"),
        post_timestamp: 1650214965,
    };

    if db.insert(message) {
        println!("success");
    } else {
        println!("failed")
    }
}
