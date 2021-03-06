use diesel;
use diesel::dsl::sum;
use diesel::prelude::*;
use diesel::sql_types::Text;
use diesel::PgConnection;

use std::env;
use std::ops::Deref;

use rand::Rng;

use r2d2;
use r2d2_diesel::ConnectionManager;
use rocket::http::Status;
use rocket::request::{self, FromRequest};
use rocket::{Outcome, Request, State};

use crate::db::models::{Channel, Message, NewChannel, NewMessage, NewUser, User, UserWithScore};
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
                        actively_logged: true,
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
    embedded_migrations::run(db_conn).unwrap();
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

sql_function!(fn lower(x: Text) -> Text);

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
        .filter(lower(messages::sender_login).eq(&user_name.to_lowercase()))
        .load::<Message>(conn)
        .expect(&format!("Error loading messages for user: {}", &user_name))
}

pub fn messages_in_channel(channel_name: String, conn: &PgConnection) -> Vec<Message> {
    all_messages
        .order(messages::id.desc())
        .filter(lower(messages::channel).eq(&channel_name.to_lowercase()))
        .load::<Message>(conn)
        .expect(&format!(
            "Error loading messages for channel: {}",
            &channel_name
        ))
}

pub fn random_message(conn: &PgConnection) -> Option<Message> {
    let count = all_messages
        .count()
        .get_result(conn)
        .expect("Error counting messages");
    let rand = rand::thread_rng().gen_range(0..count);
    print!("{}", rand);
    match all_messages
        .order(messages::id.desc())
        .offset(rand)
        .load::<Message>(conn)
        .expect("Error loading messages from database")
        .first()
    {
        Some(message) => Some(message.to_owned()),
        None => None,
    }
}

pub fn random_message_for_channel(channel_name: String, conn: &PgConnection) -> Option<Message> {
    let count = all_messages
        .order_by(messages::id)
        .filter(lower(messages::channel).eq(&channel_name.to_lowercase()))
        .count()
        .group_by(messages::id)
        .get_result(conn)
        .expect(&format!(
            "Error counting messages for channel: {}",
            &channel_name
        ));
    let rand = rand::thread_rng().gen_range(0..count);
    print!("{}", rand);
    match all_messages
        .order(messages::id.desc())
        .filter(lower(messages::channel).eq(&channel_name.to_lowercase()))
        .offset(rand)
        .load::<Message>(conn)
        .expect("Error loading messages from database")
        .first()
    {
        Some(message) => Some(message.to_owned()),
        None => None,
    }
}

pub fn every_channel(conn: &PgConnection) -> Vec<Channel> {
    all_channels
        .order(channels::channel_name.desc())
        .load::<Channel>(conn)
        .expect("Error loading channels from database")
}

pub fn channel_with_name(channel_name: String, conn: &PgConnection) -> Option<Channel> {
    match all_channels
        .order(channels::channel_name.desc())
        .filter(lower(channels::channel_name).eq(&channel_name.to_lowercase()))
        .load::<Channel>(conn)
        .expect("Error loading channels from database")
        .first()
    {
        Some(channel) => Some(channel.to_owned()),
        None => None,
    }
}

pub fn insert_channel(channel: NewChannel, conn: &PgConnection) -> bool {
    diesel::insert_into(channels::table)
        .values(&channel)
        .execute(conn)
        .is_ok()
}

pub fn is_channel_actively_logged(channel_name: String, conn: &PgConnection) -> bool {
    all_channels
        .order(channels::channel_name.desc())
        .filter(lower(channels::channel_name).eq(&channel_name.to_lowercase()))
        .load::<Channel>(conn)
        .expect(&format!(
            "Error loading property for channel {}",
            channel_name
        ))
        .first()
        .expect(&format!(
            "Error loading property for channel {}",
            channel_name
        ))
        .actively_logged
}

pub fn every_user(conn: &PgConnection) -> Vec<User> {
    all_users
        .order(users::user_login.desc())
        .load::<User>(conn)
        .expect("Error loading users from database")
}

pub fn user_with_name(user_name: String, conn: &PgConnection) -> Option<User> {
    match all_users
        .order(users::user_login.desc())
        .filter(lower(users::user_login).eq(&user_name.to_lowercase()))
        .load::<User>(conn)
        .expect("Error loading users from database")
        .first()
    {
        Some(user) => Some(user.to_owned()),
        None => None,
    }
}

pub fn top_users_by_score(size: usize, conn: &PgConnection) -> Vec<UserWithScore> {
    let top_users = all_users
        .inner_join(all_messages.on(users::user_login.like(messages::sender_login)))
        .select((
            users::user_login,
            diesel::dsl::sql::<diesel::sql_types::BigInt>("sum(score)"),
        ))
        .group_by(users::user_login)
        .order_by(sum(messages::score).desc())
        .load::<UserWithScore>(conn)
        .expect("Error loading users from database");

    if top_users.len() <= size {
        return top_users;
    }

    top_users[0..size].to_vec()
}

pub fn opt_out_user(user_name: String, conn: &PgConnection) -> bool {
    diesel::update(users::table)
        .filter(lower(users::user_login).eq(&user_name.to_lowercase()))
        .set(users::opted_out.eq(true))
        .execute(conn)
        .is_ok()
}

pub fn opt_in_user(user_name: String, conn: &PgConnection) -> bool {
    diesel::update(users::table)
        .filter(lower(users::user_login).eq(&user_name.to_lowercase()))
        .set(users::opted_out.eq(false))
        .execute(conn)
        .is_ok()
}

pub fn insert_user(user: NewUser, conn: &PgConnection) -> bool {
    diesel::insert_into(users::table)
        .values(&user)
        .execute(conn)
        .is_ok()
}
