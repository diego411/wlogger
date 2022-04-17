use crate::schema::messages;

#[derive(Queryable)]
pub struct Message {
    pub id: i32,
    pub content: String,
    pub channel: String,
    pub sender_login: String,
    pub post_timestamp: i32,
}

#[derive(Insertable)]
#[table_name = "messages"]
pub struct NewMessage {
    pub content: String,
    pub channel: String,
    pub sender_login: String,
    pub post_timestamp: i32,
}
