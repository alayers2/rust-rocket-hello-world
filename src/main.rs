#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use rocket::Rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

fn build_rocket() -> Rocket {
    rocket::ignite()
    .mount("/", routes![index])
}

fn main() {
    rocket::ignite()
    .mount("/", routes![index])
    .launch();
}


#[cfg(test)]
 mod test {
     use super::build_rocket;
     use rocket::local::Client;
     use rocket::http::{Status};

     #[test]
     fn hello_world() {
         let client = Client::new(build_rocket()).expect("valid rocket instance");
         let mut response = client.get("/").dispatch();

         assert_eq!(response.status(), Status::Ok);
         assert_eq!(response.body_string(), Some("Hello, world!".into()));
     }
 }
