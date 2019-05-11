#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

extern crate image;
extern crate qrcode;


use qrcode::QrCode;

use rocket::http::RawStr;

mod qrcode_response;

use qrcode_response::QrCodeResponse;

#[get("/<message>")]
fn message(message: &RawStr) -> QrCodeResponse {
    let code = QrCode::new(message).unwrap();

    QrCodeResponse::new(code)
}

fn main() {
    rocket::ignite().mount("/", routes![message]).launch();
}
