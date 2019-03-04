use crate::util::param::ImageHandler;

use imagers::ImageError;
use imagers::ImageResult;
use libwebp_sys;

use std::mem;
use std::ptr;
use std::slice;

pub fn webp_encode_webp(data: &Vec<u8>, p: ImageHandler) -> ImageResult<Vec<u8>> {
    unsafe {
        let wp: *mut libwebp_sys::WebPPicture = &mut Default::default();
        let config: *mut libwebp_sys::WebPConfig = &mut Default::default();

        libwebp_sys::WebPPictureAlloc(wp);

        libwebp_sys::WebPConfigInitInternal(
            config,
            libwebp_sys::WebPPreset_WEBP_PRESET_DEFAULT,
            75.0 as f32,
            libwebp_sys::WEBP_ENCODER_ABI_VERSION,
        );

        let mut decoder_config: *mut libwebp_sys::WebPDecoderConfig = &mut Default::default();
        let mut output_buffer: *mut libwebp_sys::WebPDecBuffer = &mut Default::default();
        let mut bitstream: *mut libwebp_sys::WebPBitstreamFeatures = &mut Default::default();

        *output_buffer = (*decoder_config).output;
        *bitstream = (*decoder_config).input;
        libwebp_sys::WebPInitDecoderConfigInternal(
            decoder_config,
            libwebp_sys::WEBP_ENCODER_ABI_VERSION,
        );
        let mut status: libwebp_sys::VP8StatusCode = libwebp_sys::VP8StatusCode_VP8_STATUS_OK;
        status = libwebp_sys::WebPGetFeaturesInternal(
            data.as_ptr(),
            data.len(),
            bitstream,
            libwebp_sys::WEBP_ENCODER_ABI_VERSION,
        );
        if (status != libwebp_sys::VP8StatusCode_VP8_STATUS_OK) {
            return Err(ImageError::FormatError("png format error".to_string()));
        }
        (*wp).use_argb = (*bitstream).has_alpha;
        (*wp).height = (*bitstream).height as i32;
        (*wp).width = (*bitstream).width as i32;
        let param = p
            .set_height((*wp).height)
            .set_width((*wp).width)
            .adapt()
            .unwrap();
        if (*wp).use_argb == 1 {
            let d = libwebp_sys::WebPDecodeRGBA(
                data.as_ptr(),
                data.len(),
                &mut (*wp).width,
                &mut (*wp).height,
            );
            let stride = 4 * (*wp).width * mem::size_of::<u8>() as i32;
            libwebp_sys::WebPPictureImportRGBA(wp, d, stride);
        } else {
            let d = libwebp_sys::WebPDecodeRGB(
                data.as_ptr(),
                data.len(),
                &mut (*wp).width,
                &mut (*wp).height,
            );
            let stride = 3 * (*wp).width * mem::size_of::<u8>() as i32;
            libwebp_sys::WebPPictureImportRGB(wp, d, stride);
        }

        let writer: *mut libwebp_sys::WebPMemoryWriter = &mut Default::default();
        libwebp_sys::WebPMemoryWriterInit(writer);
        (*wp).writer = Some(libwebp_sys::WebPMemoryWrite);
        (*wp).custom_ptr = writer as *mut libc::c_void;
        match param.resize {
            Some(r) => {
                println!("resize width: {} height: {}", r.width, r.height);
                libwebp_sys::WebPPictureRescale(wp, r.width, r.height);
            }
            None => {}
        }

        match param.crop {
            Some(c) => {
                println!(
                    "crop x: {} y: {} width: {} height: {}",
                    c.x, c.y, c.width, c.height
                );
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
}
