use crate::db::schema::channels;
use crate::db::schema::messages;

#[derive(Serialize, Queryable, Debug, Clone)]
pub struct Message {
    pub id: i32,
    pub content: String,
    pub channel: String,
    pub sender_login: String,
    pub post_timestamp: i32,
}

#[derive(Serialize, Deserialize, Insertable)]
#[table_name = "messages"]
pub struct NewMessage {
    pub content: String,
    pub channel: String,
    pub sender_login: String,
    pub post_timestamp: i32,
}

#[derive(Serialize, Queryable, Debug, Clone)]
pub struct Channel {
    pub id: i32,
    pub channel_name: String,
}

#[derive(Serialize, Deserialize, Insertable)]
#[table_name = "channels"]
pub struct NewChannel {
    pub channel_name: String,
}