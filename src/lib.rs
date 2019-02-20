#[macro_use]
mod macros;
mod jpg;
mod webp;

pub use webp::WebPConfig;
pub use webp::WebPPicture;

use std::error::Error;
use std::fmt;

pub type ImageResult<T> = Result<T, ImageError>;

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
    (b"WEBP", ImageFormat::WEBP),
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

/// An enumeration of Image errors
#[derive(Debug)]
pub enum ImageError {
    /// The Image is not formatted properly
    FormatError(String),

    /// The Decoder does not support this image format
    UnsupportedError(String),

    TranformError(String),
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
        }
    }
}

pub fn guess_format(buffer: Vec<u8>) -> ImageResult<ImageFormat> {
    for &(signature, format) in &MAGIC_BYTES {
        if buffer.starts_with(signature) {
            return Ok(format);
        }
    }
    Err(ImageError::UnsupportedError(
        "Unsupported image format".to_string(),
    ))
}
