extern crate iron;
extern crate logger;
extern crate env_logger;

use iron::prelude::*;
use iron::status;
use logger::Logger;
use logger::Format;

static FORMAT: &'static str = "Uri: {uri}, Method: {method}, Status: {status}, Duration: \
                               {response-time}, Time: {request-time}";

fn hello_world_handler(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, "Hello World!")))
}

fn main() {
    println!("Running service");

    env_logger::init().unwrap();

    let format = Format::new(FORMAT);
    let mut chain = Chain::new(hello_world_handler);

    chain.link(Logger::new(Some(format.unwrap())));

    match Iron::new(chain).http("127.0.0.1:4000") {
        Result::Ok(listening) => println!("{:?}", listening),
        Result::Err(err) => panic!("{:?}", err),
    }
}
