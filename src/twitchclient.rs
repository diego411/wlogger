use crate::database;
use crate::models;
use twitch_irc::login::StaticLoginCredentials;
use twitch_irc::message::ServerMessage;
use twitch_irc::TwitchIRCClient;
use twitch_irc::{ClientConfig, SecureTCPTransport};

pub async fn setup() {
    println!("starting setup");
    let pool = database::init_pool();

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
                    let new_message = models::NewMessage {
                        channel: msg.channel_login,
                        content: msg.message_text,
                        sender_login: msg.sender.name,
                        post_timestamp: 1,
                    };
                    database::insert(new_message, &pool.get().unwrap());
                }
                _ => {}
            }
        }
    });

    client.join("daumenloser".to_owned()).unwrap();
    println!("joined #daumenloser");

    join_handle.await.unwrap();
}
