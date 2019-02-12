extern crate libwebp_sys;

use std::default::Default;
use std::mem;
use std::path::Path;

use lodepng;
use rgb::*;

fn main() {
    let path = Path::new("in.png");
    let mut state = lodepng::State::new();
    let wp: *mut libwebp_sys::WebPPicture = &mut Default::default();
    unsafe {
        libwebp_sys::WebPPictureAlloc(wp);
        match state.decode_file(&path) {
            Ok(image) => match image {
                lodepng::Image::RGBA(bitmap) => {
                    (*wp).height = bitmap.height as i32;
                    (*wp).width = bitmap.width as i32;
                    let stride = 4 * bitmap.width * mem::size_of::<u8>();
                    println!("Decoded image {} x {}", bitmap.width, bitmap.height);
                    libwebp_sys::WebPPictureImportRGBA(
                        wp,
                        bitmap.buffer.as_bytes().as_ptr(),
                        stride as i32,
                    );
                    println!("The first pixel is {}", bitmap.buffer[0]);
                    //println!("The raw bytes are {:?}", bitmap.buffer.as_bytes());
                }
                x => println!("Decoded some other image format {:?}", x),
            },
            Err(reason) => println!("Could not load {}, because: {}", path.display(), reason),
        }

        libwebp_sys::WebPPictureFree(wp);
    }
}
