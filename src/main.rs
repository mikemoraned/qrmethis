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
use rocket::request::FromParam;
use rocket::Response;
use std::io::Write;

struct Message<'a>(&'a RawStr);

impl<'r> FromParam<'r> for Message<'r> {
    type Error = &'static str;

    fn from_param(param: &'r RawStr) -> Result<Self, Self::Error> {
        if param.len() <= 10 {
            Ok(Message(param))
        } else {
            Err("message too long")
        }
    }
}

fn bad_request<T>(reason: &str) -> Result<Response, T> {
    Response::build()
        .status(Status::BadRequest)
        .sized_body(Cursor::new(reason))
        .ok()
}

#[get("/<message>")]
fn message<'r>(message: Result<Message<'r>, &'static str>) -> Result<Response<'r>, Status> {
    match message {
        Ok(Message(message)) => {
            let qr_code = QrCode::new(message).map_err(|e| bad_request("can't convert message to qr code"))?;

            let image = qr_code.render::<Luma<u8>>().build();

            let mut buffer = Vec::new();
            png::PNGEncoder::new(buffer.by_ref())
                .encode(&image, image.width(), image.height(), ColorType::Gray(8))
                .map_err(|e| bad_request("can't create image"))?;

            Response::build()
                .header(ContentType::PNG)
                .sized_body(Cursor::new(buffer))
                .ok()
        }
        Err(e) => bad_request(e),
    }
}

fn main() {
    rocket::ignite().mount("/", routes![message]).launch();
}