#[macro_use]
mod macros;
pub mod jpg;
pub mod param;
pub mod png;
pub mod webp;

pub use jpg::jpg_encode_webp;
pub use param::Crop;
pub use param::ImageHandler;
pub use param::ImageHandlerBuilder;
pub use param::RegionCrop;
pub use param::Resize;
pub use png::png_encode_webp;
pub use webp::webp_encode_webp;
pub use webp::WebPConfig;
pub use webp::WebPPicture;

use actix_web::{client::SendRequestError, HttpResponse, ResponseError};

use std::error::Error;
use std::fmt;

pub type ImageResult<T> = Result<T, ImageError>;

#[derive(Default)]
pub struct Image {
    pub pic: Vec<u8>,
    pub height: i32,
    pub width: i32,
}

impl Image {
    pub fn set_height(&mut self, height: i32) {
        self.height = height;
    }

    pub fn set_width(&mut self, width: i32) {
        self.width = width;
    }
}

/// An enumeration of supported image formats.
/// Not all formats support both encoding and decoding.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ImageFormat {
    /// An Image in PNG Format
    PNG,

    /// An Image in JPEG Format
    JPEG,

    /// An Image in GIF Format
    GIF,

    /// An Image in WEBP Format
    WEBP,

    /// An Image in general PNM Format
    PNM,

    /// An Image in TIFF Format
    TIFF,

    /// An Image in TGA Format
    TGA,

    /// An Image in BMP Format
    BMP,

    /// An Image in ICO Format
    ICO,

    /// An Image in Radiance HDR Format
    HDR,
}

static MAGIC_BYTES: [(&'static [u8], ImageFormat); 17] = [
    (b"\x89PNG\r\n\x1a\n", ImageFormat::PNG),
    (&[0xff, 0xd8, 0xff], ImageFormat::JPEG),
    (b"GIF89a", ImageFormat::GIF),
    (b"GIF87a", ImageFormat::GIF),
    (b"RIFF", ImageFormat::WEBP),
    (b"MM.*", ImageFormat::TIFF),
    (b"II*.", ImageFormat::TIFF),
    (b"BM", ImageFormat::BMP),
    (&[0, 0, 1, 0], ImageFormat::ICO),
    (b"#?RADIANCE", ImageFormat::HDR),
    (b"P1", ImageFormat::PNM),
    (b"P2", ImageFormat::PNM),
    (b"P3", ImageFormat::PNM),
    (b"P4", ImageFormat::PNM),
    (b"P5", ImageFormat::PNM),
    (b"P6", ImageFormat::PNM),
    (b"P7", ImageFormat::PNM),
];

#[derive(Debug)]
pub enum ImageError {
    /// The Image is not formatted properly
    FormatError(String),

    /// The Decoder does not support this image format
    UnsupportedError(String),

    TranformError(String),

    ServiceError(String),

    NotFoundOrigin(String),
}

impl fmt::Display for ImageError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match *self {
            ImageError::FormatError(ref e) => write!(fmt, "Format error: {}", e),
            ImageError::UnsupportedError(ref f) => write!(
                fmt,
                "The Decoder does not support the \
                 image format `{}`",
                f
            ),
            ImageError::TranformError(ref f) => write!(fmt, "Tranform error: {}", f),
            ImageError::ServiceError(ref f) => write!(fmt, "service error: {}", f),
            ImageError::NotFoundOrigin(ref f) => write!(fmt, "not found image: {}", f),
        }
    }
}

impl std::error::Error for ImageError {
    fn description(&self) -> &str {
        match *self {
            ImageError::FormatError(ref _e) => &"Format error",
            ImageError::UnsupportedError(ref _f) => {
                &"The Decoder does not support the image format"
            }
            ImageError::TranformError(ref _f) => &"Tranform error",
            ImageError::ServiceError(ref _f) => &"Service error",
            ImageError::NotFoundOrigin(ref _f) => &"not found image",
        }
    }
}

pub fn guess_format(buffer: &Vec<u8>) -> ImageResult<ImageFormat> {
    for &(signature, format) in &MAGIC_BYTES {
        if buffer.starts_with(signature) {
            return Ok(format);
        }
    }
    Err(ImageError::UnsupportedError(
        "Unsupported image format".to_string(),
    ))
}

impl ResponseError for ImageError {
    fn error_response(&self) -> HttpResponse {
        match self {
            ImageError::UnsupportedError(_) => {
                HttpResponse::UnsupportedMediaType().body(format!("{}", self.description()))
            }
            _ => HttpResponse::Conflict().body(format!("{}", self.description())),
        }

    }
}

impl From<actix_web::error::Error> for ImageError {
    fn from(err: actix_web::error::Error) -> Self {
        ImageError::ServiceError(err.to_string())
    }
}

impl From<SendRequestError> for ImageError {
    fn from(err: SendRequestError) -> Self {
        ImageError::ServiceError(format!("actix http client error: {}", err.to_string()))
    }
}
