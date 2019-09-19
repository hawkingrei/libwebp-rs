use crate::param::ImageHandler;
use crate::Image;
use crate::ImageError;
use crate::ImageResult;

use std::ptr;

pub fn jpg_encode_webp(data: &[u8], mut p: ImageHandler) -> ImageResult<Image> {
    unsafe {
        let wp: *mut libwebp_sys::WebPPicture = &mut Default::default();
        let config: *mut libwebp_sys::WebPConfig = &mut Default::default();
        let mut image_result: Image = Default::default();

        libwebp_sys::WebPPictureAlloc(wp);

        if p.quality() > 0 {
            libwebp_sys::WebPConfigInitInternal(
                config,
                libwebp_sys::WebPPreset_WEBP_PRESET_DEFAULT,
                p.quality() as f32,
                libwebp_sys::WEBP_ENCODER_ABI_VERSION,
            );
        } else {
            libwebp_sys::WebPConfigInitInternal(
                config,
                libwebp_sys::WebPPreset_WEBP_PRESET_DEFAULT,
                75.0 as f32,
                libwebp_sys::WEBP_ENCODER_ABI_VERSION,
            );
        }

        let decoder_config: *mut libwebp_sys::WebPDecoderConfig = &mut Default::default();
        let output_buffer: *mut libwebp_sys::WebPDecBuffer = &mut Default::default();
        let bitstream: *mut libwebp_sys::WebPBitstreamFeatures = &mut Default::default();

        *output_buffer = (*decoder_config).output;
        *bitstream = (*decoder_config).input;

        if libwebp_sys::ReadJPEG(data.as_ptr(), data.len(), wp, 1, ptr::null_mut()) != 1 {
            return Err(ImageError::FormatError("jpg format error".to_string()));
        }
        image_result.set_height((*wp).height);
        image_result.set_width((*wp).width);

        p.set_height((*wp).height as i32);
        p.set_width((*wp).width as i32);

        let param = p.adapt()?;

        let writer: *mut libwebp_sys::WebPMemoryWriter = &mut Default::default();
        libwebp_sys::WebPMemoryWriterInit(writer);
        (*wp).writer = Some(libwebp_sys::WebPMemoryWrite);
        (*wp).custom_ptr = writer as *mut libc::c_void;
        if let Some(r) = param.resize {
            if r.width != 0 && r.height != 0 {
                if libwebp_sys::WebPPictureRescale(wp, r.width, r.height) != 1 {
                    libwebp_sys::WebPPictureFree(wp);
                    return Err(ImageError::FormatError(
                        "jpg WebPPictureRescale error".to_string(),
                    ));
                }
                image_result.set_height(r.height);
                image_result.set_width(r.width);
            }
        }

        if let Some(c) = param.crop {
            if libwebp_sys::WebPPictureView(wp, c.x, c.y, c.width, c.height, wp) != 1 {
                libwebp_sys::WebPPictureFree(wp);
                return Err(ImageError::FormatError(
                    "jpg WebPPictureView error".to_string(),
                ));
            }
            image_result.set_height(c.height);
            image_result.set_width(c.width);
        }

        if libwebp_sys::WebPEncode(config, wp) == 1 {
            image_result.pic =
                Vec::from_raw_parts((*writer).mem, (*writer).size, (*writer).size).clone();

            libwebp_sys::WebPPictureFree(wp);
            return Ok(image_result);
        }
        libwebp_sys::WebPPictureFree(wp);

        Err(ImageError::FormatError("jpg encode jpg error".to_string()))
    }
}
