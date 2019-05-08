#[macro_use]
extern crate nickel;

use nickel::{HttpRouter, Nickel};

fn main() {
    let mut server = Nickel::new();

    server.get(
        "/:message",
        middleware! { |request|
            let message = request.param("message").unwrap();

            message
        },
    );

    server.listen("127.0.0.1:6767").unwrap();
}
