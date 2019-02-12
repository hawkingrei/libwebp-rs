extern crate libwebp_sys;

use rgb::*;
use std::default::Default;
use std::path::Path;

use lodepng;

fn main() {
    let path = Path::new("in.png");
    let mut state = lodepng::State::new();

    match state.decode_file(&path) {
        Ok(image) => match image {
            lodepng::Image::RGBA(bitmap) => {
                println!("Decoded image {} x {}", bitmap.width, bitmap.height);
                println!("The first pixel is {}", bitmap.buffer[0]);
                println!("The raw bytes are {:?}", bitmap.buffer.as_bytes());
            }
            x => println!("Decoded some other image format {:?}", x),
        },
        Err(reason) => println!("Could not load {}, because: {}", path.display(), reason),
    }
    let wp: *mut libwebp_sys::WebPPicture = &mut Default::default();
    unsafe {
        libwebp_sys::WebPPictureAlloc(wp);

        libwebp_sys::WebPPictureFree(wp);
    }
}
