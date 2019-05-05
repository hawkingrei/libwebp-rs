use crate::param::ImageHandler;
use crate::webp::WebPConfig;
use crate::webp::WebPPicture;
use crate::Image;
use crate::ImageError;
use crate::ImageResult;
use libjpeg_turbo_sys;

use crate::webp::webp_config_init;

pub fn jpg_encode_webp(data: &Vec<u8>, mut p: ImageHandler) -> ImageResult<Image> {
    unsafe {
        let wp: *mut libwebp_sys::WebPPicture = &mut Default::default();
        let config: *mut libwebp_sys::WebPConfig = &mut Default::default();
        let mut image_result: Image = Default::default();

        libwebp_sys::WebPPictureAlloc(wp);

        libwebp_sys::WebPConfigInitInternal(
            config,
            libwebp_sys::WebPPreset_WEBP_PRESET_DEFAULT,
            75.0 as f32,
            libwebp_sys::WEBP_ENCODER_ABI_VERSION,
        );

        let decoder_config: *mut libwebp_sys::WebPDecoderConfig = &mut Default::default();
        let output_buffer: *mut libwebp_sys::WebPDecBuffer = &mut Default::default();
        let bitstream: *mut libwebp_sys::WebPBitstreamFeatures = &mut Default::default();

        *output_buffer = (*decoder_config).output;
        *bitstream = (*decoder_config).input;

        let mut metadata: *mut libwebp_sys::Metadata = &mut Default::default();
        libwebp_sys::MetadataInit(metadata);
        if libwebp_sys::ReadJPEG(data.as_ptr(), data.len(), wp, 1, metadata) != 1 {
            return Err(ImageError::FormatError("jpg format error".to_string()));
        }
        image_result.set_height((*wp).height);
        image_result.set_width((*wp).width);

        p.set_height((*wp).height as i32);
        p.set_width((*wp).width as i32);

        let param = p.adapt().unwrap();

        let writer: *mut libwebp_sys::WebPMemoryWriter = &mut Default::default();
        libwebp_sys::WebPMemoryWriterInit(writer);
        (*wp).writer = Some(libwebp_sys::WebPMemoryWrite);
        (*wp).custom_ptr = writer as *mut libc::c_void;
        match param.resize {
            Some(r) => {
                if r.width != 0 && r.height != 0 {
                    if libwebp_sys::WebPPictureRescale(wp, r.width, r.height) != 1 {
                        return Err(ImageError::FormatError(
                            "jpg WebPPictureRescale error".to_string(),
                        ));
                    }
                    image_result.set_height(r.height);
                    image_result.set_width(r.width);
                }
            }
            None => {}
        }
        match param.crop {
            Some(c) => {
                if libwebp_sys::WebPPictureView(wp, c.x, c.y, c.width, c.height, wp) != 1 {
                    return Err(ImageError::FormatError(
                        "jpg WebPPictureView error".to_string(),
                    ));
                }
                image_result.set_height(c.height);
                image_result.set_width(c.width);
            }
            None => {}
        }

        if libwebp_sys::WebPEncode(config, wp) == 1 {

            let out: *mut libwebp_sys::WebPMemoryWriter = &mut Default::default();
            libwebp_sys::WebPMemoryWriterInit(out);
            let mut writen_len = &mut Default::default();
            libwebp_sys::WriteWebPWithMetadata(out, wp, writer, metadata, 1, writen_len);

            image_result.pic = Vec::from_raw_parts((*out).mem, (*out).size, (*out).size).clone();
            libwebp_sys::WebPPictureFree(wp);
            return Ok(image_result);
        }


        libwebp_sys::WebPPictureFree(wp);
        return Err(ImageError::FormatError("jpg encode jpg error".to_string()));
    }
}
