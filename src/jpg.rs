use crate::param::ImageHandler;
use crate::webp::WebPConfig;
use crate::webp::WebPPicture;
use crate::ImageError::FormatError;
use crate::ImageResult;

use cv::core::CvType;
use cv::core::Rect;
use cv::imgcodecs::ImageReadMode;
use cv::imgcodecs::ImageWriteMode;
use cv::imgproc::ColorConversion;
use cv::imgproc::InterpolationFlag;
use cv::mat::Mat;
use libjpeg_turbo_sys;

use std::mem;

pub fn jpg_encode_mat(data: &Vec<u8>) -> Mat {
    unsafe {
        let mut dinfo: *mut libjpeg_turbo_sys::jpeg_decompress_struct = &mut Default::default();
        let jerr: *mut libjpeg_turbo_sys::jpeg_error_mgr = &mut Default::default();

        let mut wp: WebPPicture = Default::default();
        let mut config: WebPConfig = Default::default();
        config.webp_config_init();

        libjpeg_turbo_sys::jpeg_CreateDecompress(
            dinfo,
            libjpeg_turbo_sys::JPEG_LIB_VERSION as i32,
            mem::size_of::<libjpeg_turbo_sys::jpeg_decompress_struct>(),
        );
        (*dinfo).err = libjpeg_turbo_sys::jpeg_std_error(jerr);
        libjpeg_turbo_sys::jpeg_mem_src(dinfo, data.as_ptr(), data.len() as u64);
        libjpeg_turbo_sys::jpeg_read_header(dinfo, 1);
        libjpeg_turbo_sys::jpeg_start_decompress(dinfo);

        let row_stride =
            (*dinfo).output_width * (*dinfo).output_components as u32 * mem::size_of::<u8>() as u32;
        let buffer_size = row_stride * (*dinfo).image_height;
        let mut buffer = vec![0u8; buffer_size as usize];

        while (*dinfo).output_scanline < (*dinfo).output_height {
            let offset = (*dinfo).output_scanline as usize * row_stride as usize;
            let mut jsamparray = [buffer[offset..].as_mut_ptr()];
            libjpeg_turbo_sys::jpeg_read_scanlines(dinfo, jsamparray.as_mut_ptr(), 1);
        }

        Mat::from_buffer(
            (*dinfo).output_height as i32,
            (*dinfo).output_width as i32,
            CvType::Cv8UC3,
            &buffer,
        )
    }
}

pub fn mat_encode_jpg_quality(m: Mat, quality: i32) -> ImageResult<Vec<u8>> {
    let flags = vec![ImageWriteMode::JpegQuality as i32, quality];
    match m.image_encode(".jpg", flags) {
        Ok(data) => return Ok(data),
        Err(err) => return Err(FormatError(err.to_string())),
    }
}

pub fn mat_encode_jpg(m: Mat) -> ImageResult<Vec<u8>> {
    match m.image_encode(".jpeg", vec![ImageWriteMode::JpegQuality as i32, 98]) {
        Ok(data) => return Ok(data),
        Err(err) => return Err(FormatError(err.to_string())),
    }
}

