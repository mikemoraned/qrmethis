#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

extern crate image;
extern crate qrcode;

// mod qrcode_response;
// mod graphql_handlers;
// mod graphql_schema;

use qrcode::QrCode;

use rocket::http::RawStr;

use qrcode_response::QrCodeResponse;

#[get("/<message>")]
fn message(message: &RawStr) -> QrCodeResponse {
    let code = QrCode::new(message).unwrap();

    QrCodeResponse::new(code)
}

fn main() {
    rocket::ignite()
        .mount("/", routes![message])
        .mount(
            "/",
            routes![graphiql, get_graphql_handler, post_graphql_handler],
        )
        .launch();
}
