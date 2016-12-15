#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate sendgrid;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;
use sendgrid::v3::V3Sender;

pub fn establish_postgres_connection() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set.");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

pub struct Mailer {
    sender: V3Sender,
}

impl Mailer {
    pub fn new(api_key: String) -> Mailer {
        let sender = V3Sender::new(api_key);
        Mailer { sender: sender }
    }
}

#[test]
fn it_works() {}
