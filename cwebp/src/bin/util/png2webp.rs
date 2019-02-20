use imagers::ImageResult;

use std::mem;

use imagers::ImageError;
use lodepng;
use rgb::*;

pub fn png_encode_webp(data: Vec<u8>, resize: Vec<i32>, crop: Vec<i32>) -> ImageResult<Vec<u8>> {
    unsafe {
        let mut state = lodepng::State::new();
        match state.decode(data) {
            Ok(image) => match image {
                lodepng::Image::RGBA(bitmap) => {
                    let mut wp: imagers::WebPPicture = Default::default();
                    let mut config: imagers::WebPConfig = Default::default();
                    config.webp_config_init();

                    wp.set_height(bitmap.height as i32);
                    wp.set_width(bitmap.width as i32);
                    let stride = 4 * bitmap.width * mem::size_of::<u8>();

                    wp.import_rgba(bitmap.buffer.as_bytes().to_vec(), stride as i32);

                    let result = wp.encode(config);
                    return Ok(result.unwrap());
                }
                lodepng::Image::RGB(bitmap) => {
                    let mut wp: imagers::WebPPicture = Default::default();
                    let mut config: imagers::WebPConfig = Default::default();
                    config.webp_config_init();

                    wp.set_height(bitmap.height as i32);
                    wp.set_width(bitmap.width as i32);
                    let stride = 4 * bitmap.width * mem::size_of::<u8>();

                    wp.import_rgba(bitmap.buffer.as_bytes().to_vec(), stride as i32);

                    let result = wp.encode(config);
                    return Ok(result.unwrap());
                }
                _ => return Err(ImageError::FormatError("png format error".to_string())),
            },
            Err(reason) => {
                return Err(ImageError::FormatError(reason.to_string()));
            }
        }
    }
}
