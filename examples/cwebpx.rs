use std::default::Default;
use std::ffi::CStr;
use std::fs;
use std::mem;
use std::path::Path;

use imagers;
use libc;
use lodepng;
use rgb::*;

fn main() {
    let path = Path::new("in.png");
    let mut state = lodepng::State::new();

    unsafe {
        match state.decode_file(&path) {
            Ok(image) => match image {
                lodepng::Image::RGBA(bitmap) => {
                    let mut wp: imagers::WebPPicture = Default::default();
                    let mut config: imagers::WebPConfig = Default::default();
                    config.WebPConfigInit();
                    wp.set_height(bitmap.height as i32);
                    wp.set_width(bitmap.width as i32);
                    let stride = 4 * bitmap.width * mem::size_of::<u8>();
                    println!("Decoded image {} x {}", bitmap.width, bitmap.height);
                    wp.ImportRGBA(bitmap.buffer.as_bytes().as_ptr(), stride as i32);

                    println!("The first pixel is {}", bitmap.buffer[0]);
                    let result = wp.encode(config);
                    println!("ok");
                    fs::write("out.webp", result);
                    //println!("The raw bytes are {:?}", bitmap.buffer.as_bytes());
                }
                x => println!("Decoded some other image format {:?}", x),
            },
            Err(reason) => println!("Could not load {}, because: {}", path.display(), reason),
        }
    }
}
