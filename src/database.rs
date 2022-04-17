use diesel;
use diesel::prelude::*;
use diesel::PgConnection;

use crate::models::{Message, NewMessage};
use crate::schema::messages;
use crate::schema::messages::dsl::messages as all_messages;

pub struct Database {
    connection: PgConnection,
}

impl Database {
    pub fn new(conn: PgConnection) -> Database {
        Database { connection: conn }
    }

    pub fn all(&self) -> Vec<Message> {
        all_messages
            .order(messages::id.desc())
            .load::<Message>(&self.connection)
            .expect("Error loading messages from database")
    }

    pub fn insert(&self, message: NewMessage) -> bool {
        diesel::insert_into(messages::table)
            .values(&message)
            .execute(&self.connection)
            .is_ok()
    }
}
