use crate::param::ImageHandler;
use crate::webp::WebPConfig;
use crate::webp::WebPPicture;
use crate::ImageError;
use crate::ImageResult;

use libjpeg_turbo_sys;

use std::mem;

use crate::webp::webp_config_init;

pub fn jpg_encode_webp(data: &Vec<u8>, p: ImageHandler) -> ImageResult<Vec<u8>> {
    unsafe {
        let mut dinfo: *mut libjpeg_turbo_sys::jpeg_decompress_struct = &mut Default::default();
        let jerr: *mut libjpeg_turbo_sys::jpeg_error_mgr = &mut Default::default();

        let wp: *mut libwebp_sys::WebPPicture = &mut Default::default();
        let config: *mut libwebp_sys::WebPConfig = &mut Default::default();
        libwebp_sys::WebPPictureAlloc(wp);

        libwebp_sys::WebPConfigInitInternal(
            config,
            libwebp_sys::WebPPreset_WEBP_PRESET_DEFAULT,
            75.0 as f32,
            libwebp_sys::WEBP_ENCODER_ABI_VERSION,
        );

        libjpeg_turbo_sys::jpeg_CreateDecompress(
            dinfo,
            libjpeg_turbo_sys::JPEG_LIB_VERSION as i32,
            mem::size_of::<libjpeg_turbo_sys::jpeg_decompress_struct>(),
        );
        (*dinfo).err = libjpeg_turbo_sys::jpeg_std_error(jerr);
        libjpeg_turbo_sys::jpeg_mem_src(dinfo, data.as_ptr(), data.len() as u64);
        libjpeg_turbo_sys::jpeg_read_header(dinfo, 1);
        libjpeg_turbo_sys::jpeg_start_decompress(dinfo);

        let param = p
            .set_height((*dinfo).output_height as i32)
            .set_width((*dinfo).output_width as i32)
            .adapt()
            .unwrap();
        (*wp).height = (*dinfo).output_height as i32;
        (*wp).width = (*dinfo).output_width as i32;
        let row_stride =
            (*dinfo).output_width * (*dinfo).output_components as u32 * mem::size_of::<u8>() as u32;
        let buffer_size = row_stride * (*dinfo).image_height;
        let mut buffer = vec![0u8; buffer_size as usize];

        while (*dinfo).output_scanline < (*dinfo).output_height {
            let offset = (*dinfo).output_scanline as usize * row_stride as usize;
            let mut jsamparray = [buffer[offset..].as_mut_ptr()];
            libjpeg_turbo_sys::jpeg_read_scanlines(dinfo, jsamparray.as_mut_ptr(), 1);
        }

        libwebp_sys::WebPPictureImportRGB(wp, buffer.as_ptr(), row_stride as i32);
        let writer: *mut libwebp_sys::WebPMemoryWriter = &mut Default::default();
        libwebp_sys::WebPMemoryWriterInit(writer);
        (*wp).writer = Some(libwebp_sys::WebPMemoryWrite);
        (*wp).custom_ptr = writer as *mut libc::c_void;
        match param.resize {
            Some(r) => {
                if (r.width != 0 && r.height != 0) {
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
            let result = Vec::from_raw_parts((*writer).mem, (*writer).size, (*writer).size).clone();
            libwebp_sys::WebPPictureFree(wp);
            return Ok(result);
        }
        libwebp_sys::WebPPictureFree(wp);
        return Err(ImageError::FormatError("jpg webp encode error".to_string()));
    }
}
