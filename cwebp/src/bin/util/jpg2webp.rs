use imagers::ImageResult;
use libjpeg_turbo_sys;

use crate::util::param::ImageHandler;
use std::mem;

pub fn jpg_encode_webp(data: &Vec<u8>, p: ImageHandler) -> ImageResult<Vec<u8>> {
    unsafe {
        let mut dinfo: *mut libjpeg_turbo_sys::jpeg_decompress_struct = &mut Default::default();
        let mut jerr: *mut libjpeg_turbo_sys::jpeg_error_mgr = &mut Default::default();

        let mut wp: imagers::WebPPicture = Default::default();
        let mut config: imagers::WebPConfig = Default::default();
        config.webp_config_init();

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
        wp.set_height((*dinfo).output_height as i32);
        wp.set_width((*dinfo).output_width as i32);

        let row_stride =
            (*dinfo).output_width * (*dinfo).output_components as u32 * mem::size_of::<u8>() as u32;
        let buffer_size = row_stride * (*dinfo).image_height;
        let mut buffer = vec![0u8; buffer_size as usize];

        while (*dinfo).output_scanline < (*dinfo).output_height {
            let offset = (*dinfo).output_scanline as usize * row_stride as usize;
            let mut jsamparray = [buffer[offset..].as_mut_ptr()];
            libjpeg_turbo_sys::jpeg_read_scanlines(dinfo, jsamparray.as_mut_ptr(), 1);
        }
        println!("Decoded into {} raw pixel bytes", buffer.len());
        wp.import_rgb(buffer, row_stride as i32).unwrap();

        match param.resize {
            Some(r) => wp.rescale(r.width, r.height).unwrap(),
            None => {}
        }
        match param.crop {
            Some(c) => wp.crop(c.x, c.y, c.width, c.height).unwrap(),
            None => {}
        }
        let result = wp.encode(config);
        Ok(result.unwrap())
    }
}
