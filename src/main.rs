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

use db::database;
use routes::routes::*;

pub mod db;
// pub mod models;
pub mod routes;
// pub mod schema;
pub mod twitchclient;

fn rocket() -> rocket::Rocket {
    let pool = database::init_pool();
    rocket::ignite()
        .manage(pool)
        .mount("/api/v1/", routes![index, new, new_channel, channel_index])
}

#[tokio::main]
pub async fn main() {
    dotenv().ok();

    tokio::spawn(async move {
        rocket().launch();
    });

    twitchclient::setup().await;
}
