use diesel;
use diesel::prelude::*;
use diesel::PgConnection;

use std::env;
use std::ops::Deref;

use r2d2;
use r2d2_diesel::ConnectionManager;
use rocket::http::Status;
use rocket::request::{self, FromRequest};
use rocket::{Outcome, Request, State};

use crate::db::models::{Channel, Message, NewChannel, NewMessage};
use crate::db::schema::channels;
use crate::db::schema::channels::dsl::channels as all_channels;
use crate::db::schema::messages;
use crate::db::schema::messages::dsl::messages as all_messages;

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn init_pool() -> Pool {
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(db_url);
    r2d2::Pool::new(manager).expect("db pool failure")
}

pub struct Conn(pub r2d2::PooledConnection<ConnectionManager<PgConnection>>);

impl<'a, 'r> FromRequest<'a, 'r> for Conn {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Conn, ()> {
        let pool = request.guard::<State<Pool>>()?;
        match pool.get() {
            Ok(conn) => Outcome::Success(Conn(conn)),
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ())),
        }
    }
}

impl Deref for Conn {
    type Target = PgConnection;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub fn all(conn: &PgConnection) -> Vec<Message> {
    all_messages
        .order(messages::id.desc())
        .load::<Message>(conn)
        .expect("Error loading messages from database")
}

pub fn insert_message(message: NewMessage, conn: &PgConnection) -> bool {
    diesel::insert_into(messages::table)
        .values(&message)
        .execute(conn)
        .is_ok()
}

pub fn every_channel(conn: &PgConnection) -> Vec<Channel> {
    all_channels
        .order(channels::id.desc())
        .load::<Channel>(conn)
        .expect("Error loading channels from database")
}

pub fn add_channel(channel: NewChannel, conn: &PgConnection) -> bool {
    diesel::insert_into(channels::table)
        .values(&channel)
        .execute(conn)
        .is_ok()
}
