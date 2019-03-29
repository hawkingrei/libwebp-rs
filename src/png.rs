use crate::webp::WebPConfig;
use crate::webp::WebPPicture;
use crate::ImageError;
use crate::ImageResult;

use std::mem;

use lodepng;
use lodepng::Decoder;
use rgb::*;

use crate::webp::webp_config_init;

use crate::param::ImageHandler;

pub fn png_encode_webp(data: &Vec<u8>, p: ImageHandler) -> ImageResult<Vec<u8>> {
    unsafe {
        let mut state = lodepng::Decoder::new();
        match state.decode(data) {
            Ok(image) => match image {
                lodepng::Image::RGBA(bitmap) => {
                    let wp: *mut libwebp_sys::WebPPicture = &mut Default::default();
                    let config: *mut libwebp_sys::WebPConfig = &mut Default::default();
                    libwebp_sys::WebPPictureAlloc(wp);

                    libwebp_sys::WebPConfigInitInternal(
                        config,
                        libwebp_sys::WebPPreset_WEBP_PRESET_DEFAULT,
                        75.0 as f32,
                        libwebp_sys::WEBP_ENCODER_ABI_VERSION,
                    );

                    let decoder_config: *mut libwebp_sys::WebPDecoderConfig =
                        &mut Default::default();
                    let output_buffer: *mut libwebp_sys::WebPDecBuffer = &mut Default::default();
                    let bitstream: *mut libwebp_sys::WebPBitstreamFeatures =
                        &mut Default::default();

                    *output_buffer = (*decoder_config).output;
                    *bitstream = (*decoder_config).input;

                    let param = p
                        .set_height(bitmap.height as i32)
                        .set_width(bitmap.width as i32)
                        .adapt()
                        .unwrap();
                    (*wp).height = bitmap.height as i32;
                    (*wp).width = bitmap.width as i32;
                    let stride = 4 * bitmap.width * mem::size_of::<u8>();

                    println!("height {:?}", (*wp).height);
                    println!("width {:?}", (*wp).width);
                    println!("{:?}", bitmap.buffer.as_bytes().to_vec().len());
                    if libwebp_sys::WebPPictureImportRGBA(
                        wp,
                        bitmap.buffer.as_bytes().to_vec().as_ptr(),
                        stride as i32,
                    ) != 1
                    {
                        return Err(ImageError::FormatError(
                            "png WebPPictureImportRGBA error".to_string(),
                        ));
                    }

                    let writer: *mut libwebp_sys::WebPMemoryWriter = &mut Default::default();
                    libwebp_sys::WebPMemoryWriterInit(writer);
                    (*wp).writer = Some(libwebp_sys::WebPMemoryWrite);
                    (*wp).custom_ptr = writer as *mut libc::c_void;
                    match param.resize {
                        Some(r) => {
                            if r.width != 0 && r.height != 0 {
                                libwebp_sys::WebPPictureRescale(wp, r.width, r.height);
                            }
                        }
                        None => {}
                    }
                    match param.crop {
                        Some(c) => {
                            libwebp_sys::WebPPictureView(wp, c.x, c.y, c.width, c.height, wp);
                        }
                        None => {}
                    }

                    if libwebp_sys::WebPEncode(config, wp) == 1 {
                        return Ok(Vec::from_raw_parts(
                            (*writer).mem,
                            (*writer).size,
                            (*writer).size,
                        ));
                    }
                    return Err(ImageError::FormatError("png format error".to_string()));
                }
                lodepng::Image::RGB(bitmap) => {
                    let wp: *mut libwebp_sys::WebPPicture = &mut Default::default();
                    let config: *mut libwebp_sys::WebPConfig = &mut Default::default();
                    libwebp_sys::WebPPictureAlloc(wp);

                    libwebp_sys::WebPConfigInitInternal(
                        config,
                        libwebp_sys::WebPPreset_WEBP_PRESET_DEFAULT,
                        75.0 as f32,
                        libwebp_sys::WEBP_ENCODER_ABI_VERSION,
                    );

                    let decoder_config: *mut libwebp_sys::WebPDecoderConfig =
                        &mut Default::default();
                    let output_buffer: *mut libwebp_sys::WebPDecBuffer = &mut Default::default();
                    let bitstream: *mut libwebp_sys::WebPBitstreamFeatures =
                        &mut Default::default();

                    *output_buffer = (*decoder_config).output;
                    *bitstream = (*decoder_config).input;

                    let param = p
                        .set_height(bitmap.height as i32)
                        .set_width(bitmap.width as i32)
                        .adapt()
                        .unwrap();

                    (*wp).height = (*bitstream).height as i32;
                    (*wp).width = (*bitstream).width as i32;
                    let stride = 3 * bitmap.width * mem::size_of::<u8>();

                    libwebp_sys::WebPPictureImportRGB(
                        wp,
                        bitmap.buffer.as_bytes().to_vec().as_ptr(),
                        stride as i32,
                    );

                    let writer: *mut libwebp_sys::WebPMemoryWriter = &mut Default::default();
                    libwebp_sys::WebPMemoryWriterInit(writer);
                    (*wp).writer = Some(libwebp_sys::WebPMemoryWrite);
                    (*wp).custom_ptr = writer as *mut libc::c_void;
                    match param.resize {
                        Some(r) => {
                            if r.width != 0 && r.height != 0 {
                                libwebp_sys::WebPPictureRescale(wp, r.width, r.height);
                            }
                        }
                        None => {}
                    }
                    match param.crop {
                        Some(c) => {
                            libwebp_sys::WebPPictureView(wp, c.x, c.y, c.width, c.height, wp);
                        }
                        None => {}
                    }

                    if libwebp_sys::WebPEncode(config, wp) == 1 {
                        return Ok(Vec::from_raw_parts(
                            (*writer).mem,
                            (*writer).size,
                            (*writer).size,
                        ));
                    }
                    return Err(ImageError::FormatError("png format error 1".to_string()));
                }
                _ => return Err(ImageError::FormatError("png format error 2".to_string())),
            },
            Err(reason) => {
                return Err(ImageError::FormatError(reason.to_string()));
            }
        }
    }
}
