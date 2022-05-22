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

use crate::db::models::{Channel, Message, NewChannel, NewMessage, NewUser, User};
use crate::db::schema::channels;
use crate::db::schema::channels::dsl::channels as all_channels;
use crate::db::schema::messages;
use crate::db::schema::messages::dsl::messages as all_messages;
use crate::db::schema::users;
use crate::db::schema::users::dsl::users as all_users;

use crate::controllers::config as ConfigController;

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub async fn init() {
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db_conn =
        PgConnection::establish(&db_url).expect(&format!("Error connecting to {}", db_url));
    run_migrations(&db_conn);
    pull_channels(&db_conn).await;
}

async fn pull_channels(db_conn: &PgConnection) {
    match ConfigController::fetch_config().await {
        Ok(resp) => {
            for channel_name in resp.channels {
                insert_channel(
                    NewChannel {
                        channel_name: channel_name,
                    },
                    db_conn,
                );
            }
        }
        Err(err) => println!(
            "Failed to fetch channel config from config service with error: {}",
            err
        ),
    };
}

fn run_migrations(db_conn: &PgConnection) {
    embed_migrations!();
    embedded_migrations::run(db_conn);
}

pub fn init_pool() -> Pool {
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(&db_url);
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

pub fn every_message(conn: &PgConnection) -> Vec<Message> {
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

pub fn messages_by_user(user_name: String, conn: &PgConnection) -> Vec<Message> {
    all_messages
        .order(messages::id.desc())
        .filter(messages::sender_login.eq(&user_name))
        .load::<Message>(conn)
        .expect(&format!("Error loading messages for user: {}", &user_name))
}

pub fn messages_in_channel(channel_name: String, conn: &PgConnection) -> Vec<Message> {
    all_messages
        .order(messages::id.desc())
        .filter(messages::channel.eq(&channel_name))
        .load::<Message>(conn)
        .expect(&format!(
            "Error loading messages for channel: {}",
            &channel_name
        ))
}

pub fn every_channel(conn: &PgConnection) -> Vec<Channel> {
    all_channels
        .order(channels::channel_name.desc())
        .load::<Channel>(conn)
        .expect("Error loading channels from database")
}

pub fn insert_channel(channel: NewChannel, conn: &PgConnection) -> bool {
    diesel::insert_into(channels::table)
        .values(&channel)
        .execute(conn)
        .is_ok()
}

pub fn every_user(conn: &PgConnection) -> Vec<User> {
    all_users
        .order(users::user_login.desc())
        .load::<User>(conn)
        .expect("Error loading users from database")
}

pub fn insert_user(user: NewUser, conn: &PgConnection) -> bool {
    diesel::insert_into(users::table)
        .values(&user)
        .execute(conn)
        .is_ok()
}
