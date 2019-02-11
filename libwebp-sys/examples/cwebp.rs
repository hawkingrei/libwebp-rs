extern crate libwebp_sys;


use std::default::Default;

fn main() {
    let wp: *mut libwebp_sys::WebPPicture = &mut Default::default();
}
