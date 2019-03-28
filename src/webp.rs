use crate::param::ImageHandler;
use crate::ImageError;
use crate::ImageResult;

use libc;
use std::mem;

#[repr(C)]
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
    pub unsafe fn webp_config_init(&mut self) {
        libwebp_sys::WebPConfigInitInternal(
            self.webp_config,
            libwebp_sys::WebPPreset_WEBP_PRESET_DEFAULT,
            75.0 as f32,
            libwebp_sys::WEBP_ENCODER_ABI_VERSION,
        );
    }

    #[inline(always)]
    pub unsafe fn webp_config_costum_init(&mut self, arg: f32) {
        libwebp_sys::WebPConfigInitInternal(
            self.webp_config,
            libwebp_sys::WebPPreset_WEBP_PRESET_DEFAULT,
            arg,
            libwebp_sys::WEBP_ENCODER_ABI_VERSION,
        );
    }

    #[inline(always)]
    pub fn as_ptr(&mut self) -> *mut libwebp_sys::WebPConfig {
        self.webp_config
    }
}

pub struct WebPPicture {
    pub wp: *mut libwebp_sys::WebPPicture,
}

impl Default for WebPPicture {
    #[inline(always)]
    fn default() -> Self {
        unsafe {
            let wp: *mut libwebp_sys::WebPPicture = &mut Default::default();
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

    #[inline]
    pub fn import_rgba(&mut self, rgba: Vec<u8>, rgba_stride: libc::c_int) -> WebPResult<()> {
        unsafe {
            try_ffi!(
                libwebp_sys::WebPPictureImportRGBA(self.wp, rgba.as_ptr(), rgba_stride),
                (),
                WebPError::ImportRGBAError
            )
        }
    }

    #[inline]
    pub fn import_rgb(&mut self, rgba: Vec<u8>, rgba_stride: libc::c_int) -> WebPResult<()> {
        unsafe {
            try_ffi!(
                libwebp_sys::WebPPictureImportRGB(self.wp, rgba.as_ptr(), rgba_stride),
                (),
                WebPError::ImportRGBError
            )
        }
    }

    #[inline(always)]
    pub fn rescale(&mut self, width: libc::c_int, height: libc::c_int) -> WebPResult<()> {
        unsafe {
            try_ffi!(
                libwebp_sys::WebPPictureRescale(self.wp, width, height),
                (),
                WebPError::RescaleError
            )
        }
    }

    #[inline(always)]
    pub fn crop(
        &mut self,
        left: libc::c_int,
        top: libc::c_int,
        width: libc::c_int,
        height: libc::c_int,
    ) -> WebPResult<()> {
        unsafe {
            try_ffi!(
                libwebp_sys::WebPPictureView(self.wp, left, top, width, height, self.wp),
                (),
                WebPError::CropError
            )
        }
    }

    #[inline(always)]
    pub fn encode(&mut self, mut config: WebPConfig) -> WebPResult<Vec<u8>> {
        unsafe {
            let writer: *mut libwebp_sys::WebPMemoryWriter = &mut Default::default();
            libwebp_sys::WebPMemoryWriterInit(writer);
            (*self.wp).writer = Some(libwebp_sys::WebPMemoryWrite);
            (*self.wp).custom_ptr = writer as *mut libc::c_void;
            try_ffi!(
                libwebp_sys::WebPEncode(config.as_ptr(), self.wp),
                Vec::from_raw_parts((*writer).mem, (*writer).size, (*writer).size),
                WebPError::EncodeError
            )
        }
    }
}

pub type WebPResult<T> = Result<T, WebPError>;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum WebPError {
    RescaleError,
    CropError,
    EncodeError,
    ImportRGBAError,
    ImportRGBError,
}

pub fn webp_encode_webp(data: &Vec<u8>, p: ImageHandler) -> ImageResult<Vec<u8>> {
    unsafe {
        let wp: *mut libwebp_sys::WebPPicture = &mut Default::default();
        let config: *mut libwebp_sys::WebPConfig = &mut Default::default();

        libwebp_sys::WebPPictureAlloc(wp);

        libwebp_sys::WebPConfigInitInternal(
            config,
            libwebp_sys::WebPPreset_WEBP_PRESET_DEFAULT,
            75.0 as f32,
            libwebp_sys::WEBP_ENCODER_ABI_VERSION,
        );

        let decoder_config: *mut libwebp_sys::WebPDecoderConfig = &mut Default::default();
        let output_buffer: *mut libwebp_sys::WebPDecBuffer = &mut Default::default();
        let bitstream: *mut libwebp_sys::WebPBitstreamFeatures = &mut Default::default();

        *output_buffer = (*decoder_config).output;
        *bitstream = (*decoder_config).input;
        libwebp_sys::WebPInitDecoderConfigInternal(
            decoder_config,
            libwebp_sys::WEBP_ENCODER_ABI_VERSION,
        );
        let mut status: libwebp_sys::VP8StatusCode = libwebp_sys::VP8StatusCode_VP8_STATUS_OK;
        status = libwebp_sys::WebPGetFeaturesInternal(
            data.as_ptr(),
            data.len(),
            bitstream,
            libwebp_sys::WEBP_ENCODER_ABI_VERSION,
        );
        if (status != libwebp_sys::VP8StatusCode_VP8_STATUS_OK) {
            return Err(ImageError::FormatError("png format error".to_string()));
        }
        (*wp).use_argb = (*bitstream).has_alpha;
        (*wp).height = (*bitstream).height as i32;
        (*wp).width = (*bitstream).width as i32;
        let param = p
            .set_height((*wp).height)
            .set_width((*wp).width)
            .adapt()
            .unwrap();
        if (*wp).use_argb == 1 {
            let d = libwebp_sys::WebPDecodeRGBA(
                data.as_ptr(),
                data.len(),
                &mut (*wp).width,
                &mut (*wp).height,
            );
            let stride = 4 * (*wp).width * mem::size_of::<u8>() as i32;
            libwebp_sys::WebPPictureImportRGBA(wp, d, stride);
        } else {
            let d = libwebp_sys::WebPDecodeRGB(
                data.as_ptr(),
                data.len(),
                &mut (*wp).width,
                &mut (*wp).height,
            );
            let stride = 3 * (*wp).width * mem::size_of::<u8>() as i32;
            libwebp_sys::WebPPictureImportRGB(wp, d, stride);
        }

        let writer: *mut libwebp_sys::WebPMemoryWriter = &mut Default::default();
        libwebp_sys::WebPMemoryWriterInit(writer);
        (*wp).writer = Some(libwebp_sys::WebPMemoryWrite);
        (*wp).custom_ptr = writer as *mut libc::c_void;
        match param.resize {
            Some(r) => {
                libwebp_sys::WebPPictureRescale(wp, r.width, r.height);
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
            return Ok(Vec::from_raw_parts(
                (*writer).mem,
                (*writer).size,
                (*writer).size,
            ));
        }
        return Err(ImageError::FormatError("png format error".to_string()));
    }
}
