use crate::webp::WebPConfig;
use crate::webp::WebPPicture;
use crate::ImageError;
use crate::ImageResult;

use std::mem;

use lodepng;
use rgb::*;

use crate::param::ImageHandler;

pub fn png_encode_webp(data: &Vec<u8>, p: ImageHandler) -> ImageResult<Vec<u8>> {
    unsafe {
        let mut state = lodepng::State::new();
        match state.decode(data) {
            Ok(image) => match image {
                lodepng::Image::RGBA(bitmap) => {
                    let mut wp: WebPPicture = Default::default();
                    let mut config: WebPConfig = Default::default();
                    config.webp_config_init();
                    println!(
                        "input image width: {} height: {}",
                        bitmap.width, bitmap.height
                    );
                    let param = p
                        .set_height(bitmap.height as i32)
                        .set_width(bitmap.width as i32)
                        .adapt()
                        .unwrap();
                    wp.set_height(bitmap.height as i32);
                    wp.set_width(bitmap.width as i32);
                    let stride = 4 * bitmap.width * mem::size_of::<u8>();

                    wp.import_rgba(bitmap.buffer.as_bytes().to_vec(), stride as i32)
                        .unwrap();
                    match param.resize {
                        Some(r) => {
                            println!("resize width: {} height: {}", r.width, r.height);
                            if r.width != 0 && r.height != 0 {
                                wp.rescale(r.width, r.height).unwrap();
                            }
                        }
                        None => {}
                    }
                    match param.crop {
                        Some(c) => {
                            println!(
                                "crop x: {} y: {} width: {} height: {}",
                                c.x, c.y, c.width, c.height
                            );
                            wp.crop(c.x, c.y, c.width, c.height).unwrap();
                        }
                        None => {}
                    }

                    let result = wp.encode(config);
                    return Ok(result.unwrap());
                }
                lodepng::Image::RGB(bitmap) => {
                    let mut wp: WebPPicture = Default::default();
                    let mut config: WebPConfig = Default::default();
                    config.webp_config_init();

                    let param = p
                        .set_height(bitmap.height as i32)
                        .set_width(bitmap.width as i32)
                        .adapt()
                        .unwrap();

                    wp.set_height(bitmap.height as i32);
                    wp.set_width(bitmap.width as i32);
                    let stride = 4 * bitmap.width * mem::size_of::<u8>();

                    wp.import_rgba(bitmap.buffer.as_bytes().to_vec(), stride as i32)
                        .unwrap();
                    match param.resize {
                        Some(r) => {
                            println!("resize width: {} height: {}", r.width, r.height);
                            wp.rescale(r.width, r.height).unwrap();
                        }
                        None => {}
                    }
                    match param.crop {
                        Some(c) => {
                            println!(
                                "crop x: {} y: {} width: {} height: {}",
                                c.x, c.y, c.width, c.height
                            );
                            wp.crop(c.x, c.y, c.width, c.height).unwrap();
                        }
                        None => {}
                    }

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
