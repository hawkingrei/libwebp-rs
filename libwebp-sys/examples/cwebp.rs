extern crate libwebp_sys as webp;

use webp*;

use std::Default;

fn main() {
    let wp: *mut webp::WebPPicture = Default::default();
}
