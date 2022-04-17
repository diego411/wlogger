use crate::db::database;
use crate::db::database::Conn as db_conn;
use crate::db::models::{NewChannel, NewMessage};
use rocket_contrib::json::Json;
use serde_json::Value;

#[get("/messages", format = "application/json")]
pub fn index(conn: db_conn) -> Json<Value> {
    let all_messages = database::all(&conn);

    Json(json!({
        "status": 200,
        "result": all_messages,
    }))
}

#[post("/messages", format = "application/json", data = "<new_message>")]
pub fn new(conn: db_conn, new_message: Json<NewMessage>) -> Json<Value> {
    Json(json!({
        "status": database::insert_message(new_message.into_inner(), &conn),
        "result": database::all(&conn).first(),
    }))
}

#[post("/channels", format = "application/json", data = "<new_channel>")]
pub fn new_channel(conn: db_conn, new_channel: Json<NewChannel>) -> Json<Value> {
    Json(json!({
        "status": database::add_channel(new_channel.into_inner(), &conn),
        "result": database::every_channel(&conn).first(),
    }))
}

#[get("/channels", format = "application/json")]
pub fn channel_index(conn: db_conn) -> Json<Value> {
    let all_channels = database::every_channel(&conn);

    Json(json!({
        "status": 200,
        "result": all_channels,
    }))
}
