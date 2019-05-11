use std::io::Cursor;


use image::Luma;
use image::{png, ColorType};
use qrcode::QrCode;
use rocket::http::ContentType;
use rocket::request::Request;
use rocket::response::{self, Responder, Response};

use std::io::Write;

pub struct QrCodeResponse {
    qr_code: QrCode,
}

impl QrCodeResponse {
    pub fn new(qr_code: QrCode) -> QrCodeResponse {
        QrCodeResponse { qr_code }
    }
}

impl<'r> Responder<'r> for QrCodeResponse {
    fn respond_to(self, _: &Request) -> response::Result<'r> {
        let image = self.qr_code.render::<Luma<u8>>().build();

        let mut buffer = Vec::new();
        png::PNGEncoder::new(buffer.by_ref())
            .encode(&image, image.width(), image.height(), ColorType::Gray(8))
            .unwrap();

        Response::build()
            .header(ContentType::PNG)
            .sized_body(Cursor::new(buffer))
            .ok()
    }
}