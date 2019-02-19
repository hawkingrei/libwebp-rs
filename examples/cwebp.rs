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
    let wp: *mut libwebp_sys::WebPPicture = &mut Default::default();
    let config: *mut libwebp_sys::WebPConfig = &mut Default::default();

    unsafe {
        libwebp_sys::WebPPictureAlloc(wp);

        match state.decode_file(&path) {
            Ok(image) => match image {
                lodepng::Image::RGBA(bitmap) => {
                    let writer: *mut libwebp_sys::WebPMemoryWriter = &mut Default::default();

                    imagers::WebPConfigInit(config);
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
                    libwebp_sys::WebPMemoryWriterInit(writer);
                    (*wp).writer = Some(libwebp_sys::WebPMemoryWrite);
                    (*wp).custom_ptr = writer as *mut libc::c_void;
                    libwebp_sys::WebPEncode(config, wp);

                    let result = Vec::from_raw_parts((*writer).mem, (*writer).size, (*writer).size);
                    fs::write("out.webp", result);
                    //println!("The raw bytes are {:?}", bitmap.buffer.as_bytes());
                }
                x => println!("Decoded some other image format {:?}", x),
            },
            Err(reason) => println!("Could not load {}, because: {}", path.display(), reason),
        }
        libwebp_sys::WebPPictureFree(wp);
    }
}
