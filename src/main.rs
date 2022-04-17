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

use twitch_irc::login::StaticLoginCredentials;
use twitch_irc::TwitchIRCClient;
use twitch_irc::{ClientConfig, SecureTCPTransport};

pub mod database;
pub mod models;
pub mod routes;
pub mod schema;

fn rocket() -> rocket::Rocket {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = database::init_pool(database_url);
    rocket::ignite()
        .manage(pool)
        .mount("/api/v1/", routes![index, new])
}

#[tokio::main]
pub async fn main() {
    tokio::spawn(async move {
        rocket().launch();
    });

    let config = ClientConfig::default();
    let (mut incoming_messages, client) =
        TwitchIRCClient::<SecureTCPTransport, StaticLoginCredentials>::new(config);

    let join_handle = tokio::spawn(async move {
        while let Some(message) = incoming_messages.recv().await {
            println!("Received message: {:?}", message);
        }
    });

    client.join("daumenloser".to_owned()).unwrap();

    join_handle.await.unwrap();
}
