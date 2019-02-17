use std::default::Default;
use std::fs;
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

unsafe fn ReadJPEG(data: Vec<u8>) {
    let mut dinfo: *mut libjpeg_turbo_sys::jpeg_decompress_struct = &mut Default::default();
    let mut jerr: *mut libjpeg_turbo_sys::jpeg_error_mgr = &mut Default::default();

    (*dinfo).err = libjpeg_turbo_sys::jpeg_std_error(jerr);
    libjpeg_turbo_sys::jpeg_read_header(dinfo, 1);
    libjpeg_turbo_sys::jpeg_start_decompress(dinfo);

    let width = (*dinfo).output_width;
    let height = (*dinfo).output_height;

    let stride = (*dinfo).output_width * (*dinfo).output_components as u32 * mem::size_of::<u8>();

    libjpeg_turbo_sys::jpeg_finish_decompress(dinfo);
    libjpeg_turbo_sys::jpeg_destroy_decompress(dinfo);
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
    fs::read("in.jpg").unwrap();
}
