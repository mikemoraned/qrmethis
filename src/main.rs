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

struct LimitedMessage<'a>(&'a RawStr);

impl<'r> FromParam<'r> for LimitedMessage<'r> {
    type Error = &'static str;

    fn from_param(param: &'r RawStr) -> Result<Self, Self::Error> {
        if param.len() <= 10 {
            Ok(LimitedMessage(param))
        } else {
            Err("message too long")
        }
    }
}

#[get("/<message>")]
fn message<'r>(message: Result<LimitedMessage<'r>, &'static str>) -> Result<Response<'r>, Status> {
    match message {
        Ok(LimitedMessage(message)) => {
            let qr_code = QrCode::new(message).map_err(|_| Status::BadRequest)?;

            let image = qr_code.render::<Luma<u8>>().build();

            let mut buffer = Vec::new();
            png::PNGEncoder::new(buffer.by_ref())
                .encode(&image, image.width(), image.height(), ColorType::Gray(8))
                .map_err(|_| Status::BadRequest)?;

            Response::build()
                .header(ContentType::PNG)
                .sized_body(Cursor::new(buffer))
                .ok()
        },
        Err(e) => {
            Response::build()
            .status(Status::BadRequest)
            .sized_body(Cursor::new(e))
            .ok()
        }
    }
}

fn main() {
    rocket::ignite().mount("/", routes![message]).launch();
}