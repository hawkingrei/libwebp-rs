extern crate libwebp_sys;

use std::default::Default;
use std::mem;
use std::path::Path;

use lodepng;
use rgb::*;

#[inline(always)]
unsafe fn WebPConfigInit(config: *mut libwebp_sys::WebPConfig) -> libc::c_int {
    libwebp_sys::WebPConfigInitInternal(
        config,
        libwebp_sys::WebPPreset_WEBP_PRESET_DEFAULT,
        75.0 as f32,
        libwebp_sys::WEBP_ENCODER_ABI_VERSION,
    )
}

fn main() {
    let path = Path::new("in.png");
    let mut state = lodepng::State::new();
    let wp: *mut libwebp_sys::WebPPicture = &mut Default::default();
    let config: *mut libwebp_sys::WebPConfig = &mut Default::default();
    unsafe {
        libwebp_sys::WebPPictureAlloc(wp);
        WebPConfigInit(config);
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
                    libwebp_sys::WebPEncode(config, wp);
                    //println!("The raw bytes are {:?}", bitmap.buffer.as_bytes());
                }
                x => println!("Decoded some other image format {:?}", x),
            },
            Err(reason) => println!("Could not load {}, because: {}", path.display(), reason),
        }

        libwebp_sys::WebPPictureFree(wp);
    }
}
