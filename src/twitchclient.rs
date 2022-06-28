use crate::db::database;
use crate::db::models;
use crate::db::models::NewUser;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::sync::mpsc::UnboundedReceiver;
use twitch_irc::login::StaticLoginCredentials;
use twitch_irc::message::ServerMessage;
use twitch_irc::TwitchIRCClient;
use twitch_irc::{ClientConfig, SecureTCPTransport};

use std::sync::Arc;
use tokio::sync::Mutex;

use crate::controllers::wed as WEDController;

#[derive(Debug)]
pub struct TwitchClient {
    pub client: TwitchIRCClient<SecureTCPTransport, StaticLoginCredentials>,
    pub irc_stream: Arc<Mutex<UnboundedReceiver<ServerMessage>>>,
}

impl TwitchClient {
    pub fn new() -> TwitchClient {
        let config = ClientConfig::default();
        let (incoming_messages, client) =
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
                        let mut new_user: Option<NewUser> = None;
                        match database::user_with_name(
                            msg.sender.name.clone(),
                            &pool.get().unwrap(),
                        ) {
                            Some(user) => {
                                if user.opted_out {
                                    continue;
                                }
                            }
                            None => {
                                new_user = Some(models::NewUser {
                                    user_login: msg.sender.name.clone(),
                                    opted_out: false,
                                });
                            }
                        }

                        let mut emotes = std::collections::HashMap::new();

                        for emote in msg.emotes {
                            emotes.insert(emote.code, emote.id);
                        }

                        let wed_response = WEDController::fetch_wed_response(
                            msg.channel_login.clone(),
                            msg.message_text.clone(),
                            emotes,
                        )
                        .await;

                        let wed_response = match wed_response {
                            Ok(response) => response,
                            Err(_) => {
                                println!("WED fetch failed in [#{}]", msg.channel_login);
                                continue;
                            }
                        };

                        if wed_response.is_weeb {
                            //TODO this should be cached
                            if !database::is_channel_actively_logged(
                                msg.channel_login.clone(),
                                &pool.get().unwrap(),
                            ) {
                                continue;
                            };
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
                                score: wed_response.number_of_weeb_terms,
                            };

                            match new_user {
                                None => (),
                                Some(user) => {
                                    database::insert_user(user, &pool.get().unwrap());
                                }
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
            client_self.join(channel.channel_name.to_owned());
        }

        join_handle.await.unwrap();
    }

    pub fn join(&self, channel_login: String) {
        match self.client.join(channel_login.clone()) {
            Ok(()) => println!("Joined {}", channel_login),
            Err(err) => println!("Failed to join {} with err: {}", channel_login, err),
        }
    }
}
