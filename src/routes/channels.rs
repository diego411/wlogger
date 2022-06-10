use crate::db::database;
use crate::db::database::Conn as db_conn;
use crate::db::models::{Channel, NewChannel};
use crate::twitchclient::TwitchClient;
use rocket::State;
use rocket_contrib::json::Json;
use serde_json::Value;

use std::sync::Arc;

#[get("/channels", format = "application/json")]
pub fn channel_index(conn: db_conn) -> Json<Value> {
    let all_channels = database::every_channel(&conn);

    Json(json!({
        "status": 200,
        "result": all_channels,
    }))
}

#[get("/channels/<channel_name>", format = "application/json")]
pub fn channel(conn: db_conn, channel_name: String) -> Json<Value> {
    match database::channel_with_name(channel_name.clone(), &conn) {
        Some(_) => (),
        None => {
            return Json(json!({
                "status": 200,
                "channel_name": channel_name,
                "exists": false,
                "message_count": 0,
                "score": 0,
                "messages": Vec::<Channel>::new(),
            }))
        }
    }

    let all_messages_in_channel = database::messages_in_channel(channel_name.clone(), &conn);
    let message_count = all_messages_in_channel.len();

    let mut score = 0;
    for message in &all_messages_in_channel {
        score = score + message.score;
    }

    Json(json!({
        "status": 200,
        "channel_name": channel_name,
        "exists": true,
        "message_count": message_count,
        "score": score,
        "messages": all_messages_in_channel,
    }))
}

#[post("/channels", format = "application/json", data = "<new_channel>")]
pub fn new_channel(
    conn: db_conn,
    twitch_client: State<Arc<TwitchClient>>,
    new_channel: Json<NewChannel>,
) -> Json<Value> {
    let channel = new_channel.into_inner();
    twitch_client.join(channel.channel_name.to_owned());
    Json(json!({
        "status": database::insert_channel(channel, &conn),
        "result": database::every_channel(&conn).first(),
    }))
}
