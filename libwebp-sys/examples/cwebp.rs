extern crate libwebp_sys;

use std::default::Default;
use std::ffi::CStr;
use std::mem;
use std::path::Path;

use libc;
use lodepng;
use rgb::*;

unsafe extern "C" fn MyViewer(
    data: *const u8,
    data_size: usize,
    picture: *const libwebp_sys::WebPPicture,
) -> libc::c_int {
    let out: *mut libc::FILE = (*picture).custom_ptr as *mut libc::FILE;
    if data_size > 0 {
        libc::fwrite(data as *const libc::c_void, data_size, 1, out) as i32
    } else {
        1
    }
}

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
        let c_file = libc::fopen(
            CStr::from_bytes_with_nul_unchecked(b"out.webp\0").as_ptr(),
            CStr::from_bytes_with_nul_unchecked(b"wb\0").as_ptr(),
        );
        libwebp_sys::WebPPictureAlloc(wp);

        (*wp).writer = Some(MyViewer);
        (*wp).custom_ptr = c_file as *mut libc::c_void;
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
        libc::fclose(c_file);
        libwebp_sys::WebPPictureFree(wp);
    }
}
