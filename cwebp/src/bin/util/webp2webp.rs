use crate::util::param::ImageHandler;

use imagers::ImageError;
use imagers::ImageResult;
use libwebp_sys;

use std::mem;
use std::ptr;
use std::slice;

pub fn webp_encode_webp(data: &Vec<u8>, p: ImageHandler) -> ImageResult<Vec<u8>> {
    unsafe {
        let mut pic: *mut libwebp_sys::WebPPicture = &mut Default::default();

        let status: libwebp_sys::VP8StatusCode = libwebp_sys::VP8StatusCode_VP8_STATUS_OK;
        let mut config: *mut libwebp_sys::WebPDecoderConfig = &mut Default::default();
        let mut output_buffer: *mut libwebp_sys::WebPDecBuffer = &mut Default::default();
        let mut bitstream: *mut libwebp_sys::WebPBitstreamFeatures = &mut Default::default();

        let mut stride;

        *output_buffer = (*config).output;
        *bitstream = (*config).input;

        libwebp_sys::WebPInitDecoderConfigInternal(config, libwebp_sys::WEBP_ENCODER_ABI_VERSION);

        let status = libwebp_sys::WebPGetFeaturesInternal(
            data.as_ptr(),
            data.len(),
            bitstream,
            libwebp_sys::WEBP_ENCODER_ABI_VERSION,
        );
        if (status != libwebp_sys::VP8StatusCode_VP8_STATUS_OK) {
            return Err(ImageError::FormatError("png format error".to_string()));
        }

        let has_alpha = (*bitstream).has_alpha;
        let param = p
            .set_height((*bitstream).height as i32)
            .set_width((*bitstream).width as i32)
            .adapt()
            .unwrap();
        (*pic).height = (*bitstream).height as i32;
        (*pic).width = (*bitstream).width as i32;
        if ((*pic).use_argb == 1) {
            stride = (*bitstream).width * 4;
        } else {
            if has_alpha == 1 {
                stride = (*bitstream).width * 5 / 2;
                (*pic).colorspace = libwebp_sys::WebPEncCSP_WEBP_YUV420A;
            } else {
                stride = (*bitstream).width * 3 / 2;
                (*pic).colorspace = libwebp_sys::WebPEncCSP_WEBP_YUV420;
            }
        }
        libwebp_sys::WebPPictureAlloc(pic);
        if ((*pic).use_argb == 1) {
            if cfg!(target_endian = "big") {
                (*output_buffer).colorspace = libwebp_sys::WEBP_CSP_MODE_MODE_ARGB;
            } else {
                (*output_buffer).colorspace = libwebp_sys::WEBP_CSP_MODE_MODE_BGRA;
            }
            (*output_buffer).u.RGBA.rgba = (*pic).argb as *mut u8;
            (*output_buffer).u.RGBA.stride = (*pic).argb_stride * mem::size_of::<u32>() as i32;
            (*output_buffer).u.RGBA.size =
                ((*pic).argb_stride * mem::size_of::<u32>() as i32 * (*pic).height) as usize;
        } else {
            println!("fuck");
            (*output_buffer).colorspace = if has_alpha == 1 {
                libwebp_sys::WEBP_CSP_MODE_MODE_YUVA
            } else {
                libwebp_sys::WEBP_CSP_MODE_MODE_YUV
            };
            (*output_buffer).u.YUVA.y = (*pic).y;
            (*output_buffer).u.YUVA.u = (*pic).u;
            (*output_buffer).u.YUVA.v = (*pic).v;
            (*output_buffer).u.YUVA.a = if has_alpha == 1 {
                (*pic).a
            } else {
                ptr::null_mut()
            };
            (*output_buffer).u.YUVA.y_stride = (*pic).y_stride;
            (*output_buffer).u.YUVA.u_stride = (*pic).uv_stride;
            (*output_buffer).u.YUVA.v_stride = (*pic).uv_stride;
            (*output_buffer).u.YUVA.a_stride = if has_alpha == 1 { (*pic).a_stride } else { 0 };
            (*output_buffer).u.YUVA.y_size = ((*pic).height * (*pic).y_stride) as usize;
            (*output_buffer).u.YUVA.u_size = (((*pic).height + 1) / 2 * (*pic).uv_stride) as usize;
            (*output_buffer).u.YUVA.v_size = (((*pic).height + 1) / 2 * (*pic).uv_stride) as usize;
            (*output_buffer).u.YUVA.a_size = ((*pic).height * (*pic).a_stride) as usize;
        }
        (*output_buffer).is_external_memory = 1;
        libwebp_sys::WebPDecode(data.as_ptr(), data.len(), config);
        /*
        if
            == libwebp_sys::VP8StatusCode_VP8_STATUS_OK
            && (*pic).use_argb == 1
        {
            let mut argb = slice::from_raw_parts_mut(
                (*pic).argb,
                ((*pic).argb_stride * (*pic).width) as usize,
            );
            for y in 0..(*pic).height {
                for x in 0..(*pic).width {
                    argb[(y * (*pic).argb_stride + x) as usize] |= 0xff000000_u32;
                }
            }
        }
        */
        let mut ok = 1;
        if (status != libwebp_sys::VP8StatusCode_VP8_STATUS_OK) {
            ok = 0
        }
        //libwebp_sys::WebPFreeDecBuffer(output_buffer);
        //if ok == 0 {
        //    return Err(ImageError::FormatError("png format error".to_string()));
        //}

        let mut wconfig: imagers::WebPConfig = Default::default();
        wconfig.webp_config_init();

        let writer: *mut libwebp_sys::WebPMemoryWriter = &mut Default::default();
        libwebp_sys::WebPMemoryWriterInit(writer);
        (*pic).writer = Some(libwebp_sys::WebPMemoryWrite);
        (*pic).custom_ptr = writer as *mut libc::c_void;

        if libwebp_sys::WebPEncode(wconfig.as_ptr(), pic) == 1 {
            return Ok(Vec::from_raw_parts(
                (*writer).mem,
                (*writer).size,
                (*writer).size,
            ));
        }
        return Err(ImageError::FormatError("png format error".to_string()));
    }
}
