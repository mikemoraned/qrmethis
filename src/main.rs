#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate log;

use std::io::Cursor;

use image::{png, ColorType, Luma};
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
                // let image = qr_code
                //     .render::<Luma<u8>>()
                //     .min_dimensions(300, 300)
                //     .build();

                // let mut buffer = Vec::new();
                // png::PNGEncoder::new(buffer.by_ref())
                //     .encode(&image, image.width(), image.height(), ColorType::Gray(8))
                //     .map_err(|e| {
                //         warn!("when creating image: {}", e);
                //         Status::BadRequest
                //     })?;

                // TODO: convert QrCode to colors, and then convert these to GIF indexed
                // colors, then export a frame as a GIF
                // then, starting randomly flipping these indexed colors, either to 0 or 1
                // which represent original colors, or do a random other indexed colots
                // do this N times, and then export these as frames of the GIF animation

                // let mut buffer = Vec::new();
                // let frame =
                //     gif::Frame::from_rgb(image.width() as u16, image.height() as u16, &image);
                // gif::Encoder::new(&mut buffer.by_ref())
                //     .encode(&frame)
                //     .map_err(|e| {
                //         warn!("when creating image: {}", e);
                //         Status::BadRequest
                //     })?;

                Response::build()
                    .header(ContentType::PNG)
                    .sized_body(Cursor::new(buffer))
                    .ok()
                // Response::build()
                //     .header(ContentType::GIF)
                //     .sized_body(Cursor::new(buffer))
                //     .ok()
            }
            Err(_) => bad_request("can't convert message to qr code"),
        },
        Err(e) => bad_request(e),
    }
}

fn main() {
    rocket::ignite().mount("/", routes![message]).launch();
}