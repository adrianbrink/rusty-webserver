extern crate postgres;
extern crate webserver_rust;
#[macro_use]
extern crate nickel;

use std::env;
use webserver_rust::*;
use nickel::{Nickel, HttpRouter};
use postgres::{Connection, TlsMode};

fn main() {
    let mut server = Nickel::new();
    server.get("**", middleware!("Hello World"));
    server.listen("127.0.0.1:8080");
    println!("Hello, world!");

    // let postgres_ip = env::var("POSTGRES_PORT_5432_TCP_ADDR")
    //     .expect("This container needs to be linked with a container running postgres.");
    // let postgres_connection_string = format!("postgres://postgres@{}", postgres_ip);
    // let _conn = Connection::connect(&*postgres_connection_string, TlsMode::None)
    //     .expect("This might work");
}