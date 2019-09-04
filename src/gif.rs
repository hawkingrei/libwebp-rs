use crate::param::ImageHandler;
use crate::Image;
use crate::ImageError;
use crate::ImageResult;

use std::convert::TryInto;
use std::fmt;
use std::io::Write;
use std::process::{Command, Stdio};
use std::ptr;

use libc;

const GIF_LIMIT_SIZE: i32 = 450 * 450;
const GIF_MAX_FRAME: i32 = 300;

pub fn gif_encode_webp(data: &mut Vec<u8>, mut p: ImageHandler) -> ImageResult<Image> {
    match gif_info(data) {
        Ok(info) => {
            if info.frame_count > GIF_MAX_FRAME
                || info.width * info.height > GIF_LIMIT_SIZE && !p.first_frame
            {
                let mut image_result: Image = Default::default();
                image_result.pic = (*data).clone();
                image_result.width = info.width;
                image_result.height = info.height;
                return Err(ImageError::LimitError(
                    image_result,
                    "over the limitation".to_string(),
                ));
            }
            p.set_height(info.height as i32);
            p.set_width(info.width as i32);
            let param = p.adapt()?;
            if param.first_frame {
                return gif_to_webp(data, param);
            }
            if let Some(resize) = param.resize {
                if resize.height != 0 || resize.width != 0 {
                    return gif_all_resize_webp(data, param);
                } else {
                    return gif_to_webp(data, param);
                }
            }
            return gif_to_webp(data, param);
        }
        Err(e) => {
            return Err(ImageError::FormatError(e.to_string()));
        }
    }
}

fn gif_all_resize_webp(data: &mut Vec<u8>, p: ImageHandler) -> ImageResult<Image> {
    if let Some(resize) = p.resize {
        match Command::new("gifsicle")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .arg("--careful")
            .arg(format!("--resize={}x{}", resize.width, resize.height))
            .spawn()
        {
            Ok(mut child) => {
                let child_stdin = child.stdin.as_mut().unwrap();
                child_stdin
                    .write_all(data)
                    .map_err(|e| ImageError::TranformError(e.to_string()))?;
                match child.wait_with_output() {
                    Ok(mut result) => {
                        if result.status.success() {
                            return gif_to_webp(&mut result.stdout, p);
                        }
                        return Err(ImageError::TranformError(
                            "exiting not return 0".to_string(),
                        ));
                    }
                    Err(e) => return Err(ImageError::TranformError(e.to_string())),
                }
            }
            Err(e) => return Err(ImageError::TranformError(e.to_string())),
        }
    }
    return Err(ImageError::TranformError("resize is none".to_string()));
}

pub struct GIFInfo {
    pub frame_count: i32,
    pub height: i32,
    pub width: i32,
}

impl fmt::Debug for GIFInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("GIFInfo")
            .field("height", &self.height)
            .field("width", &self.width)
            .field("frame count", &self.frame_count)
            .finish()
    }
}

