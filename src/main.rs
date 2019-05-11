#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

extern crate image;
extern crate qrcode;

//use image::{png, ColorType};
// use image::Luma;
use qrcode::QrCode;

use rocket::http::RawStr;

mod qrcode_response;

use qrcode_response::QrCodeResponse;

#[get("/<message>")]
fn message(message: &RawStr) -> QrCodeResponse {
    let code = QrCode::new(message).unwrap();

    //    // Render the bits into an image.
    //    let image = code.render::<Luma<u8>>().build();
    //
    //    let bytes = vec![];
    //    png::PNGEncoder::new(bytes).encode(&image, image.width(), image.height(), ColorType::Gray(1)).unwrap();

    //    bytes
    QrCodeResponse { qr_code: code }
}

fn main() {
    rocket::ignite().mount("/", routes![message]).launch();
}
