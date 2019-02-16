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

unsafe fn ReadJPEG() {}

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
}
