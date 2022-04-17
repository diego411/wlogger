use crate::database;
use crate::database::Conn as db_conn;
use crate::models::{Message, NewMessage};
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
        "status": database::insert(new_message.into_inner(), &conn),
        "result": database::all(&conn).first(),
    }))
}
