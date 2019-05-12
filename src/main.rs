#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

extern crate image;
extern crate qrcode;

mod graphql_handlers;
mod graphql_schema;
mod qrcode_response;

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
        .manage(graphql_schema::Model::new())
        .manage(graphql_schema::Schema::new(
            graphql_schema::QueryRoot {},
            juniper::EmptyMutation::<graphql_schema::Model>::new(),
        ))
        .mount("/", routes![message])
        .mount(
            "/",
            routes![
                graphql_handlers::graphiql,
                graphql_handlers::get_graphql_handler,
                graphql_handlers::post_graphql_handler,
            ],
        )
        .launch();
}
