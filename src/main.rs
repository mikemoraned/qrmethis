#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

extern crate image;
extern crate qrcode;

use image::ImageBuffer;
use image::Luma;
use qrcode::QrCode;

use rocket::http::RawStr;

#[get("/<message>")]
fn message(message: &RawStr) -> ImageBuffer {
    let code = QrCode::new(message).unwrap();

    // Render the bits into an image.
    let image = code.render::<Luma<u8>>().build();

    let string = code.render().light_color(' ').dark_color('#').build();
    println!("{}", string);

    // format!("{}", message.as_str())
    image
}

fn main() {
    rocket::ignite().mount("/", routes![message]).launch();
}