pub fn gif_info(data: &mut Vec<u8>) -> ImageResult<GIFInfo> {
    unsafe {
        let mut frame_number = 0;
        let mut loop_count: i32 = 0;
        let mut gif_err: i32 = 0;
        let mut code_size: i32 = 0;
        let mut buf_src: *mut libwebp_sys::BufferSource = &mut libwebp_sys::BufferSource {
            buf: ptr::null_mut(),
            p: ptr::null_mut(),
            remain: 0,
        };
        (*buf_src).buf = data.as_mut_ptr();
        (*buf_src).p = data.as_mut_ptr();
        (*buf_src).remain = data.len().try_into().unwrap();
        let mut gif: *mut libwebp_sys::GifFileType = libwebp_sys::DGifOpen(
            buf_src as *mut core::ffi::c_void,
            Some(libwebp_sys::readGifBuffer),
            &mut gif_err,
        );
        let mut done = 0;
        let mut code_block: *mut libwebp_sys::GifByteType = ptr::null_mut();
        loop {
            let mut gtype: libwebp_sys::GifRecordType = 0;
            if libwebp_sys::DGifGetRecordType(gif, &mut gtype) == 0 {
                return Err(ImageError::FormatError(
                    "fail to get gif record type".to_string(),
                ));
            }
            match gtype {
                libwebp_sys::GifRecordType_IMAGE_DESC_RECORD_TYPE => {
                    let mut image_desc: libwebp_sys::GifImageDesc = (*gif).Image;
                    if libwebp_sys::DGifGetImageDesc(gif) == 0 {
                        return Err(ImageError::FormatError("fail to get gif desc".to_string()));
                    }
                    if frame_number == 0 {
                        if (*gif).SWidth == 0 || (*gif).SHeight == 0 {
                            image_desc.Left = 0;
                            image_desc.Top = 0;
                            (*gif).SWidth = image_desc.Width;
                            (*gif).SHeight = image_desc.Height;
                            if (*gif).SWidth <= 0 || (*gif).SHeight <= 0 {
                                return Err(ImageError::FormatError(
                                    "illagel gif size".to_string(),
                                ));
                            }
                        }
                    }
                    frame_number = frame_number + 1;
                    if libwebp_sys::DGifGetCode(gif, &mut code_size, &mut code_block) == 0 {
                        return Err(ImageError::FormatError("fail to get gif code".to_string()));
                    }
                    while !code_block.is_null() {
                        if libwebp_sys::DGifGetCodeNext(gif, &mut code_block) == 0 {
                            return Err(ImageError::FormatError(
                                "fail to get gif code next".to_string(),
                            ));
                        }
                    }
                }
                libwebp_sys::GifRecordType_EXTENSION_RECORD_TYPE => {
                    let mut extension: i32 = 0;
                    let mut data: *mut libwebp_sys::GifByteType = ptr::null_mut();
                    if libwebp_sys::DGifGetExtension(gif, &mut extension, &mut data) == 0 {
                        return Err(ImageError::FormatError(
                            "fail to get gif extension".to_string(),
                        ));
                    }
                    if data.is_null() {
                        continue;
                    }
                    match extension {
                        0xf2 | 0xf9 | 0x01 => {}
                        0xff => {
                            if *(data.offset(0)) == 11 {
                                if libc::memcmp(
                                    data.offset(1) as *const libc::c_void,
                                    "NETSCAPE2.0".as_ptr() as *const libc::c_void,
                                    11,
                                ) == 0
                                    || libc::memcmp(
                                        data.offset(1) as *const libc::c_void,
                                        "ANIMEXTS1.0".as_ptr() as *const libc::c_void,
                                        11,
                                    ) == 0
                                {
                                    if libwebp_sys::GIFReadLoopCount(
                                        gif,
                                        &mut data,
                                        &mut loop_count,
                                    ) == 0
                                    {
                                        return Err(ImageError::FormatError(
                                            "fail to read gif loop count".to_string(),
                                        ));
                                    }
                                } else {
                                    let is_xmp: bool = libc::memcmp(
                                        data.offset(1) as *const libc::c_void,
                                        "XMP DataXMP".as_ptr() as *const libc::c_void,
                                        11,
                                    ) == 0;
                                    let is_icc: bool = libc::memcmp(
                                        data.offset(1) as *const libc::c_void,
                                        "ICCRGBG1012".as_ptr() as *const libc::c_void,
                                        11,
                                    ) == 0;
                                    if is_icc || is_xmp {
                                        if libwebp_sys::DGifGetExtensionNext(gif, &mut data) == 0 {
                                            // goto end
                                            return Err(ImageError::FormatError(
                                                "fail to get gif extension next".to_string(),
                                            ));
                                        }
                                    }
                                }
                            }
                        }
                        _ => {}
                    }

                    while !data.is_null() {
                        if libwebp_sys::DGifGetExtensionNext(gif, &mut data) == 0 {
                            // goto end
                            return Err(ImageError::FormatError(
                                "fail to get gif extension next".to_string(),
                            ));
                        }
                    }
                }
                libwebp_sys::GifRecordType_TERMINATE_RECORD_TYPE => {
                    done = 1;
                }
                _ => {
                    return Err(ImageError::FormatError(
                        "unknow gif record type".to_string(),
                    ));
                }
            }
            if done == 1 {
                break;
            }
        }
        return Ok(GIFInfo {
            frame_count: frame_number,
            height: (*gif).SHeight,
            width: (*gif).SWidth,
        });
    }
}

