use crate::db::database;
use crate::db::database::Conn as db_conn;
use crate::db::models::NewUser;
use rocket_contrib::json::Json;
use serde_json::Value;

#[get("/users", format = "application/json")]
pub fn user_index(conn: db_conn) -> Json<Value> {
    let all_users = database::every_user(&conn);

    Json(json!({
        "status": 200,
        "result": all_users,
    }))
}

#[get("/users/<user_name>", format = "application/json")]
pub fn user(conn: db_conn, user_name: String) -> Json<Value> {
    let user = match database::user_with_name(user_name.clone(), &conn) {
        Some(user) => user,
        None => {
            return Json(json!({
                "status": 200,
                "user_name": user_name,
                "exists": false
            }))
        }
    };

    let messages_for_user = database::messages_by_user(user_name.clone(), &conn);
    let message_count = messages_for_user.len();

    let mut score = 0;
    for message in &messages_for_user {
        score = score + message.score;
    }

    Json(json!({
        "status": 200,
        "user_name": user_name,
        "exists": true,
        "message_count": message_count,
        "score": score,
        "opted_out": user.opted_out,
        "messages": messages_for_user,
    }))
}

#[patch("/users/<user_name>", format = "application/json", data = "<props>")]
pub fn patch_user(conn: db_conn, user_name: String, props: Json<Value>) -> Json<Value> {
    match props.get("opt_out") {
        Some(opt_out) => match opt_out.as_bool() {
            Some(b) => {
                if b {
                    database::opt_out_user(user_name.clone(), &conn);
                } else {
                    database::opt_in_user(user_name.clone(), &conn);
                }
            }
            None => {
                return Json(json!({
                    "status": 400,
                    "error": "Value for property opt_out should be of boolean"
                }))
            }
        },
        None => (),
    }
    Json(json!({
        "status": 200,
        "user": database::user_with_name(user_name, &conn)
    }))
}

#[post("/users", format = "application/json", data = "<new_user>")]
pub fn new_user(conn: db_conn, new_user: Json<NewUser>) -> Json<Value> {
    Json(json!({
        "status": database::insert_user(new_user.into_inner(), &conn),
        "result": database::every_user(&conn).first(),
    }))
}