pub fn jpg_encode_jpg(data: &Vec<u8>, p: ImageHandler) -> ImageResult<Vec<u8>> {
    unsafe {
        let mut dinfo: *mut libjpeg_turbo_sys::jpeg_decompress_struct = &mut Default::default();
        let jerr: *mut libjpeg_turbo_sys::jpeg_error_mgr = &mut Default::default();

        libjpeg_turbo_sys::jpeg_CreateDecompress(
            dinfo,
            libjpeg_turbo_sys::JPEG_LIB_VERSION as i32,
            mem::size_of::<libjpeg_turbo_sys::jpeg_decompress_struct>(),
        );
        (*dinfo).err = libjpeg_turbo_sys::jpeg_std_error(jerr);
        libjpeg_turbo_sys::jpeg_mem_src(dinfo, data.as_ptr(), data.len() as u64);
        libjpeg_turbo_sys::jpeg_read_header(dinfo, 1);
        libjpeg_turbo_sys::jpeg_start_decompress(dinfo);

        let param = p
            .set_height((*dinfo).output_height as i32)
            .set_width((*dinfo).output_width as i32)
            .adapt()
            .unwrap();

        let mut data_mat = Mat::image_decode(data.as_slice(), ImageReadMode::Unchanged);

        match param.resize {
            Some(r) => {
                println!("resize width: {} height: {}", r.width, r.height);
                //data_mat.resize_by(r.width as f64, r.height as f64);
            }
            None => {}
        }
        match param.crop {
            Some(c) => {
                println!(
                    "crop x: {} y: {} width: {} height: {}",
                    c.x, c.y, c.width, c.height
                );
                let r = Rect {
                    /// x coordinate of the left-top corner
                    x: c.x,
                    /// y coordinate of the left-top corner
                    y: c.y,
                    /// width of this rectangle
                    width: c.width,
                    /// height of this rectangle
                    height: c.height,
                };
                data_mat = data_mat.roi(r);
            }
            None => {}
        }
        return mat_encode_jpg(data_mat);
    }
}

pub fn jpg_encode_webp(data: &Vec<u8>, p: ImageHandler) -> ImageResult<Vec<u8>> {
    unsafe {
        let mut dinfo: *mut libjpeg_turbo_sys::jpeg_decompress_struct = &mut Default::default();
        let jerr: *mut libjpeg_turbo_sys::jpeg_error_mgr = &mut Default::default();

        let mut wp: WebPPicture = Default::default();
        let mut config: WebPConfig = Default::default();
        config.webp_config_init();

        libjpeg_turbo_sys::jpeg_CreateDecompress(
            dinfo,
            libjpeg_turbo_sys::JPEG_LIB_VERSION as i32,
            mem::size_of::<libjpeg_turbo_sys::jpeg_decompress_struct>(),
        );
        (*dinfo).err = libjpeg_turbo_sys::jpeg_std_error(jerr);
        libjpeg_turbo_sys::jpeg_mem_src(dinfo, data.as_ptr(), data.len() as u64);
        libjpeg_turbo_sys::jpeg_read_header(dinfo, 1);
        libjpeg_turbo_sys::jpeg_start_decompress(dinfo);

        let param = p
            .set_height((*dinfo).output_height as i32)
            .set_width((*dinfo).output_width as i32)
            .adapt()
            .unwrap();
        wp.set_height((*dinfo).output_height as i32);
        wp.set_width((*dinfo).output_width as i32);

        let row_stride =
            (*dinfo).output_width * (*dinfo).output_components as u32 * mem::size_of::<u8>() as u32;
        let buffer_size = row_stride * (*dinfo).image_height;
        let mut buffer = vec![0u8; buffer_size as usize];

        while (*dinfo).output_scanline < (*dinfo).output_height {
            let offset = (*dinfo).output_scanline as usize * row_stride as usize;
            let mut jsamparray = [buffer[offset..].as_mut_ptr()];
            libjpeg_turbo_sys::jpeg_read_scanlines(dinfo, jsamparray.as_mut_ptr(), 1);
        }
        println!("Decoded into {} raw pixel bytes", buffer.len());
        wp.import_rgb(buffer, row_stride as i32).unwrap();

        match param.resize {
            Some(r) => {
                println!("resize width: {} height: {}", r.width, r.height);
                wp.rescale(r.width, r.height).unwrap();
            }
            None => {}
        }
        match param.crop {
            Some(c) => {
                println!(
                    "crop x: {} y: {} width: {} height: {}",
                    c.x, c.y, c.width, c.height
                );
                wp.crop(c.x, c.y, c.width, c.height).unwrap();
            }
            None => {}
        }
        let result = wp.encode(config);
        Ok(result.unwrap())
    }
}
