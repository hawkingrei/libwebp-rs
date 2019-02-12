extern crate libwebp_sys;

use std::default::Default;

fn main() {
    let wp: *mut libwebp_sys::WebPPicture = &mut Default::default();
    libwebp_sys::WebPPictureAlloc(libwebp_sys);

    libwebp_sys::WebPPictureFree(libwebp_sys);
}
