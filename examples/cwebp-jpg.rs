use std::default::Default;
use std::ffi::CStr;
use std::fs;
use std::mem;
use std::path::Path;

use libjpeg_turbo_sys;

unsafe extern "C" fn MyViewer(
    data: *const u8,
    data_size: usize,
    picture: *const libwebp_sys::WebPPicture,
) -> libc::c_int {
    let out: *mut libc::FILE = (*picture).custom_ptr as *mut libc::FILE;
    if data_size > 0 {
        libc::fwrite(data as *const libc::c_void, data_size, 1, out) as i32
    } else {
        1
    }
}

#[inline(always)]
unsafe fn WebPConfigInit(config: *mut libwebp_sys::WebPConfig) -> libc::c_int {
    libwebp_sys::WebPConfigInitInternal(
        config,
        libwebp_sys::WebPPreset_WEBP_PRESET_DEFAULT,
        75.0 as f32,
        libwebp_sys::WEBP_ENCODER_ABI_VERSION,
    )
}

fn main() {
    let path = Path::new("in.jpg");
    let mut data = fs::read("in.jpg").unwrap();

    unsafe {
        let mut dinfo: *mut libjpeg_turbo_sys::jpeg_decompress_struct = &mut Default::default();
        let mut jerr: *mut libjpeg_turbo_sys::jpeg_error_mgr = &mut Default::default();
        let wp: *mut libwebp_sys::WebPPicture = &mut Default::default();
        let config: *mut libwebp_sys::WebPConfig = &mut Default::default();

        let c_file = libc::fopen(
            CStr::from_bytes_with_nul_unchecked(b"out.webp\0").as_ptr(),
            CStr::from_bytes_with_nul_unchecked(b"wb\0").as_ptr(),
        );
        (*wp).writer = Some(MyViewer);
        (*wp).custom_ptr = c_file as *mut libc::c_void;
        imagers::WebPConfigInit(config);

        libwebp_sys::WebPPictureAlloc(wp);
        libjpeg_turbo_sys::jpeg_CreateDecompress(
            dinfo,
            libjpeg_turbo_sys::JPEG_LIB_VERSION as i32,
            mem::size_of::<libjpeg_turbo_sys::jpeg_decompress_struct>(),
        );
        (*dinfo).err = libjpeg_turbo_sys::jpeg_std_error(jerr);
        libjpeg_turbo_sys::jpeg_mem_src(dinfo, data.as_ptr(), data.len() as u64);
        libjpeg_turbo_sys::jpeg_read_header(dinfo, 1);
        libjpeg_turbo_sys::jpeg_start_decompress(dinfo);

        (*wp).width = (*dinfo).output_width as i32;
        (*wp).height = (*dinfo).output_height as i32;

        println!("Decoded image {} x {}", (*wp).width, (*wp).height);

        let row_stride =
            (*dinfo).output_width * (*dinfo).output_components as u32 * mem::size_of::<u8>() as u32;
        let buffer_size = row_stride * (*dinfo).image_height;
        let mut buffer = vec![0u8; buffer_size as usize];

        while (*dinfo).output_scanline < (*dinfo).output_height {
            let offset = (*dinfo).output_scanline as usize * row_stride as usize;
            let mut jsamparray = [buffer[offset..].as_mut_ptr()];
            libjpeg_turbo_sys::jpeg_read_scanlines(dinfo, jsamparray.as_mut_ptr(), 1);
        }

        libjpeg_turbo_sys::jpeg_finish_decompress(dinfo);
        libjpeg_turbo_sys::jpeg_destroy_decompress(dinfo);

        libwebp_sys::WebPPictureImportRGB(wp, buffer.as_ptr(), row_stride as i32);

        libc::fclose(c_file);
        libwebp_sys::WebPPictureFree(wp);
    }
}
