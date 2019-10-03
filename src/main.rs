#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use std::env;
use rocket::config::{Config, Environment};

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

fn main() {
    let port = match env::var("PORT") {
        Ok(p) => p,
        Err(_) => String::from("3005"),
    };

    let port = match port.parse::<u16>() {
        Ok(p) => p,
        Err(_) => {panic!("port must be a number")},
    };

    let config = match Config::build(Environment::Production)
        .port(port)
        .finalize() {
            Ok(c) => c,
            Err(e) => panic!("error reading config {}", e),
        };

    rocket::custom(config).mount("/", routes![index]).launch();
}