#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use rocket::http::RawStr;

#[get("/<message>")]
fn message(message: &RawStr) -> String {
    format!("{}", message.as_str())
}

fn main() {
    rocket::ignite().mount("/", routes![message]).launch();
}
