/* automatically generated by rust-bindgen */

pub type wchar_t = libc :: c_int ; pub type max_align_t = f64 ; pub type __int8_t = libc :: c_schar ; pub type __uint8_t = libc :: c_uchar ; pub type __int16_t = libc :: c_short ; pub type __uint16_t = libc :: c_ushort ; pub type __int32_t = libc :: c_int ; pub type __uint32_t = libc :: c_uint ; pub type __int64_t = libc :: c_longlong ; pub type __uint64_t = libc :: c_ulonglong ; pub type __darwin_intptr_t = libc :: c_long ; pub type __darwin_natural_t = libc :: c_uint ; pub type __darwin_ct_rune_t = libc :: c_int ; # [ repr ( C ) ] # [ derive ( Copy , Clone ) ] pub union __mbstate_t { pub __mbstate8 : [ libc :: c_char ; 128usize ] , pub _mbstateL : libc :: c_longlong , _bindgen_union_align : [ u64 ; 16usize ] , } impl Default for __mbstate_t { fn default ( ) -> Self { unsafe { :: std :: mem :: zeroed ( ) } } } pub type __darwin_mbstate_t = __mbstate_t ; pub type __darwin_ptrdiff_t = libc :: c_long ; pub type __darwin_size_t = libc :: c_ulong ; pub type __darwin_va_list = __builtin_va_list ; pub type __darwin_wchar_t = libc :: c_int ; pub type __darwin_rune_t = __darwin_wchar_t ; pub type __darwin_wint_t = libc :: c_int ; pub type __darwin_clock_t = libc :: c_ulong ; pub type __darwin_socklen_t = __uint32_t ; pub type __darwin_ssize_t = libc :: c_long ; pub type __darwin_time_t = libc :: c_long ; pub type __darwin_blkcnt_t = __int64_t ; pub type __darwin_blksize_t = __int32_t ; pub type __darwin_dev_t = __int32_t ; pub type __darwin_fsblkcnt_t = libc :: c_uint ; pub type __darwin_fsfilcnt_t = libc :: c_uint ; pub type __darwin_gid_t = __uint32_t ; pub type __darwin_id_t = __uint32_t ; pub type __darwin_ino64_t = __uint64_t ; pub type __darwin_ino_t = __darwin_ino64_t ; pub type __darwin_mach_port_name_t = __darwin_natural_t ; pub type __darwin_mach_port_t = __darwin_mach_port_name_t ; pub type __darwin_mode_t = __uint16_t ; pub type __darwin_off_t = __int64_t ; pub type __darwin_pid_t = __int32_t ; pub type __darwin_sigset_t = __uint32_t ; pub type __darwin_suseconds_t = __int32_t ; pub type __darwin_uid_t = __uint32_t ; pub type __darwin_useconds_t = __uint32_t ; pub type __darwin_uuid_t = [ libc :: c_uchar ; 16usize ] ; pub type __darwin_uuid_string_t = [ libc :: c_char ; 37usize ] ; # [ repr ( C ) ] # [ derive ( Debug , Copy , Clone ) ] pub struct __darwin_pthread_handler_rec { pub __routine : :: std :: option :: Option < unsafe extern "C" fn ( arg1 : * mut libc :: c_void ) > , pub __arg : * mut libc :: c_void , pub __next : * mut __darwin_pthread_handler_rec , } impl Default for __darwin_pthread_handler_rec { fn default ( ) -> Self { unsafe { :: std :: mem :: zeroed ( ) } } } # [ repr ( C ) ] # [ derive ( Copy , Clone ) ] pub struct _opaque_pthread_attr_t { pub __sig : libc :: c_long , pub __opaque : [ libc :: c_char ; 56usize ] , } impl Default for _opaque_pthread_attr_t { fn default ( ) -> Self { unsafe { :: std :: mem :: zeroed ( ) } } } # [ repr ( C ) ] # [ derive ( Copy , Clone ) ] pub struct _opaque_pthread_cond_t { pub __sig : libc :: c_long , pub __opaque : [ libc :: c_char ; 40usize ] , } impl Default for _opaque_pthread_cond_t { fn default ( ) -> Self { unsafe { :: std :: mem :: zeroed ( ) } } } # [ repr ( C ) ] # [ derive ( Debug , Default , Copy , Clone ) ] pub struct _opaque_pthread_condattr_t { pub __sig : libc :: c_long , pub __opaque : [ libc :: c_char ; 8usize ] , } # [ repr ( C ) ] # [ derive ( Copy , Clone ) ] pub struct _opaque_pthread_mutex_t { pub __sig : libc :: c_long , pub __opaque : [ libc :: c_char ; 56usize ] , } impl Default for _opaque_pthread_mutex_t { fn default ( ) -> Self { unsafe { :: std :: mem :: zeroed ( ) } } } # [ repr ( C ) ] # [ derive ( Debug , Default , Copy , Clone ) ] pub struct _opaque_pthread_mutexattr_t { pub __sig : libc :: c_long , pub __opaque : [ libc :: c_char ; 8usize ] , } # [ repr ( C ) ] # [ derive ( Debug , Default , Copy , Clone ) ] pub struct _opaque_pthread_once_t { pub __sig : libc :: c_long , pub __opaque : [ libc :: c_char ; 8usize ] , } # [ repr ( C ) ] # [ derive ( Copy , Clone ) ] pub struct _opaque_pthread_rwlock_t { pub __sig : libc :: c_long , pub __opaque : [ libc :: c_char ; 192usize ] , } impl Default for _opaque_pthread_rwlock_t { fn default ( ) -> Self { unsafe { :: std :: mem :: zeroed ( ) } } } # [ repr ( C ) ] # [ derive ( Debug , Default , Copy , Clone ) ] pub struct _opaque_pthread_rwlockattr_t { pub __sig : libc :: c_long , pub __opaque : [ libc :: c_char ; 16usize ] , } # [ repr ( C ) ] # [ derive ( Copy , Clone ) ] pub struct _opaque_pthread_t { pub __sig : libc :: c_long , pub __cleanup_stack : * mut __darwin_pthread_handler_rec , pub __opaque : [ libc :: c_char ; 8176usize ] , } impl Default for _opaque_pthread_t { fn default ( ) -> Self { unsafe { :: std :: mem :: zeroed ( ) } } } pub type __darwin_pthread_attr_t = _opaque_pthread_attr_t ; pub type __darwin_pthread_cond_t = _opaque_pthread_cond_t ; pub type __darwin_pthread_condattr_t = _opaque_pthread_condattr_t ; pub type __darwin_pthread_key_t = libc :: c_ulong ; pub type __darwin_pthread_mutex_t = _opaque_pthread_mutex_t ; pub type __darwin_pthread_mutexattr_t = _opaque_pthread_mutexattr_t ; pub type __darwin_pthread_once_t = _opaque_pthread_once_t ; pub type __darwin_pthread_rwlock_t = _opaque_pthread_rwlock_t ; pub type __darwin_pthread_rwlockattr_t = _opaque_pthread_rwlockattr_t ; pub type __darwin_pthread_t = * mut _opaque_pthread_t ; pub type __darwin_nl_item = libc :: c_int ; pub type __darwin_wctrans_t = libc :: c_int ; pub type __darwin_wctype_t = __uint32_t ; pub type int_least8_t = i8 ; pub type int_least16_t = i16 ; pub type int_least32_t = i32 ; pub type int_least64_t = i64 ; pub type uint_least8_t = u8 ; pub type uint_least16_t = u16 ; pub type uint_least32_t = u32 ; pub type uint_least64_t = u64 ; pub type int_fast8_t = i8 ; pub type int_fast16_t = i16 ; pub type int_fast32_t = i32 ; pub type int_fast64_t = i64 ; pub type uint_fast8_t = u8 ; pub type uint_fast16_t = u16 ; pub type uint_fast32_t = u32 ; pub type uint_fast64_t = u64 ; pub type u_int8_t = libc :: c_uchar ; pub type u_int16_t = libc :: c_ushort ; pub type u_int32_t = libc :: c_uint ; pub type u_int64_t = libc :: c_ulonglong ; pub type register_t = i64 ; pub type user_addr_t = u_int64_t ; pub type user_size_t = u_int64_t ; pub type user_ssize_t = i64 ; pub type user_long_t = i64 ; pub type user_ulong_t = u_int64_t ; pub type user_time_t = i64 ; pub type user_off_t = i64 ; pub type syscall_arg_t = u_int64_t ; pub type intmax_t = libc :: c_long ; pub type uintmax_t = libc :: c_ulong ; extern "C" { pub fn imaxabs ( j : intmax_t ) -> intmax_t ; } # [ repr ( C ) ] # [ derive ( Debug , Default , Copy , Clone ) ] pub struct imaxdiv_t { pub quot : intmax_t , pub rem : intmax_t , } extern "C" { pub fn imaxdiv ( __numer : intmax_t , __denom : intmax_t ) -> imaxdiv_t ; } extern "C" { pub fn strtoimax ( __nptr : * const libc :: c_char , __endptr : * mut * mut libc :: c_char , __base : libc :: c_int ) -> intmax_t ; } extern "C" { pub fn strtoumax ( __nptr : * const libc :: c_char , __endptr : * mut * mut libc :: c_char , __base : libc :: c_int ) -> uintmax_t ; } extern "C" { pub fn wcstoimax ( __nptr : * const wchar_t , __endptr : * mut * mut wchar_t , __base : libc :: c_int ) -> intmax_t ; } extern "C" { pub fn wcstoumax ( __nptr : * const wchar_t , __endptr : * mut * mut wchar_t , __base : libc :: c_int ) -> uintmax_t ; } extern "C" { pub fn WebPGetEncoderVersion ( ) -> libc :: c_int ; } extern "C" { pub fn WebPEncodeRGB ( rgb : * const u8 , width : libc :: c_int , height : libc :: c_int , stride : libc :: c_int , quality_factor : f32 , output : * mut * mut u8 ) -> usize ; } extern "C" { pub fn WebPEncodeBGR ( bgr : * const u8 , width : libc :: c_int , height : libc :: c_int , stride : libc :: c_int , quality_factor : f32 , output : * mut * mut u8 ) -> usize ; } extern "C" { pub fn WebPEncodeRGBA ( rgba : * const u8 , width : libc :: c_int , height : libc :: c_int , stride : libc :: c_int , quality_factor : f32 , output : * mut * mut u8 ) -> usize ; } extern "C" { pub fn WebPEncodeBGRA ( bgra : * const u8 , width : libc :: c_int , height : libc :: c_int , stride : libc :: c_int , quality_factor : f32 , output : * mut * mut u8 ) -> usize ; } extern "C" { pub fn WebPEncodeLosslessRGB ( rgb : * const u8 , width : libc :: c_int , height : libc :: c_int , stride : libc :: c_int , output : * mut * mut u8 ) -> usize ; } extern "C" { pub fn WebPEncodeLosslessBGR ( bgr : * const u8 , width : libc :: c_int , height : libc :: c_int , stride : libc :: c_int , output : * mut * mut u8 ) -> usize ; } extern "C" { pub fn WebPEncodeLosslessRGBA ( rgba : * const u8 , width : libc :: c_int , height : libc :: c_int , stride : libc :: c_int , output : * mut * mut u8 ) -> usize ; } extern "C" { pub fn WebPEncodeLosslessBGRA ( bgra : * const u8 , width : libc :: c_int , height : libc :: c_int , stride : libc :: c_int , output : * mut * mut u8 ) -> usize ; } extern "C" { pub fn WebPFree ( ptr : * mut libc :: c_void ) ; } pub const WebPImageHint_WEBP_HINT_DEFAULT : WebPImageHint = 0 ; pub const WebPImageHint_WEBP_HINT_PICTURE : WebPImageHint = 1 ; pub const WebPImageHint_WEBP_HINT_PHOTO : WebPImageHint = 2 ; pub const WebPImageHint_WEBP_HINT_GRAPH : WebPImageHint = 3 ; pub const WebPImageHint_WEBP_HINT_LAST : WebPImageHint = 4 ; pub type WebPImageHint = u32 ; # [ repr ( C ) ] # [ derive ( Debug , Copy , Clone ) ] pub struct WebPConfig { pub lossless : libc :: c_int , pub quality : f32 , pub method : libc :: c_int , pub image_hint : WebPImageHint , pub target_size : libc :: c_int , pub target_PSNR : f32 , pub segments : libc :: c_int , pub sns_strength : libc :: c_int , pub filter_strength : libc :: c_int , pub filter_sharpness : libc :: c_int , pub filter_type : libc :: c_int , pub autofilter : libc :: c_int , pub alpha_compression : libc :: c_int , pub alpha_filtering : libc :: c_int , pub alpha_quality : libc :: c_int , pub pass : libc :: c_int , pub show_compressed : libc :: c_int , pub preprocessing : libc :: c_int , pub partitions : libc :: c_int , pub partition_limit : libc :: c_int , pub emulate_jpeg_size : libc :: c_int , pub thread_level : libc :: c_int , pub low_memory : libc :: c_int , pub near_lossless : libc :: c_int , pub exact : libc :: c_int , pub use_delta_palette : libc :: c_int , pub use_sharp_yuv : libc :: c_int , pub pad : [ u32 ; 2usize ] , } impl Default for WebPConfig { fn default ( ) -> Self { unsafe { :: std :: mem :: zeroed ( ) } } } pub const WebPPreset_WEBP_PRESET_DEFAULT : WebPPreset = 0 ; pub const WebPPreset_WEBP_PRESET_PICTURE : WebPPreset = 1 ; pub const WebPPreset_WEBP_PRESET_PHOTO : WebPPreset = 2 ; pub const WebPPreset_WEBP_PRESET_DRAWING : WebPPreset = 3 ; pub const WebPPreset_WEBP_PRESET_ICON : WebPPreset = 4 ; pub const WebPPreset_WEBP_PRESET_TEXT : WebPPreset = 5 ; pub type WebPPreset = u32 ; extern "C" { pub fn WebPConfigInitInternal ( arg1 : * mut WebPConfig , arg2 : WebPPreset , arg3 : f32 , arg4 : libc :: c_int ) -> libc :: c_int ; } extern "C" { pub fn WebPConfigLosslessPreset ( config : * mut WebPConfig , level : libc :: c_int ) -> libc :: c_int ; } extern "C" { pub fn WebPValidateConfig ( config : * const WebPConfig ) -> libc :: c_int ; } # [ repr ( C ) ] # [ derive ( Debug , Default , Copy , Clone ) ] pub struct WebPAuxStats { pub coded_size : libc :: c_int , pub PSNR : [ f32 ; 5usize ] , pub block_count : [ libc :: c_int ; 3usize ] , pub header_bytes : [ libc :: c_int ; 2usize ] , pub residual_bytes : [ [ libc :: c_int ; 4usize ] ; 3usize ] , pub segment_size : [ libc :: c_int ; 4usize ] , pub segment_quant : [ libc :: c_int ; 4usize ] , pub segment_level : [ libc :: c_int ; 4usize ] , pub alpha_data_size : libc :: c_int , pub layer_data_size : libc :: c_int , pub lossless_features : u32 , pub histogram_bits : libc :: c_int , pub transform_bits : libc :: c_int , pub cache_bits : libc :: c_int , pub palette_size : libc :: c_int , pub lossless_size : libc :: c_int , pub lossless_hdr_size : libc :: c_int , pub lossless_data_size : libc :: c_int , pub pad : [ u32 ; 2usize ] , } pub type WebPWriterFunction = :: std :: option :: Option < unsafe extern "C" fn ( data : * const u8 , data_size : usize , picture : * const WebPPicture ) -> libc :: c_int > ; # [ repr ( C ) ] # [ derive ( Debug , Copy , Clone ) ] pub struct WebPMemoryWriter { pub mem : * mut u8 , pub size : usize , pub max_size : usize , pub pad : [ u32 ; 1usize ] , } impl Default for WebPMemoryWriter { fn default ( ) -> Self { unsafe { :: std :: mem :: zeroed ( ) } } } extern "C" { pub fn WebPMemoryWriterInit ( writer : * mut WebPMemoryWriter ) ; } extern "C" { pub fn WebPMemoryWriterClear ( writer : * mut WebPMemoryWriter ) ; } extern "C" { pub fn WebPMemoryWrite ( data : * const u8 , data_size : usize , picture : * const WebPPicture ) -> libc :: c_int ; } pub type WebPProgressHook = :: std :: option :: Option < unsafe extern "C" fn ( percent : libc :: c_int , picture : * const WebPPicture ) -> libc :: c_int > ; pub const WebPEncCSP_WEBP_YUV420 : WebPEncCSP = 0 ; pub const WebPEncCSP_WEBP_YUV420A : WebPEncCSP = 4 ; pub const WebPEncCSP_WEBP_CSP_UV_MASK : WebPEncCSP = 3 ; pub const WebPEncCSP_WEBP_CSP_ALPHA_BIT : WebPEncCSP = 4 ; pub type WebPEncCSP = u32 ; pub const WebPEncodingError_VP8_ENC_OK : WebPEncodingError = 0 ; pub const WebPEncodingError_VP8_ENC_ERROR_OUT_OF_MEMORY : WebPEncodingError = 1 ; pub const WebPEncodingError_VP8_ENC_ERROR_BITSTREAM_OUT_OF_MEMORY : WebPEncodingError = 2 ; pub const WebPEncodingError_VP8_ENC_ERROR_NULL_PARAMETER : WebPEncodingError = 3 ; pub const WebPEncodingError_VP8_ENC_ERROR_INVALID_CONFIGURATION : WebPEncodingError = 4 ; pub const WebPEncodingError_VP8_ENC_ERROR_BAD_DIMENSION : WebPEncodingError = 5 ; pub const WebPEncodingError_VP8_ENC_ERROR_PARTITION0_OVERFLOW : WebPEncodingError = 6 ; pub const WebPEncodingError_VP8_ENC_ERROR_PARTITION_OVERFLOW : WebPEncodingError = 7 ; pub const WebPEncodingError_VP8_ENC_ERROR_BAD_WRITE : WebPEncodingError = 8 ; pub const WebPEncodingError_VP8_ENC_ERROR_FILE_TOO_BIG : WebPEncodingError = 9 ; pub const WebPEncodingError_VP8_ENC_ERROR_USER_ABORT : WebPEncodingError = 10 ; pub const WebPEncodingError_VP8_ENC_ERROR_LAST : WebPEncodingError = 11 ; pub type WebPEncodingError = u32 ; # [ repr ( C ) ] # [ derive ( Debug , Copy , Clone ) ] pub struct WebPPicture { # [ doc = "" ] pub use_argb : libc :: c_int , pub colorspace : WebPEncCSP , pub width : libc :: c_int , pub height : libc :: c_int , pub y : * mut u8 , pub u : * mut u8 , pub v : * mut u8 , pub y_stride : libc :: c_int , pub uv_stride : libc :: c_int , pub a : * mut u8 , pub a_stride : libc :: c_int , pub pad1 : [ u32 ; 2usize ] , pub argb : * mut u32 , pub argb_stride : libc :: c_int , pub pad2 : [ u32 ; 3usize ] , # [ doc = "" ] pub writer : WebPWriterFunction , pub custom_ptr : * mut libc :: c_void , pub extra_info_type : libc :: c_int , pub extra_info : * mut u8 , # [ doc = "" ] pub stats : * mut WebPAuxStats , pub error_code : WebPEncodingError , pub progress_hook : WebPProgressHook , pub user_data : * mut libc :: c_void , pub pad3 : [ u32 ; 3usize ] , pub pad4 : * mut u8 , pub pad5 : * mut u8 , pub pad6 : [ u32 ; 8usize ] , # [ doc = "" ] pub memory_ : * mut libc :: c_void , pub memory_argb_ : * mut libc :: c_void , pub pad7 : [ * mut libc :: c_void ; 2usize ] , } impl Default for WebPPicture { fn default ( ) -> Self { unsafe { :: std :: mem :: zeroed ( ) } } } extern "C" { pub fn WebPPictureInitInternal ( arg1 : * mut WebPPicture , arg2 : libc :: c_int ) -> libc :: c_int ; } extern "C" { pub fn WebPPictureAlloc ( picture : * mut WebPPicture ) -> libc :: c_int ; } extern "C" { pub fn WebPPictureFree ( picture : * mut WebPPicture ) ; } extern "C" { pub fn WebPPictureCopy ( src : * const WebPPicture , dst : * mut WebPPicture ) -> libc :: c_int ; } extern "C" { pub fn WebPPlaneDistortion ( src : * const u8 , src_stride : usize , ref_ : * const u8 , ref_stride : usize , width : libc :: c_int , height : libc :: c_int , x_step : usize , type_ : libc :: c_int , distortion : * mut f32 , result : * mut f32 ) -> libc :: c_int ; } extern "C" { pub fn WebPPictureDistortion ( src : * const WebPPicture , ref_ : * const WebPPicture , metric_type : libc :: c_int , result : * mut f32 ) -> libc :: c_int ; } extern "C" { pub fn WebPPictureCrop ( picture : * mut WebPPicture , left : libc :: c_int , top : libc :: c_int , width : libc :: c_int , height : libc :: c_int ) -> libc :: c_int ; } extern "C" { pub fn WebPPictureView ( src : * const WebPPicture , left : libc :: c_int , top : libc :: c_int , width : libc :: c_int , height : libc :: c_int , dst : * mut WebPPicture ) -> libc :: c_int ; } extern "C" { pub fn WebPPictureIsView ( picture : * const WebPPicture ) -> libc :: c_int ; } extern "C" { pub fn WebPPictureRescale ( pic : * mut WebPPicture , width : libc :: c_int , height : libc :: c_int ) -> libc :: c_int ; } extern "C" { pub fn WebPPictureImportRGB ( picture : * mut WebPPicture , rgb : * const u8 , rgb_stride : libc :: c_int ) -> libc :: c_int ; } extern "C" { pub fn WebPPictureImportRGBA ( picture : * mut WebPPicture , rgba : * const u8 , rgba_stride : libc :: c_int ) -> libc :: c_int ; } extern "C" { pub fn WebPPictureImportRGBX ( picture : * mut WebPPicture , rgbx : * const u8 , rgbx_stride : libc :: c_int ) -> libc :: c_int ; } extern "C" { pub fn WebPPictureImportBGR ( picture : * mut WebPPicture , bgr : * const u8 , bgr_stride : libc :: c_int ) -> libc :: c_int ; } extern "C" { pub fn WebPPictureImportBGRA ( picture : * mut WebPPicture , bgra : * const u8 , bgra_stride : libc :: c_int ) -> libc :: c_int ; } extern "C" { pub fn WebPPictureImportBGRX ( picture : * mut WebPPicture , bgrx : * const u8 , bgrx_stride : libc :: c_int ) -> libc :: c_int ; } extern "C" { pub fn WebPPictureARGBToYUVA ( picture : * mut WebPPicture , arg1 : WebPEncCSP ) -> libc :: c_int ; } extern "C" { pub fn WebPPictureARGBToYUVADithered ( picture : * mut WebPPicture , colorspace : WebPEncCSP , dithering : f32 ) -> libc :: c_int ; } extern "C" { pub fn WebPPictureSharpARGBToYUVA ( picture : * mut WebPPicture ) -> libc :: c_int ; } extern "C" { pub fn WebPPictureSmartARGBToYUVA ( picture : * mut WebPPicture ) -> libc :: c_int ; } extern "C" { pub fn WebPPictureYUVAToARGB ( picture : * mut WebPPicture ) -> libc :: c_int ; } extern "C" { pub fn WebPCleanupTransparentArea ( picture : * mut WebPPicture ) ; } extern "C" { pub fn WebPPictureHasTransparency ( picture : * const WebPPicture ) -> libc :: c_int ; } extern "C" { pub fn WebPBlendAlpha ( pic : * mut WebPPicture , background_rgb : u32 ) ; } extern "C" { pub fn WebPEncode ( config : * const WebPConfig , picture : * mut WebPPicture ) -> libc :: c_int ; } pub type __builtin_va_list = [ __va_list_tag ; 1usize ] ; # [ repr ( C ) ] # [ derive ( Debug , Copy , Clone ) ] pub struct __va_list_tag { pub gp_offset : libc :: c_uint , pub fp_offset : libc :: c_uint , pub overflow_arg_area : * mut libc :: c_void , pub reg_save_area : * mut libc :: c_void , } impl Default for __va_list_tag { fn default ( ) -> Self { unsafe { :: std :: mem :: zeroed ( ) } } }