use crate::db::schema::channels;
use crate::db::schema::messages;
use crate::db::schema::users;

#[derive(Serialize, Queryable, Debug, Clone)]
pub struct Message {
    pub id: i32,
    pub content: String,
    pub channel: String,
    pub sender_login: String,
    pub post_timestamp: i32,
    pub score: i32,
}

#[derive(Serialize, Deserialize, Insertable)]
#[table_name = "messages"]
pub struct NewMessage {
    pub content: String,
    pub channel: String,
    pub sender_login: String,
    pub post_timestamp: i32,
    pub score: i32,
}

#[derive(Serialize, Queryable, Debug, Clone)]
pub struct Channel {
    pub channel_name: String,
    pub actively_logged: bool,
}

#[derive(Serialize, Deserialize, Insertable)]
#[table_name = "channels"]
pub struct NewChannel {
    pub channel_name: String,
    pub actively_logged: bool,
}

#[derive(Serialize, Queryable, Debug, Clone)]
pub struct User {
    pub user_login: String,
    pub opted_out: bool,
}

#[derive(Serialize, Deserialize, Insertable)]
#[table_name = "users"]
pub struct NewUser {
    pub user_login: String,
    pub opted_out: bool,
}

#[derive(Serialize, Queryable, Debug, Clone)]
pub struct UserWithScore {
    pub user_login: String,
    pub score: i64,
}
