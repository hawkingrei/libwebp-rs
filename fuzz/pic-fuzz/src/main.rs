#![allow(unused_must_use)]
#[macro_use]
extern crate afl;

use imagers::gif_encode_webp;
use imagers::jpg_encode_webp;
use imagers::png_encode_webp;
use imagers::webp_encode_webp;
use imagers::ImageFormat;
use imagers::ImageHandlerBuilder;
use imagers::Resize;

fn main() {
    fuzz!(|data: &[u8]| {
        let param = ImageHandlerBuilder::new()
            .set_edge(1)
            .set_auto_crop(true)
            .set_crop(None)
            .set_resize(Some(Resize {
                width: 100,
                height: 100,
            }))
            .set_region_crop(None)
            .finish();
        let ptype: ImageFormat;
        match imagers::guess_format(&data.to_vec()) {
            Ok(t) => ptype = t,
            Err(_) => return,
        };

        match ptype {
            ImageFormat::PNG => {
                png_encode_webp(&data.to_vec().clone(), param);
            }
            ImageFormat::JPEG => {
                jpg_encode_webp(&data.to_vec().clone(), param);
            }
            ImageFormat::WEBP => {
                webp_encode_webp(&data.to_vec().clone(), param);
            }
            ImageFormat::GIF => {
                gif_encode_webp(&data.to_vec().clone(), param);
            }
            _ => println!("not support"),
        };
    });
}
