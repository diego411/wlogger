use crate::db::database;
use crate::db::models;
use std::time::{SystemTime, UNIX_EPOCH};
use twitch_irc::login::StaticLoginCredentials;
use twitch_irc::message::ServerMessage;
use twitch_irc::TwitchIRCClient;
use twitch_irc::{ClientConfig, SecureTCPTransport};

use std::collections::HashMap;
use std::env;

pub async fn setup() {
    let pool = database::init_pool();
    let conn = &pool.get().unwrap();

    let config = ClientConfig::default();
    let (mut incoming_messages, client) =
        TwitchIRCClient::<SecureTCPTransport, StaticLoginCredentials>::new(config);

    let join_handle = tokio::spawn(async move {
        while let Some(message) = incoming_messages.recv().await {
            match message {
                ServerMessage::Privmsg(msg) => {
                    if filter(msg.channel_login.clone(), msg.message_text.clone()).await {
                        println!(
                            "[#{:?}] {:?}: {:?}",
                            msg.channel_login, msg.sender.name, msg.message_text
                        );
                        let timestamp = SystemTime::now()
                            .duration_since(UNIX_EPOCH)
                            .expect("could not get current time");

                        let new_message = models::NewMessage {
                            channel: msg.channel_login,
                            content: msg.message_text,
                            sender_login: msg.sender.name,
                            post_timestamp: timestamp.as_secs_f64() as i32,
                        };
                        database::insert_message(new_message, &pool.get().unwrap());
                    }
                }
                _ => {}
            }
        }
    });

    let all_channels = database::every_channel(conn);
    for channel in all_channels {
        client.join(channel.channel_name.to_owned()).unwrap();
    }

    join_handle.await.unwrap();
}

#[derive(Deserialize, Debug)]
struct WEDResponse {
    response_code: i32,
    is_weeb: bool,
    confidence: f32,
    number_of_weeb_terms: i32,
}

async fn filter(channel: String, message: String) -> bool {
    let mut req_body = HashMap::new();
    req_body.insert("channel", channel);
    req_body.insert("message", message);

    let client = reqwest::Client::new();
    let wed_base_url = env::var("WED_URL").expect("WED URL must be set");
    let resp = client
        .get(wed_base_url + "api/v1/hwis")
        .json(&req_body)
        .send()
        .await
        .expect("Encountered issue with WED API");

    let resp_body = resp
        .text()
        .await
        .expect("Encountered issue reading body of WED response");

    let data = serde_json::from_str::<WEDResponse>(&resp_body[..])
        .expect("Encountered issue parsing WED response");

    return data.is_weeb;
}
