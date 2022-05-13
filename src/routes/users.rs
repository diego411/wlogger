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

#[post("/users", format = "application/json", data = "<new_user>")]
pub fn new_user(conn: db_conn, new_user: Json<NewUser>) -> Json<Value> {
    Json(json!({
        "status": database::insert_user(new_user.into_inner(), &conn),
        "result": database::every_user(&conn).first(),
    }))
}
