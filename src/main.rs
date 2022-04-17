#![feature(plugin, decl_macro, proc_macro_hygiene)]
#![allow(proc_macro_derive_resolution_fallback, unused_attributes)]

#[macro_use]
extern crate rocket;
extern crate rocket_contrib;

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate r2d2;
extern crate r2d2_diesel;

use dotenv::dotenv;
use std::env;

use routes::*;

pub mod database;
pub mod models;
pub mod routes;
pub mod schema;

// use database::Database;

fn rocket() -> rocket::Rocket {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = database::init_pool(database_url);
    rocket::ignite()
        .manage(pool)
        .mount("/api/v1/", routes![index, new])
}

fn main() {
    rocket().launch();
}
