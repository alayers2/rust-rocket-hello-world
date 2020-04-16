#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use rocket::http::RawStr;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/other")]
fn other() -> &'static str {
    "Hello, other world!"
}

#[get("/<name>")]
fn name(name: &RawStr) -> String {
    format!("Hello, {}!", name.as_str())
}

mod outside {
    #[get("/outside")]
    pub fn world() -> &'static str {
        "Hello, outside world!"
    }
}

fn main() {
    rocket::ignite()
    .mount("/", routes![index])
    .mount("/hello", routes![other, outside::world, name])
    .launch();
}
