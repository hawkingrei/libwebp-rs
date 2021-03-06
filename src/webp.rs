use crate::param::ImageHandler;
use crate::Image;
use crate::ImageError;
use crate::ImageResult;

use libc;
use libwebp_sys;
use std::mem;

pub struct WebPConfig(*mut libwebp_sys::WebPConfig);

impl Default for WebPConfig {
    fn default() -> Self {
        WebPConfig(&mut Default::default())
    }
}

#[inline(always)]
pub unsafe fn webp_config_init(c: &mut WebPConfig) {
    libwebp_sys::WebPConfigInitInternal(
        c.0,
        libwebp_sys::WebPPreset_WEBP_PRESET_DEFAULT,
        75.0 as f32,
        libwebp_sys::WEBP_ENCODER_ABI_VERSION,
    );
}

#[inline(always)]
pub unsafe fn webp_config_costum_init(c: &mut WebPConfig, arg: f32) {
    libwebp_sys::WebPConfigInitInternal(
        c.0,
        libwebp_sys::WebPPreset_WEBP_PRESET_DEFAULT,
        arg,
        libwebp_sys::WEBP_ENCODER_ABI_VERSION,
    );
}

pub struct WebPPicture(*mut libwebp_sys::WebPPicture);

impl Default for WebPPicture {
    #[inline(always)]
    fn default() -> Self {
        unsafe {
            let wp: *mut libwebp_sys::WebPPicture = &mut Default::default();
            libwebp_sys::WebPPictureAlloc(wp);
            WebPPicture(wp)
        }
    }
}

impl Drop for WebPPicture {
    #[inline(always)]
    fn drop(&mut self) {
        unsafe {
            libwebp_sys::WebPPictureFree(self.0);
        }
    }
}

impl WebPPicture {
    #[inline(always)]
    pub fn height(&self) -> i32 {
        unsafe { (*self.0).height }
    }

    #[inline(always)]
    pub fn width(&self) -> i32 {
        unsafe { (*self.0).width }
    }

    #[inline(always)]
    pub fn set_height(&mut self, height: i32) {
        unsafe { (*self.0).height = height }
    }

    #[inline(always)]
    pub fn set_width(&mut self, width: i32) {
        unsafe { (*self.0).width = width }
    }

    #[inline(always)]
    pub fn import_rgba(&mut self, rgba: Vec<u8>, rgba_stride: libc::c_int) -> WebPResult<()> {
        unsafe {
            try_ffi!(
                libwebp_sys::WebPPictureImportRGBA(self.0, rgba.as_ptr(), rgba_stride),
                (),
                WebPError::ImportRGBAError
            )
        }
    }

    #[inline(always)]
    pub fn import_rgb(&mut self, rgba: Vec<u8>, rgba_stride: libc::c_int) -> WebPResult<()> {
        unsafe {
            try_ffi!(
                libwebp_sys::WebPPictureImportRGB(self.0, rgba.as_ptr(), rgba_stride),
                (),
                WebPError::ImportRGBError
            )
        }
    }

    #[inline(always)]
    pub fn rescale(&mut self, width: libc::c_int, height: libc::c_int) -> WebPResult<()> {
        unsafe {
            try_ffi!(
                libwebp_sys::WebPPictureRescale(self.0, width, height),
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
                libwebp_sys::WebPPictureView(self.0, left, top, width, height, self.0),
                (),
                WebPError::CropError
            )
        }
    }

    #[inline(always)]
    pub fn encode(&mut self, config: &WebPConfig) -> WebPResult<Vec<u8>> {
        unsafe {
            let writer: *mut libwebp_sys::WebPMemoryWriter = &mut Default::default();
            libwebp_sys::WebPMemoryWriterInit(writer);
            (*self.0).writer = Some(libwebp_sys::WebPMemoryWrite);
            (*self.0).custom_ptr = writer as *mut libc::c_void;
            try_ffi!(
                libwebp_sys::WebPEncode(config.0, self.0),
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

pub fn webp_encode_webp(data: &[u8], mut p: ImageHandler) -> ImageResult<Image> {
    unsafe {
        let wp: *mut libwebp_sys::WebPPicture = &mut Default::default();
        let config: *mut libwebp_sys::WebPConfig = &mut Default::default();
        let mut image_result: Image = Default::default();
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
        if libwebp_sys::VP8StatusCode_VP8_STATUS_OK
            != libwebp_sys::WebPGetFeaturesInternal(
                data.as_ptr(),
                data.len(),
                bitstream,
                libwebp_sys::WEBP_ENCODER_ABI_VERSION,
            )
        {
            return Err(ImageError::FormatError(
                "webp WebPGetFeaturesInternal error".to_string(),
            ));
        }
        // "not support an animated WebP file webp"
        if 1 == ((*bitstream).has_animation) {
            image_result.pic = data.to_vec();
            image_result.set_height((*bitstream).height);
            image_result.set_width((*bitstream).width);
            return Ok(image_result);
        }
        (*wp).use_argb = (*bitstream).has_alpha;
        (*wp).height = (*bitstream).height as i32;
        (*wp).width = (*bitstream).width as i32;
        image_result.set_height((*wp).height);
        image_result.set_width((*wp).width);
        p.set_height((*wp).height as i32);
        p.set_width((*wp).width as i32);
        let param = p.adapt()?;

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

        if let Some(r) = param.resize {
            libwebp_sys::WebPPictureRescale(wp, r.width, r.height);
            image_result.set_height(r.height);
            image_result.set_width(r.width);
        }

        if let Some(c) = param.crop {
            libwebp_sys::WebPPictureView(wp, c.x, c.y, c.width, c.height, wp);
            image_result.set_height(c.height);
            image_result.set_width(c.width);
        }

        if libwebp_sys::WebPEncode(config, wp) == 1 {
            image_result.pic =
                Vec::from_raw_parts((*writer).mem, (*writer).size, (*writer).size).clone();
            libwebp_sys::WebPPictureFree(wp);
            return Ok(image_result);
        }
        libwebp_sys::WebPPictureFree(wp);
        Err(ImageError::FormatError("webp format error".to_string()))
    }
}
