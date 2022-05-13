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

#[macro_use]
extern crate diesel_migrations;

extern crate dotenv;
extern crate r2d2;
extern crate r2d2_diesel;

use dotenv::dotenv;

use db::database;
use routes::channels::*;
use routes::messages::*;
use routes::users::*;

pub mod db;
pub mod routes;
pub mod twitchclient;

fn rocket() -> rocket::Rocket {
    let pool = database::init_pool();
    rocket::ignite().manage(pool).mount(
        "/api/v1/",
        routes![
            message_index,
            new_message,
            new_channel,
            channel_index,
            user_index,
            new_user
        ],
    )
}

#[tokio::main]
pub async fn main() {
    dotenv().ok();

    database::run_migrations();

    tokio::spawn(async move {
        rocket().launch();
    });

    twitchclient::setup().await;
}