fn gif_to_webp(data: &mut Vec<u8>, p: ImageHandler) -> ImageResult<Image> {
    unsafe {
        let mut image_result: Image = Default::default();

        let mut frame_duration: i32 = 0;
        let mut transparent_index: i32 = -1;
        let loop_compatibility: i32 = 0;
        let mut loop_count: i32 = 0;
        let mut stored_loop_count: i32 = 0;

        let webp_data: *mut libwebp_sys::WebPData = &mut libwebp_sys::WebPData {
            bytes: ptr::null_mut(),
            size: 0,
        };

        let config: *mut libwebp_sys::WebPConfig = &mut Default::default();
        let mut frame: *mut libwebp_sys::WebPPicture = &mut Default::default();
        let curr_canvas: *mut libwebp_sys::WebPPicture = &mut Default::default();
        let prev_canvas: *mut libwebp_sys::WebPPicture = &mut Default::default();
        let mut enc_options: libwebp_sys::WebPAnimEncoderOptions = Default::default();
        let mut enc: *mut libwebp_sys::WebPAnimEncoder = ptr::null_mut();
        #[allow(unused_assignments)]
        let mut mux: *mut libwebp_sys::WebPMux = ptr::null_mut();
        let mut orig_dispose: libwebp_sys::GIFDisposeMethod =
            libwebp_sys::GIFDisposeMethod_GIF_DISPOSE_NONE;
        libwebp_sys::PWebPDataInit(webp_data);
        libwebp_sys::WebPPictureInitInternal(frame, libwebp_sys::WEBP_ENCODER_ABI_VERSION);
        libwebp_sys::WebPPictureInitInternal(curr_canvas, libwebp_sys::WEBP_ENCODER_ABI_VERSION);
        libwebp_sys::WebPPictureInitInternal(prev_canvas, libwebp_sys::WEBP_ENCODER_ABI_VERSION);
        libwebp_sys::WebPAnimEncoderOptionsInitInternal(
            &mut enc_options,
            libwebp_sys::WEBP_MUX_ABI_VERSION,
        );
        if p.quality() > 0 {
            libwebp_sys::WebPConfigInitInternal(
                config,
                libwebp_sys::WebPPreset_WEBP_PRESET_DEFAULT,
                p.quality() as f32,
                libwebp_sys::WEBP_ENCODER_ABI_VERSION,
            );
        } else {
            libwebp_sys::WebPConfigInitInternal(
                config,
                libwebp_sys::WebPPreset_WEBP_PRESET_DEFAULT,
                75.0 as f32,
                libwebp_sys::WEBP_ENCODER_ABI_VERSION,
            );
        }
        enc_options.allow_mixed = 1;
        (*config).lossless = 0;
        let mut frame_timestamp: i32 = 0;
        let mut frame_number = 0;
        let mut gif_err: i32 = 0;
        let mut buf_src: *mut libwebp_sys::BufferSource = &mut libwebp_sys::BufferSource {
            buf: ptr::null_mut(),
            p: ptr::null_mut(),
            remain: 0,
        };
        (*buf_src).buf = data.as_mut_ptr();
        (*buf_src).p = data.as_mut_ptr();
        (*buf_src).remain = data.len().try_into().unwrap();
        let mut gif: *mut libwebp_sys::GifFileType = libwebp_sys::DGifOpen(
            buf_src as *mut core::ffi::c_void,
            Some(libwebp_sys::readGifBuffer),
            &mut gif_err,
        );
        if gif.is_null() {
            return Err(ImageError::FormatError("fail to open gif".to_string()));
        }
        let mut done = 0;
        loop {
            let mut gtype: libwebp_sys::GifRecordType = 0;
            if libwebp_sys::DGifGetRecordType(gif, &mut gtype) == 0 {
                return Err(ImageError::FormatError(
                    "fail to get gif recode type".to_string(),
                ));
            }
            match gtype {
                libwebp_sys::GifRecordType_IMAGE_DESC_RECORD_TYPE => {
                    let mut gif_rect: libwebp_sys::GIFFrameRect = Default::default();
                    let mut image_desc: libwebp_sys::GifImageDesc = (*gif).Image;
                    if libwebp_sys::DGifGetImageDesc(gif) == 0 {
                        return Err(ImageError::FormatError(
                            "fail to get gif image desc".to_string(),
                        ));
                    }
                    if frame_number == 0 {
                        if (*gif).SWidth == 0 || (*gif).SHeight == 0 {
                            image_desc.Left = 0;
                            image_desc.Top = 0;
                            (*gif).SWidth = image_desc.Width;
                            (*gif).SHeight = image_desc.Height;
                            if (*gif).SWidth <= 0 || (*gif).SHeight <= 0 {
                                return Err(ImageError::FormatError(
                                    "illagel gif size".to_string(),
                                ));
                            }
                        }
                        image_result.width = (*gif).SWidth;
                        image_result.height = (*gif).SHeight;
                        (*frame).width = (*gif).SWidth;
                        (*frame).height = (*gif).SHeight;
                        (*frame).use_argb = 1;
                        if libwebp_sys::WebPPictureAlloc(frame) == 0 {
                            return Err(ImageError::TranformError(
                                "fail to alloc webp picture".to_string(),
                            ));
                        }
                        libwebp_sys::GIFClearPic(frame, ptr::null());

                        libwebp_sys::WebPPictureCopy(frame, curr_canvas);
                        libwebp_sys::WebPPictureCopy(frame, prev_canvas);

                        // Background color.
                        libwebp_sys::GIFGetBackgroundColor(
                            (*gif).SColorMap,
                            (*gif).SBackGroundColor,
                            transparent_index,
                            &mut enc_options.anim_params.bgcolor,
                        );

                        // Initialize encoder.
                        enc = match p.resize {
                            Some(r) => {
                                if r.width != 0 && r.height != 0 {
                                    image_result.width = r.width;
                                    image_result.height = r.height;
                                    libwebp_sys::WebPAnimEncoderNewInternal(
                                        r.width,
                                        r.height,
                                        &enc_options,
                                        libwebp_sys::WEBP_MUX_ABI_VERSION,
                                    )
                                } else {
                                    libwebp_sys::WebPAnimEncoderNewInternal(
                                        (*curr_canvas).width,
                                        (*curr_canvas).height,
                                        &enc_options,
                                        libwebp_sys::WEBP_MUX_ABI_VERSION,
                                    )
                                }
                            }
                            None => libwebp_sys::WebPAnimEncoderNewInternal(
                                (*curr_canvas).width,
                                (*curr_canvas).height,
                                &enc_options,
                                libwebp_sys::WEBP_MUX_ABI_VERSION,
                            ),
                        };

                        if enc.is_null() {
                            return Err(ImageError::TranformError(
                                "fail to init WebPAnimEncoder".to_string(),
                            ));
                        }
                    }
                    if p.first_frame && frame_number >= 1 {
                        if libwebp_sys::GIFReadFrame(gif, transparent_index, &mut gif_rect, frame)
                            == 0
                        {
                            return Err(ImageError::FormatError(
                                "fail to read gif frame".to_string(),
                            ));
                        }
                    } else {
                        // Some even more broken GIF can have sub-rect with zero width/height.
                        if image_desc.Width == 0 || image_desc.Height == 0 {
                            image_desc.Width = (*gif).SWidth;
                            image_desc.Height = (*gif).SHeight;
                        }

                        if libwebp_sys::GIFReadFrame(gif, transparent_index, &mut gif_rect, frame)
                            == 0
                        {
                            return Err(ImageError::FormatError(
                                "fail to read gif frame".to_string(),
                            ));
                        }
                        // Blend frame rectangle with previous canvas to compose full canvas.
                        // Note that 'curr_canvas' is same as 'prev_canvas' at this point.
                        libwebp_sys::GIFBlendFrames(frame, &gif_rect, curr_canvas);
                        if p.first_frame {
                            match p.resize {
                                Some(r) => {
                                    if r.width != 0 && r.height != 0 {
                                        if libwebp_sys::WebPPictureRescale(
                                            curr_canvas,
                                            r.width,
                                            r.height,
                                        ) != 1
                                        {
                                            return Err(ImageError::FormatError(
                                                "gif WebPPictureRescale error".to_string(),
                                            ));
                                        }
                                    }
                                }
                                None => {}
                            }
                        }
                        if libwebp_sys::WebPAnimEncoderAdd(
                            enc,
                            curr_canvas,
                            if p.first_frame { 0 } else { frame_timestamp },
                            config,
                        ) == 0
                        {
                            return Err(ImageError::TranformError(
                                "fail to WebPAnimEncoderAdd".to_string(),
                            ));
                        } else {
                            frame_number = frame_number + 1;
                        }
                        if p.first_frame {
                            match p.resize {
                                Some(r) => {
                                    if r.width != 0 && r.height != 0 {
                                        if libwebp_sys::WebPPictureRescale(
                                            prev_canvas,
                                            r.width,
                                            r.height,
                                        ) != 1
                                        {
                                            return Err(ImageError::TranformError(
                                                "gif WebPPictureRescale error".to_string(),
                                            ));
                                        }
                                    }
                                }
                                None => {}
                            }
                        }

                        // Update canvases.
                        libwebp_sys::GIFDisposeFrame(
                            orig_dispose,
                            &gif_rect,
                            prev_canvas,
                            curr_canvas,
                        );
                        libwebp_sys::GIFCopyPixels(curr_canvas, prev_canvas);

                        // Force frames with a small or no duration to 100ms to be consistent
                        // with web browsers and other transcoding tools. This also avoids
                        // incorrect durations between frames when padding frames are
                        // discarded.
                        if frame_duration <= 10 {
                            frame_duration = 100;
                        }

                        // Update timestamp (for next frame).
                        frame_timestamp = frame_timestamp + frame_duration;
                    }

                    // In GIF, graphic control extensions are optional for a frame, so we
                    // may not get one before reading the next frame. To handle this case,
                    // we reset frame properties to reasonable defaults for the next frame.
                    orig_dispose = libwebp_sys::GIFDisposeMethod_GIF_DISPOSE_NONE;
                    frame_duration = 0;
                    transparent_index = -1;
                }
                libwebp_sys::GifRecordType_EXTENSION_RECORD_TYPE => {
                    let mut extension: i32 = 0;
                    let mut data: *mut libwebp_sys::GifByteType = ptr::null_mut();
                    if libwebp_sys::DGifGetExtension(gif, &mut extension, &mut data) == 0 {
                        // goto end
                        return Err(ImageError::FormatError(
                            "fail to get gif extension".to_string(),
                        ));
                    }
                    if data.is_null() {
                        continue;
                    }
                    if !(p.first_frame && frame_number >= 1) {
                        match extension {
                            0xf2 => {}
                            0xf9 => {
                                if libwebp_sys::GIFReadGraphicsExtension(
                                    data,
                                    &mut frame_duration,
                                    &mut orig_dispose,
                                    &mut transparent_index,
                                ) == 0
                                {
                                    // goto end
                                    return Err(ImageError::FormatError(
                                        "fail to read gif Graphics extension".to_string(),
                                    ));
                                }
                            }
                            0x01 => {}
                            0xff => {
                                if *(data.offset(0)) == 11 {
                                    if libc::memcmp(
                                        data.offset(1) as *const libc::c_void,
                                        "NETSCAPE2.0".as_ptr() as *const libc::c_void,
                                        11,
                                    ) == 0
                                        || libc::memcmp(
                                            data.offset(1) as *const libc::c_void,
                                            "ANIMEXTS1.0".as_ptr() as *const libc::c_void,
                                            11,
                                        ) == 0
                                    {
                                        if libwebp_sys::GIFReadLoopCount(
                                            gif,
                                            &mut data,
                                            &mut loop_count,
                                        ) == 0
                                        {
                                            // goto end
                                            return Err(ImageError::FormatError(
                                                "fail to read gif loop".to_string(),
                                            ));
                                        }
                                        stored_loop_count = if loop_compatibility == 0 {
                                            if loop_count != 0 {
                                                0
                                            } else {
                                                1
                                            }
                                        } else {
                                            1
                                        };
                                    } else {
                                        let is_xmp: bool = libc::memcmp(
                                            data.offset(1) as *const libc::c_void,
                                            "XMP DataXMP".as_ptr() as *const libc::c_void,
                                            11,
                                        ) == 0;
                                        let is_icc: bool = libc::memcmp(
                                            data.offset(1) as *const libc::c_void,
                                            "ICCRGBG1012".as_ptr() as *const libc::c_void,
                                            11,
                                        ) == 0;
                                        if is_icc || is_xmp {
                                            if libwebp_sys::DGifGetExtensionNext(gif, &mut data)
                                                == 0
                                            {
                                                // goto end
                                                return Err(ImageError::FormatError(
                                                    "fail to get gif extension next".to_string(),
                                                ));
                                            }
                                        }
                                    }
                                }
                            }
                            _ => {}
                        }
                    }

                    while !data.is_null() {
                        if libwebp_sys::DGifGetExtensionNext(gif, &mut data) == 0 {
                            // goto end
                            return Err(ImageError::FormatError(
                                "fail to get Gif Extension Next".to_string(),
                            ));
                        }
                    }
                }
                libwebp_sys::GifRecordType_TERMINATE_RECORD_TYPE => {
                    done = 1;
                }
                _ => {
                    return Err(ImageError::FormatError(
                        "unknown gif record type".to_string(),
                    ));
                }
            }
            if done == 1 {
                break;
            }
        }

        libwebp_sys::WebPAnimEncoderAdd(enc, ptr::null_mut(), frame_timestamp, ptr::null_mut());
        if libwebp_sys::WebPAnimEncoderAssemble(enc, webp_data) == 0 {
            return Err(ImageError::TranformError(
                "fail to WebPAnimEncoderAssemble".to_string(),
            ));
        }
        if loop_compatibility != 0 {
            if stored_loop_count != 0 {
                // if no loop-count element is seen, the default is '1' (loop-once)
                // and we need to signal it explicitly in WebP. Note however that
                // in case there's a single frame, we still don't need to store it.
                if frame_number > 1 {
                    stored_loop_count = 1;
                    loop_count = 1;
                }
            } else if loop_count > 0 && loop_count < 65535 {
                // adapt GIF's semantic to WebP's (except in the infinite-loop case)
                loop_count += 1;
            }
        }
        // loop_count of 0 is the default (infinite), so no need to signal it
        if loop_count == 0 && !p.first_frame {
            stored_loop_count = 0;
        }
        if stored_loop_count == 0 {
            // Re-mux to add loop count and/or metadata as needed.
            mux =
                libwebp_sys::WebPMuxCreateInternal(webp_data, 1, libwebp_sys::WEBP_MUX_ABI_VERSION);
            if mux.is_null() {
                return Err(ImageError::TranformError(
                    "fail to WebPMuxCreate".to_string(),
                ));
            }
            libwebp_sys::PWebPDataClear(webp_data);

            if stored_loop_count == 0 {
                let mut new_params: libwebp_sys::WebPMuxAnimParams = Default::default();
                let mut err = libwebp_sys::WebPMuxGetAnimationParams(mux, &mut new_params);
                if err != libwebp_sys::WebPMuxError_WEBP_MUX_OK {
                    return Err(ImageError::TranformError(
                        "fail to WebPMuxGetAnimationParams".to_string(),
                    ));
                }
                new_params.loop_count = loop_count;
                err = libwebp_sys::WebPMuxSetAnimationParams(mux, &mut new_params);
                if err != libwebp_sys::WebPMuxError_WEBP_MUX_OK {
                    return Err(ImageError::TranformError(
                        "fail to WebPMuxSetAnimationParams".to_string(),
                    ));
                }

                err = libwebp_sys::WebPMuxAssemble(mux, webp_data);
                if err != libwebp_sys::WebPMuxError_WEBP_MUX_OK {
                    return Err(ImageError::TranformError(
                        "fail to WebPMuxAssemble".to_string(),
                    ));
                }
            }
        }
        image_result.pic = Vec::from_raw_parts(
            (*webp_data).bytes as *mut _,
            (*webp_data).size,
            (*webp_data).size,
        )
        .clone();
        return Ok(image_result);
    }
}
