use crate::db::database;
use crate::db::models;
use std::time::{SystemTime, UNIX_EPOCH};
use twitch_irc::login::StaticLoginCredentials;
use twitch_irc::message::ServerMessage;
use twitch_irc::TwitchIRCClient;
use twitch_irc::{ClientConfig, SecureTCPTransport};

pub async fn setup() {
    println!("starting setup");
    let pool = database::init_pool();
    let conn = &pool.get().unwrap();

    let config = ClientConfig::default();
    let (mut incoming_messages, client) =
        TwitchIRCClient::<SecureTCPTransport, StaticLoginCredentials>::new(config);

    let join_handle = tokio::spawn(async move {
        while let Some(message) = incoming_messages.recv().await {
            match message {
                ServerMessage::Privmsg(msg) => {
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
