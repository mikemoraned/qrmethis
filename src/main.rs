#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use std::io::Cursor;

use image::Luma;
use image::{png, ColorType};
use qrcode::QrCode;


use rocket::http::ContentType;
use rocket::http::RawStr;
use rocket::http::Status;
use rocket::response::Response;
use std::io::Write;

#[get("/<message>")]
fn message(message: &RawStr) -> Result<Response, Status> {
    let qr_code = QrCode::new(message).unwrap();

    let image = qr_code.render::<Luma<u8>>().build();

    let mut buffer = Vec::new();
    png::PNGEncoder::new(buffer.by_ref())
        .encode(&image, image.width(), image.height(), ColorType::Gray(8))
        .map_err(|_| Status::BadRequest)?;

    Response::build()
        .header(ContentType::PNG)
        .sized_body(Cursor::new(buffer))
        .ok()
}

fn main() {
    rocket::ignite().mount("/", routes![message]).launch();
}