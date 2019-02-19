use libc;
use std::mem;

pub struct WebPConfig {
    webp_config: *mut libwebp_sys::WebPConfig,
}

impl Default for WebPConfig {
    #[inline(always)]
    fn default() -> Self {
        WebPConfig {
            webp_config: &mut Default::default(),
        }
    }
}

impl WebPConfig {
    #[inline(always)]
    pub unsafe fn WebPConfigInit(&mut self) {
        libwebp_sys::WebPConfigInitInternal(
            self.webp_config,
            libwebp_sys::WebPPreset_WEBP_PRESET_DEFAULT,
            75.0 as f32,
            libwebp_sys::WEBP_ENCODER_ABI_VERSION,
        );
    }

    #[inline(always)]
    pub fn as_ptr(&mut self) -> *mut libwebp_sys::WebPConfig {
        self.webp_config
    }
}

pub struct WebPPicture {
    wp: *mut libwebp_sys::WebPPicture,
}

impl Default for WebPPicture {
    #[inline(always)]
    fn default() -> Self {
        unsafe {
            let mut wp: *mut libwebp_sys::WebPPicture = &mut Default::default();
            libwebp_sys::WebPPictureAlloc(wp);
            WebPPicture { wp }
        }
    }
}

impl WebPPicture {
    #[inline(always)]
    pub fn height(&self) -> i32 {
        unsafe { (*self.wp).height }
    }

    #[inline(always)]
    pub fn width(&self) -> i32 {
        unsafe { (*self.wp).width }
    }

    #[inline(always)]
    pub fn set_height(&mut self, height: i32) {
        unsafe { (*self.wp).height = height }
    }

    #[inline(always)]
    pub fn set_width(&mut self, width: i32) {
        unsafe { (*self.wp).width = width }
    }

    #[inline(always)]
    pub fn ImportRGBA(&mut self, rgba: *const u8, rgba_stride: libc::c_int) {
        unsafe {
            libwebp_sys::WebPPictureImportRGBA(self.wp, rgba, rgba_stride);
        }
    }

    #[inline(always)]
    pub fn ImportRGB(&mut self, rgba: *const u8, rgba_stride: libc::c_int) {
        unsafe {
            libwebp_sys::WebPPictureImportRGB(self.wp, rgba, rgba_stride);
        }
    }

    #[inline(always)]
    pub fn encode(&mut self, mut config: WebPConfig) -> Vec<u8> {
        unsafe {
            let writer: *mut libwebp_sys::WebPMemoryWriter = &mut Default::default();
            libwebp_sys::WebPMemoryWriterInit(writer);
            (*self.wp).writer = Some(libwebp_sys::WebPMemoryWrite);
            (*self.wp).custom_ptr = writer as *mut libc::c_void;
            libwebp_sys::WebPEncode(config.as_ptr(), self.wp);
            Vec::from_raw_parts((*writer).mem, (*writer).size, (*writer).size)
        }
    }
}

#[inline(always)]
pub unsafe fn WebPConfigInit(config: *mut libwebp_sys::WebPConfig) -> libc::c_int {
    libwebp_sys::WebPConfigInitInternal(
        config,
        libwebp_sys::WebPPreset_WEBP_PRESET_DEFAULT,
        75.0 as f32,
        libwebp_sys::WEBP_ENCODER_ABI_VERSION,
    )
}
