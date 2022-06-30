use crate::db::database;
use crate::db::database::Conn as db_conn;
use crate::db::models::NewMessage;
use rocket_contrib::json::Json;
use serde_json::Value;

#[get("/messages", format = "application/json")]
pub fn message_index(conn: db_conn) -> Json<Value> {
    let all_messages = database::every_message(&conn);

    Json(json!({
        "status": 200,
        "result": all_messages,
    }))
}

#[post("/messages", format = "application/json", data = "<new_message>")]
pub fn new_message(conn: db_conn, new_message: Json<NewMessage>) -> Json<Value> {
    Json(json!({
        "status": database::insert_message(new_message.into_inner(), &conn),
        "result": database::every_message(&conn).first(),
    }))
}

#[get("/messages/random?<channel>", format = "application/json")]
pub fn random_message(conn: db_conn, channel: Option<String>) -> Json<Value> {
    match channel {
        Some(channel) => Json(json!({
            "status": 200,
            "result": database::random_message_for_channel(channel, &conn).unwrap()
        })),
        None => Json(json!({
            "status": 200,
            "result": database::random_message(&conn).unwrap()
        })),
    }
}
