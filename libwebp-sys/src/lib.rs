#![feature(test)]

extern crate test;

use std::path::Path;
use test::Bencher;

use std::default::Default;
use std::mem;

use libc;
use lodepng;
use rgb::*;

include!("./webp_bindings.rs");

pub const WEBP_ENCODER_ABI_VERSION: i32 = 526;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

#[inline(always)]
unsafe fn WebPConfigInit(config: *mut WebPConfig) -> libc::c_int {
    WebPConfigInitInternal(
        config,
        WebPPreset_WEBP_PRESET_DEFAULT,
        75.0 as f32,
        WEBP_ENCODER_ABI_VERSION,
    )
}

#[bench]
fn bench_png_to_webp(b: &mut Bencher) {
    // 1 (inclusive) to 21 (exclusive)
    b.iter(|| {
        let path = Path::new("in.png");
        let mut state = lodepng::State::new();
        let wp: *mut WebPPicture = &mut Default::default();
        let config: *mut WebPConfig = &mut Default::default();
        unsafe {
            WebPPictureAlloc(wp);

            WebPConfigInit(config);

            match state.decode_file(&path) {
                Ok(image) => match image {
                    lodepng::Image::RGBA(bitmap) => {
                        (*wp).height = bitmap.height as i32;
                        (*wp).width = bitmap.width as i32;
                        let stride = 4 * bitmap.width * mem::size_of::<u8>();

                        WebPPictureImportRGBA(wp, bitmap.buffer.as_bytes().as_ptr(), stride as i32);

                        WebPEncode(config, wp);

                        //println!("The raw bytes are {:?}", bitmap.buffer.as_bytes());
                    }
                    x => println!("Decoded some other image format {:?}", x),
                },
                Err(reason) => println!("Could not load {}, because: {}", path.display(), reason),
            }

            WebPPictureFree(wp);
        }
    });
}
