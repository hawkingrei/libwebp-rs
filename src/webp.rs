use libc;
use std::mem;

pub struct WebPConfig {
    webp_config: *mut libwebp_sys::WebPConfig,
}

impl Default for WebPConfig {
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
    pub fn as_raw(&mut self) -> *mut libwebp_sys::WebPConfig {
        self.webp_config
    }
}

pub struct WebPPicture {
    wp: *mut libwebp_sys::WebPPicture,
    writer: *mut libwebp_sys::WebPMemoryWriter,
}

impl Default for WebPPicture {
    fn default() -> Self {
        unsafe {
            let mut wp = &mut Default::default();
            let mut w: *mut libwebp_sys::WebPMemoryWriter = &mut Default::default();

            libwebp_sys::WebPPictureAlloc(wp);
            libwebp_sys::WebPMemoryWriterInit(w);
            (*wp).writer = Some(libwebp_sys::WebPMemoryWrite);
            (*wp).custom_ptr = w as *mut libc::c_void;
            let mut webp_picture = WebPPicture { wp: wp, writer: w };

            webp_picture
        }
    }
}

impl WebPPicture {
    pub fn height(&self) -> i32 {
        unsafe { (*self.wp).height }
    }

    pub fn width(&self) -> i32 {
        unsafe { (*self.wp).width }
    }

    pub fn set_height(&mut self, height: i32) {
        unsafe { (*self.wp).height = height }
    }

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

    pub fn encode(&mut self, mut config: WebPConfig) -> Vec<u8> {
        unsafe {
            libwebp_sys::WebPEncode(config.as_raw(), self.wp);
            Vec::from_raw_parts((*self.writer).mem, (*self.writer).size, (*self.writer).size)
        }
    }

    pub fn writer(
        &mut self,
        w: Option<unsafe extern "C" fn(*const u8, usize, *const libwebp_sys::WebPPicture) -> i32>,
    ) {
        unsafe {
            (*self.wp).writer = w;
        }
    }
}

impl Drop for WebPPicture {
    fn drop(&mut self) {
        unsafe {
            println!("clean drop");
            drop(self.writer);
            libwebp_sys::WebPPictureFree(self.wp);
            drop(self);
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
