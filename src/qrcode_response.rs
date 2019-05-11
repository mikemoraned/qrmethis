use std::io::Cursor;

use qrcode::QrCode;
use rocket::http::ContentType;
use rocket::request::Request;
use rocket::response::{self, Responder, Response};


pub struct QrCodeResponse {
    pub qr_code: QrCode,
}

impl<'r> Responder<'r> for QrCodeResponse {
    fn respond_to(self, _: &Request) -> response::Result<'r> {
        Response::build()
            .sized_body(Cursor::new("Snarp"))
            .raw_header("X-Person-Name", "Blah")
            .raw_header("X-Person-Age", "Foop")
            .header(ContentType::new("application", "x-person"))
            .ok()
    }
}