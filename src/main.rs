#[macro_use]
extern crate nickel;

extern crate image;
extern crate qrcode;

use image::Luma;
use qrcode::QrCode;

use nickel::{HttpRouter, Nickel};

fn main() {
    let mut server = Nickel::new();

    server.get(
        "/:message",
        middleware! { |request|
            let message = request.param("message").unwrap();

            let code = QrCode::new(message).unwrap();

            // Render the bits into an image.
            let _image = code.render::<Luma<u8>>().build();

            let string = code.render()
                .light_color(' ')
                .dark_color('#')
                .build();
            println!("{}", string);

            message
        },
    );

    server.listen("127.0.0.1:6767").unwrap();
}
