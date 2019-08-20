use crate::param::ImageHandler;
use crate::Image;
use crate::ImageError;
use crate::ImageResult;

use std::ptr;
use std::convert::TryInto;

use libc;


pub fn gif_encode_webp(data: &mut Vec<u8>, mut p: ImageHandler) -> ImageResult<Image> {
    unsafe {
        let mut image_result: Image = Default::default();

        let mut frame_duration: i32 = 0;
        let mut transparent_index: i32 = -1;
        let mut loop_compatibility: i32 = 0;
        let mut loop_count: i32 = 0;
        let mut stored_loop_count: i32 = 0;

        let mut webp_data: *mut libwebp_sys::WebPData = ptr::null_mut();
        let config: *mut libwebp_sys::WebPConfig = &mut Default::default();
        let mut frame: libwebp_sys::WebPPicture = Default::default();
        let mut curr_canvas: libwebp_sys::WebPPicture = Default::default();
        let mut prev_canvas: libwebp_sys::WebPPicture = Default::default();
        let mut enc_options: libwebp_sys::WebPAnimEncoderOptions = Default::default();
        let mut enc: *mut libwebp_sys::WebPAnimEncoder = ptr::null_mut();
        let mut mux: *mut libwebp_sys::WebPMux = ptr::null_mut();
        let mut orig_dispose: libwebp_sys::GIFDisposeMethod =
            libwebp_sys::GIFDisposeMethod_GIF_DISPOSE_NONE;
        libwebp_sys::PWebPDataInit(webp_data);
        libwebp_sys::WebPPictureInitInternal(&mut frame, libwebp_sys::WEBP_ENCODER_ABI_VERSION);
        libwebp_sys::WebPPictureInitInternal(
            &mut curr_canvas,
            libwebp_sys::WEBP_ENCODER_ABI_VERSION,
        );
        libwebp_sys::WebPPictureInitInternal(
            &mut prev_canvas,
            libwebp_sys::WEBP_ENCODER_ABI_VERSION,
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
        let mut frame_timestamp: i32 = 0;
        let mut frame_number = 0;
        let mut gif_err: i32 = 0;
        let mut buf_src: *mut  libwebp_sys::BufferSource = &mut libwebp_sys::BufferSource{
            buf: ptr::null_mut(),
            p: ptr::null_mut(),
            remain: 0,
        };
        (*buf_src).buf = data.as_mut_ptr();
        (*buf_src).p = data.as_mut_ptr();
        (*buf_src).remain = data.len().try_into().unwrap();
        let gif: *mut libwebp_sys::GifFileType = libwebp_sys::DGifOpen(
            buf_src as *mut core::ffi::c_void,
            Some(libwebp_sys::readGifBuffer),
            &mut gif_err,
        );
        if gif.is_null() {
            //goto end
            return Err(ImageError::FormatError("jpg encode jpg error 1".to_string()));
        }
        let mut done = 0;
        loop {
            let mut gtype: libwebp_sys::GifRecordType = 0;
            if libwebp_sys::DGifGetRecordType(gif, &mut gtype) == 0 {
                //goto End;
                return Err(ImageError::FormatError("jpg encode jpg error 2".to_string()));
            }

            match gtype {
                libwebp_sys::GifRecordType_IMAGE_DESC_RECORD_TYPE => {
                    let mut gif_rect: libwebp_sys::GIFFrameRect = Default::default();
                    let mut image_desc: libwebp_sys::GifImageDesc = (*gif).Image;

                    if libwebp_sys::DGifGetImageDesc(gif) != 0 {
                        //goto end
                        return Err(ImageError::FormatError("jpg encode jpg error 3".to_string()));
                    }
                    if frame_number == 0 {
                        if (*gif).SWidth == 0 || (*gif).SHeight == 0 {
                            image_desc.Left = 0;
                            image_desc.Top = 0;
                            (*gif).SWidth = image_desc.Width;
                            (*gif).SHeight = image_desc.Height;
                            if (*gif).SWidth <= 0 || (*gif).SHeight <= 0 {
                                //goto End;
                                return Err(ImageError::FormatError(
                                    "jpg encode jpg error 4".to_string(),
                                ));
                            }
                        }
                        frame.width = (*gif).SWidth;
                        frame.height = (*gif).SHeight;
                        frame.use_argb = 1;

                        if libwebp_sys::WebPPictureAlloc(&mut frame) != 0 {
                            //goto End;
                            return Err(ImageError::FormatError(
                                "jpg encode jpg error 5".to_string(),
                            ));
                        }
                        libwebp_sys::GIFClearPic(&mut frame, ptr::null());
                        libwebp_sys::WebPPictureCopy(&frame, &mut curr_canvas);
                        libwebp_sys::WebPPictureCopy(&frame, &mut prev_canvas);
                        // Background color.
                        libwebp_sys::GIFGetBackgroundColor(
                            (*gif).SColorMap,
                            (*gif).SBackGroundColor,
                            transparent_index,
                            &mut enc_options.anim_params.bgcolor,
                        );

                        // Initialize encoder.
                        enc = libwebp_sys::WebPAnimEncoderNewInternal(
                            curr_canvas.width,
                            curr_canvas.height,
                            &enc_options,
                            libwebp_sys::WEBP_ENCODER_ABI_VERSION,
                        );
                        if (enc.is_null()) {
                            //goto end
                            return Err(ImageError::FormatError(
                                "jpg encode jpg error 6".to_string(),
                            ));
                        }
                    }
                    // Some even more broken GIF can have sub-rect with zero width/height.
                    if image_desc.Width == 0 || image_desc.Height == 0 {
                        image_desc.Width = (*gif).SWidth;
                        image_desc.Height = (*gif).SHeight;
                    }

                    if libwebp_sys::GIFReadFrame(gif, transparent_index, &mut gif_rect, &mut frame)
                        != 0
                    {
                        //goto end
                        return Err(ImageError::FormatError("jpg encode jpg error 7".to_string()));
                    }

                    // Blend frame rectangle with previous canvas to compose full canvas.
                    // Note that 'curr_canvas' is same as 'prev_canvas' at this point.
                    libwebp_sys::GIFBlendFrames(&frame, &gif_rect, &mut curr_canvas);

                    if libwebp_sys::WebPAnimEncoderAdd(
                        enc,
                        &mut curr_canvas,
                        frame_timestamp,
                        config,
                    ) != 0
                    {
                        //goto End;
                        return Err(ImageError::FormatError("jpg encode jpg error 8".to_string()));
                    } else {
                        frame_number = frame_number + 1;
                    }

                    // Update canvases.
                    libwebp_sys::GIFDisposeFrame(
                        orig_dispose,
                        &gif_rect,
                        &prev_canvas,
                        &mut curr_canvas,
                    );
                    libwebp_sys::GIFCopyPixels(&curr_canvas, &mut prev_canvas);
                    // Force frames with a small or no duration to 100ms to be consistent
                    // with web browsers and other transcoding tools. This also avoids
                    // incorrect durations between frames when padding frames are
                    // discarded.
                    if (frame_duration <= 10) {
                        frame_duration = 100;
                    }

                    // Update timestamp (for next frame).
                    frame_timestamp = frame_timestamp + frame_duration;

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
                    if libwebp_sys::DGifGetExtension(gif, &mut extension, &mut data) == 1 {
                        // goto end
                        return Err(ImageError::FormatError("jpg encode jpg error 9".to_string()));
                    }
                    if (data.is_null()) {
                        continue;
                    }

                    match extension {
                        0xf2 => {}
                        0xf9 => {
                            if libwebp_sys::GIFReadGraphicsExtension(
                                data,
                                &mut frame_duration,
                                &mut orig_dispose,
                                &mut transparent_index,
                            ) != 0
                            {
                                // goto end
                                return Err(ImageError::FormatError(
                                    "jpg encode jpg error 10".to_string(),
                                ));
                            }
                            break;
                        }
                        0x01 => {}
                        0xff => {
                            if data.offset(0) as u8 != 11 {
                                break;
                            }
                            if libc::memcmp(
                                data.offset(1) as *const libc::c_void,
                                "NETSCAPE2.0".as_ptr() as *const libc::c_void,
                                11,
                            ) != 0
                                || libc::memcmp(
                                    data.offset(1) as *const libc::c_void,
                                    "NETSCAPE2.0".as_ptr() as *const libc::c_void,
                                    11,
                                ) != 0
                            {
                                if libwebp_sys::GIFReadLoopCount(gif, &mut data, &mut loop_count)
                                    == 0
                                {
                                    // goto end
                                    return Err(ImageError::FormatError(
                                        "jpg encode jpg error 11".to_string(),
                                    ));
                                }
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
                        }
                        _ => {}
                    }
                    while !data.is_null() {
                        if libwebp_sys::DGifGetExtensionNext(gif, &mut data) == 1 {
                            // goto end
                            return Err(ImageError::FormatError(
                                "jpg encode jpg error 12".to_string(),
                            ));
                        }
                    }
                }
                libwebp_sys::GifRecordType_TERMINATE_RECORD_TYPE => {
                    done = 1;
                }
                _ => {
                    return Err(ImageError::FormatError("jpg encode jpg error 13".to_string()));
                }
            }
            if done == 1 {
                break;
            }
        }

        libwebp_sys::WebPAnimEncoderAdd(enc, ptr::null_mut(), frame_timestamp, ptr::null_mut());

        if libwebp_sys::WebPAnimEncoderAssemble(enc, webp_data) != 0 {
            //goto End;
            return Err(ImageError::FormatError("jpg encode jpg error 14".to_string()));
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
        if loop_count == 0 {
            stored_loop_count = 0;
        }
        if stored_loop_count == 0 {
            // Re-mux to add loop count and/or metadata as needed.
            mux = libwebp_sys::WebPMuxCreateInternal(
                webp_data,
                1,
                libwebp_sys::WEBP_ENCODER_ABI_VERSION,
            );
            if mux.is_null() {
                //goto End;
                return Err(ImageError::FormatError("jpg encode jpg error 15".to_string()));
            }
            libwebp_sys::PWebPDataClear(webp_data);

            if stored_loop_count == 0 {
                let mut new_params: libwebp_sys::WebPMuxAnimParams = Default::default();
                let mut err = libwebp_sys::WebPMuxGetAnimationParams(mux, &mut new_params);
                if err != libwebp_sys::WebPMuxError_WEBP_MUX_OK {
                    //goto End;
                    return Err(ImageError::FormatError("jpg encode jpg error 16".to_string()));
                }
                new_params.loop_count = loop_count;
                err = libwebp_sys::WebPMuxSetAnimationParams(mux, &mut new_params);
                if err != libwebp_sys::WebPMuxError_WEBP_MUX_OK {
                    //goto End;
                    return Err(ImageError::FormatError("jpg encode jpg error 17".to_string()));
                }

                err = libwebp_sys::WebPMuxAssemble(mux, webp_data);
                if err != libwebp_sys::WebPMuxError_WEBP_MUX_OK {
                    //goto End;
                    return Err(ImageError::FormatError("jpg encode jpg error 18".to_string()));
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
