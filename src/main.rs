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
use std::sync::Arc;

use db::database;
use routes::channels::*;
use routes::messages::*;
use routes::users::*;

pub mod db;
pub mod routes;
pub mod twitchclient;

use twitchclient::TwitchClient;

fn rocket(twitch_client: Arc<TwitchClient>) -> rocket::Rocket {
    let pool = database::init_pool();
    rocket::ignite().manage(pool).manage(twitch_client).mount(
        "/api/v1/",
        routes![
            message_index,
            channel,
            new_message,
            new_channel,
            channel_index,
            user_index,
            user,
            new_user
        ],
    )
}

#[tokio::main]
pub async fn main() {
    dotenv().ok();

    database::run_migrations();

    let twitch_client = Arc::new(TwitchClient::new());
    let twitch_client_arc_clone = Arc::clone(&twitch_client);
    tokio::spawn(async move {
        rocket(twitch_client_arc_clone).launch();
    });

    TwitchClient::run(twitch_client).await;
}
