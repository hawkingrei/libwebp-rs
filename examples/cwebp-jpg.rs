use std::default::Default;
use std::ffi::CStr;
use std::fs;
use std::mem;
use std::path::Path;

use libjpeg_turbo_sys;

fn main() {
    let path = Path::new("in.jpg");
    let mut data = fs::read("in.jpg").unwrap();

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

        wp.set_height((*dinfo).output_height as i32);
        wp.set_width((*dinfo).output_width as i32);
        println!("Decoded image {} x {}", wp.width(), wp.height());

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
        wp.import_rgb(buffer, row_stride as i32);
        wp.rescale(2000, 1500);
        wp.crop(0, 0, 500, 500);

        let result = wp.encode(config);
        println!("{:?}", result.len());
        fs::write("out.webp", result);

        libjpeg_turbo_sys::jpeg_finish_decompress(dinfo);
        libjpeg_turbo_sys::jpeg_destroy_decompress(dinfo);
    }
}
