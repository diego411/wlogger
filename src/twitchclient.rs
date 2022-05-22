use crate::db::database;
use crate::db::models;
use std::collections::HashSet;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::sync::mpsc::UnboundedReceiver;
use twitch_irc::login::StaticLoginCredentials;
use twitch_irc::message::ServerMessage;
use twitch_irc::TwitchIRCClient;
use twitch_irc::{ClientConfig, SecureTCPTransport};

use std::collections::HashMap;
use std::env;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug)]
pub struct TwitchClient {
    pub client: TwitchIRCClient<SecureTCPTransport, StaticLoginCredentials>,
    pub irc_stream: Arc<Mutex<UnboundedReceiver<ServerMessage>>>,
}

impl TwitchClient {
    pub fn new() -> TwitchClient {
        let config = ClientConfig::default();
        let (mut incoming_messages, client) =
            TwitchIRCClient::<SecureTCPTransport, StaticLoginCredentials>::new(config);
        TwitchClient {
            client: client,
            irc_stream: Arc::new(Mutex::new(incoming_messages)),
        }
    }

    pub fn stream(&self) -> Arc<Mutex<UnboundedReceiver<ServerMessage>>> {
        Arc::clone(&self.irc_stream)
    }

    pub async fn run(client_self: Arc<TwitchClient>) {
        let client = Arc::clone(&client_self);
        let pool = database::init_pool();
        let conn = &pool.get().unwrap();

        let join_handle = tokio::spawn(async move {
            while let Some(message) = client.stream().lock().await.recv().await {
                match message {
                    ServerMessage::Privmsg(msg) => {
                        let wed_response =
                            fetch_wed_response(msg.channel_login.clone(), msg.message_text.clone())
                                .await;

                        let wed_response = match wed_response {
                            Ok(response) => response,
                            Err(_) => {
                                println!("WED fetch failed in [#{}]", msg.channel_login);
                                continue;
                            }
                        };

                        if wed_response.is_weeb {
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
                                sender_login: msg.sender.name.clone(),
                                post_timestamp: timestamp.as_secs_f64() as i32,
                                score: wed_response.number_of_weeb_terms,
                            };

                            let user = models::NewUser {
                                user_login: msg.sender.name,
                            };

                            database::insert_user(user, &pool.get().unwrap());
                            database::insert_message(new_message, &pool.get().unwrap());
                        }
                    }
                    _ => {}
                }
            }
        });

        let all_channels = database::every_channel(conn);
        for channel in all_channels {
            client_self.join(channel.channel_name.to_owned());
        }

        join_handle.await.unwrap();
    }

    pub fn join(&self, channel_login: String) {
        self.client.join(channel_login);
    }
}

#[derive(Deserialize, Debug)]
struct WEDResponse {
    response_code: i32,
    is_weeb: bool,
    confidence: f32,
    number_of_weeb_terms: i32,
}

async fn fetch_wed_response(
    channel: String,
    message: String,
) -> Result<WEDResponse, Box<dyn std::error::Error>> {
    let mut req_body = HashMap::new();
    req_body.insert("channel", channel);
    req_body.insert("message", message);

    let client = reqwest::Client::new();
    let wed_base_url = env::var("WED_URL").expect("WED URL must be set");
    let resp = client
        .get(wed_base_url + "api/v1/hwis")
        .json(&req_body)
        .send()
        .await?;
    let resp_body = resp.text().await?;
    let response = serde_json::from_str::<WEDResponse>(&resp_body[..])?;
    Ok(response)
}
