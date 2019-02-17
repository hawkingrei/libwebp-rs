#[inline(always)]
pub unsafe fn WebPConfigInit(config: *mut libwebp_sys::WebPConfig) -> libc::c_int {
    libwebp_sys::WebPConfigInitInternal(
        config,
        libwebp_sys::WebPPreset_WEBP_PRESET_DEFAULT,
        75.0 as f32,
        libwebp_sys::WEBP_ENCODER_ABI_VERSION,
    )
}
