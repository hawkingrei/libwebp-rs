#[macro_use]
extern crate afl;
extern crate http;
extern crate imagers;


use imagers::ImageFormat;
use imagers::gif_encode_webp;
use imagers::gif_info;
use imagers::jpg_encode_webp;
use imagers::png_encode_webp;
use imagers::webp_encode_webp;
use imagers::Crop;
use imagers::ImageHandlerBuilder;
use imagers::RegionCrop;
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
        let ptype = imagers::guess_format(&data.to_vec()).unwrap();
        loop {
            match ptype {
                ImageFormat::PNG => {
                    png_encode_webp(&data.to_vec().clone(), param).unwrap();
                }
                ImageFormat::JPEG => {
                    jpg_encode_webp(&data.to_vec().clone(), param).unwrap();
                }
                ImageFormat::WEBP => {
                    webp_encode_webp(&data.to_vec().clone(), param).unwrap();
                }
                ImageFormat::GIF => {
                    gif_encode_webp(&mut data.to_vec().clone(), param).unwrap();
                }
                _ => println!("not support"),
            }
        }
    });
}
