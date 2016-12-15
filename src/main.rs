extern crate postgres;
extern crate iron;
#[macro_use]
extern crate nickel;

use iron::prelude::*;
use iron::status;
use std::env;
use nickel::{Nickel, HttpRouter};
use postgres::{Connection, TlsMode};

fn main() {
    // let client = twilio::Client::new("AC48b3955fdf390d1ee7fe52efcabbfb2d", "dcd49ce427035532f62585eeb7c63af0");
    let mut server = Nickel::new();
    server.get("**", middleware!("Hello World"));
    server.listen("127.0.0.1:4443");
    println!("Hello, world!");
    // let _server = Iron::new(hello_world).http("localhost:4040").expect("Server didn't start.");
    // let postgres_ip = env::var("POSTGRES_PORT_5432_TCP_ADDR")
    //     .expect("This container needs to be linked with a container running postgres.");
    // let postgres_connection_string = format!("postgres://postgres@{}", postgres_ip);
    // let _conn = Connection::connect(&*postgres_connection_string, TlsMode::None)
    //     .expect("This might work");

    // Iron::new(|_: &mut Request| Ok(Response::with((status::Ok, "Hello World!"))))
    //     .http("localhost:4040")
    //     .unwrap();
}

fn hello_world(_: &mut Request) -> IronResult<Response> {
    println!("dddd");
    Ok(Response::with((status::Ok, "Hello World!")))
}