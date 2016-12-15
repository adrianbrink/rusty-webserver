extern crate postgres;

use std::env;
use postgres::{Connection, TlsMode};

fn main() {
    println!("Hello, world!");
    let postgres_ip = env::var("POSTGRES_PORT_5432_TCP_ADDR")
        .expect("This container needs to be linked with a container running postgres.");
    let postgres_connection_string = format!("postgres://postgres@{}", postgres_ip);
    let conn = Connection::connect(&*postgres_connection_string, TlsMode::None)
        .expect("This might work");
}