#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate log;

use std::io::Cursor;


use image::Rgb;
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
        Ok(Message(message)) => match QrCode::new(message) {
            Ok(qr_code) => {
                let image = qr_code
                    .render()
                    .dark_color(Rgb { data: [0, 0, 0] })
                    .light_color(Rgb {
                        data: [254, 254, 254],
                    })
                    .min_dimensions(300, 300)
                    .build();

                let mut buffer = Vec::new();
                png::PNGEncoder::new(buffer.by_ref())
                    .encode(&image, image.width(), image.height(), ColorType::RGB(8))
                    .map_err(|e| {
                        warn!("when creating image: {}", e);
                        Status::BadRequest
                    })?;

                Response::build()
                    .header(ContentType::PNG)
                    .sized_body(Cursor::new(buffer))
                    .ok()
            }
            Err(_) => bad_request("can't convert message to qr code"),
        },
        Err(e) => bad_request(e),
    }
}

fn main() {
    rocket::ignite().mount("/", routes![message]).launch();
}