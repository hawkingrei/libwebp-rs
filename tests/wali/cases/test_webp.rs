use imagers::ImageHandler;

use crate::case;
use crate::test_config;
use crc::{crc32, Hasher32};
use imagers::webp_encode_webp;
use imagers::Resize;
use std::fs;
use std::path::Path;

fn tranform_webp_to_webp(config: &test_config, case: case<ImageHandler>) -> Result<(), String> {
    let mut input: String = case.input;
    let mut expected: String = case.expected;
    let mut is_corrupted: bool = case.is_corrupted;
    let mut im: ImageHandler = case.param;
    let mut fact_input = config.input.clone();
    fact_input.push_str("webp/");
    fact_input.push_str(input.as_str());
    println!("{}", fact_input);
    let mut fact_output = config.output.clone();
    fact_output.push_str(&expected);
    println!("{}", fact_output);

    let mut fact_expected = config.expected.clone();
    fact_expected.push_str("webp/");
    fact_expected.push_str(&expected);
    println!("{}", fact_expected);

    let data = match fs::read(Path::new(&fact_input)) {
        Ok(d) => d,
        Err(err) => return Err(format!("{} fail to read. {}", fact_input, err)),
    };

    let result = match webp_encode_webp(&data.clone(), im) {
        Ok(result) => result,
        Err(err) => {
            if is_corrupted {
                return Ok(());
            } else {
                return Err(format!("{} fail at webp_encode_webp. {}", input, err));
            }
        }
    };

    let expected_data = match fs::read(Path::new(&fact_expected)) {
        Ok(d) => d,
        Err(err) => return Err(format!("{} fail to read. {}", fact_expected, err)),
    };

    let mut digest_output = crc32::Digest::new(crc32::IEEE);
    let mut digest_expected = crc32::Digest::new(crc32::IEEE);
    digest_expected.write(expected_data.as_slice());
    digest_output.write(result.as_slice());

    fs::write(Path::new(Path::new(&fact_output)), result);

    if digest_expected.sum32() != digest_output.sum32() {
        return Err(format!("{} fail to test", input));
    }
    return Ok(());
}

wali_test!(
    test_alpha_color_cache_100h_100w_1e_1c,
    tranform_webp_to_webp,
    case::<ImageHandler>::new()
        .set_input("alpha_color_cache.webp")
        .set_expected("alpha_color_cache_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandler::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
        )
);
wali_test!(
    test_alpha_filter_0_method_0_100h_100w_1e_1c,
    tranform_webp_to_webp,
    case::<ImageHandler>::new()
        .set_input("alpha_filter_0_method_0.webp")
        .set_expected("alpha_filter_0_method_0_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandler::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
        )
);
wali_test!(
    test_alpha_filter_0_method_1_100h_100w_1e_1c,
    tranform_webp_to_webp,
    case::<ImageHandler>::new()
        .set_input("alpha_filter_0_method_1.webp")
        .set_expected("alpha_filter_0_method_1_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandler::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
        )
);
wali_test!(
    test_alpha_filter_1_100h_100w_1e_1c,
    tranform_webp_to_webp,
    case::<ImageHandler>::new()
        .set_input("alpha_filter_1.webp")
        .set_expected("alpha_filter_1_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandler::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
        )
);
wali_test!(
    test_alpha_filter_1_method_0_100h_100w_1e_1c,
    tranform_webp_to_webp,
    case::<ImageHandler>::new()
        .set_input("alpha_filter_1_method_0.webp")
        .set_expected("alpha_filter_1_method_0_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandler::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
        )
);
wali_test!(
    test_alpha_filter_1_method_1_100h_100w_1e_1c,
    tranform_webp_to_webp,
    case::<ImageHandler>::new()
        .set_input("alpha_filter_1_method_1.webp")
        .set_expected("alpha_filter_1_method_1_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandler::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
        )
);
wali_test!(
    test_alpha_filter_2_100h_100w_1e_1c,
    tranform_webp_to_webp,
    case::<ImageHandler>::new()
        .set_input("alpha_filter_2.webp")
        .set_expected("alpha_filter_2_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandler::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
        )
);
wali_test!(
    test_alpha_filter_2_method_0_100h_100w_1e_1c,
    tranform_webp_to_webp,
    case::<ImageHandler>::new()
        .set_input("alpha_filter_2_method_0.webp")
        .set_expected("alpha_filter_2_method_0_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandler::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
        )
);
wali_test!(
    test_alpha_filter_2_method_1_100h_100w_1e_1c,
    tranform_webp_to_webp,
    case::<ImageHandler>::new()
        .set_input("alpha_filter_2_method_1.webp")
        .set_expected("alpha_filter_2_method_1_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandler::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
        )
);
wali_test!(
    test_alpha_filter_3_100h_100w_1e_1c,
    tranform_webp_to_webp,
    case::<ImageHandler>::new()
        .set_input("alpha_filter_3.webp")
        .set_expected("alpha_filter_3_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandler::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
        )
);
wali_test!(
    test_alpha_filter_3_method_0_100h_100w_1e_1c,
    tranform_webp_to_webp,
    case::<ImageHandler>::new()
        .set_input("alpha_filter_3_method_0.webp")
        .set_expected("alpha_filter_3_method_0_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandler::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
        )
);
wali_test!(
    test_alpha_filter_3_method_1_100h_100w_1e_1c,
    tranform_webp_to_webp,
    case::<ImageHandler>::new()
        .set_input("alpha_filter_3_method_1.webp")
        .set_expected("alpha_filter_3_method_1_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandler::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
        )
);
wali_test!(
    test_alpha_no_compression_100h_100w_1e_1c,
    tranform_webp_to_webp,
    case::<ImageHandler>::new()
        .set_input("alpha_no_compression.webp")
        .set_expected("alpha_no_compression_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandler::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
        )
);
wali_test!(
    test_bad_palette_index_100h_100w_1e_1c,
    tranform_webp_to_webp,
    case::<ImageHandler>::new()
        .set_input("bad_palette_index.webp")
        .set_expected("bad_palette_index_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandler::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
        )
);
wali_test!(
    test_big_endian_bug_393_100h_100w_1e_1c,
    tranform_webp_to_webp,
    case::<ImageHandler>::new()
        .set_input("big_endian_bug_393.webp")
        .set_expected("big_endian_bug_393_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandler::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
        )
);
wali_test!(
    test_bryce_100h_100w_1e_1c,
    tranform_webp_to_webp,
    case::<ImageHandler>::new()
        .set_input("bryce.webp")
        .set_expected("bryce_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandler::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
        )
);
wali_test!(
    test_bug3_100h_100w_1e_1c,
    tranform_webp_to_webp,
    case::<ImageHandler>::new()
        .set_input("bug3.webp")
        .set_expected("bug3_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandler::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
        )
);
wali_test!(
    test_color_cache_bits_11_100h_100w_1e_1c,
    tranform_webp_to_webp,
    case::<ImageHandler>::new()
        .set_input("color_cache_bits_11.webp")
        .set_expected("color_cache_bits_11_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandler::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
        )
);
wali_test!(
    test_lossless1_100h_100w_1e_1c,
    tranform_webp_to_webp,
    case::<ImageHandler>::new()
        .set_input("lossless1.webp")
        .set_expected("lossless1_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandler::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
        )
);
wali_test!(
    test_lossless2_100h_100w_1e_1c,
    tranform_webp_to_webp,
    case::<ImageHandler>::new()
        .set_input("lossless2.webp")
        .set_expected("lossless2_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandler::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
        )
);
wali_test!(
    test_lossless3_100h_100w_1e_1c,
    tranform_webp_to_webp,
    case::<ImageHandler>::new()
        .set_input("lossless3.webp")
        .set_expected("lossless3_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandler::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
        )
);
wali_test!(
    test_lossless4_100h_100w_1e_1c,
    tranform_webp_to_webp,
    case::<ImageHandler>::new()
        .set_input("lossless4.webp")
        .set_expected("lossless4_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandler::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
        )
);
wali_test!(
    test_lossless_big_random_alpha_100h_100w_1e_1c,
    tranform_webp_to_webp,
    case::<ImageHandler>::new()
        .set_input("lossless_big_random_alpha.webp")
        .set_expected("lossless_big_random_alpha_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandler::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
        )
);
wali_test!(
    test_lossless_color_transform_100h_100w_1e_1c,
    tranform_webp_to_webp,
    case::<ImageHandler>::new()
        .set_input("lossless_color_transform.webp")
        .set_expected("lossless_color_transform_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandler::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
        )
);
wali_test!(
    test_lossless_vec_1_0_100h_100w_1e_1c,
    tranform_webp_to_webp,
    case::<ImageHandler>::new()
        .set_input("lossless_vec_1_0.webp")
        .set_expected("lossless_vec_1_0_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandler::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
        )
);
wali_test!(
    test_lossless_vec_1_1_100h_100w_1e_1c,
    tranform_webp_to_webp,
    case::<ImageHandler>::new()
        .set_input("lossless_vec_1_1.webp")
        .set_expected("lossless_vec_1_1_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandler::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
        )
);
wali_test!(
    test_lossless_vec_1_10_100h_100w_1e_1c,
    tranform_webp_to_webp,
    case::<ImageHandler>::new()
        .set_input("lossless_vec_1_10.webp")
        .set_expected("lossless_vec_1_10_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandler::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
        )
);
wali_test!(
    test_lossless_vec_1_11_100h_100w_1e_1c,
    tranform_webp_to_webp,
    case::<ImageHandler>::new()
        .set_input("lossless_vec_1_11.webp")
        .set_expected("lossless_vec_1_11_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandler::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
        )
);
wali_test!(
    test_lossless_vec_1_12_100h_100w_1e_1c,
    tranform_webp_to_webp,
    case::<ImageHandler>::new()
        .set_input("lossless_vec_1_12.webp")
        .set_expected("lossless_vec_1_12_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandler::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
        )
);
wali_test!(
    test_lossless_vec_1_13_100h_100w_1e_1c,
    tranform_webp_to_webp,
    case::<ImageHandler>::new()
        .set_input("lossless_vec_1_13.webp")
        .set_expected("lossless_vec_1_13_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandler::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
        )
);
wali_test!(
    test_lossless_vec_1_14_100h_100w_1e_1c,
    tranform_webp_to_webp,
    case::<ImageHandler>::new()
        .set_input("lossless_vec_1_14.webp")
        .set_expected("lossless_vec_1_14_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandler::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
        )
);
wali_test!(
    test_lossless_vec_1_15_100h_100w_1e_1c,
    tranform_webp_to_webp,
    case::<ImageHandler>::new()
        .set_input("lossless_vec_1_15.webp")
        .set_expected("lossless_vec_1_15_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandler::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
        )
);
wali_test!(
    test_lossless_vec_1_2_100h_100w_1e_1c,
    tranform_webp_to_webp,
    case::<ImageHandler>::new()
        .set_input("lossless_vec_1_2.webp")
        .set_expected("lossless_vec_1_2_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandler::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
        )
);
wali_test!(
    test_lossless_vec_1_3_100h_100w_1e_1c,
    tranform_webp_to_webp,
    case::<ImageHandler>::new()
        .set_input("lossless_vec_1_3.webp")
        .set_expected("lossless_vec_1_3_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandler::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
        )
);
wali_test!(
    test_lossless_vec_1_4_100h_100w_1e_1c,
    tranform_webp_to_webp,
    case::<ImageHandler>::new()
        .set_input("lossless_vec_1_4.webp")
        .set_expected("lossless_vec_1_4_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandler::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
        )
);
wali_test!(
    test_lossless_vec_1_5_100h_100w_1e_1c,
    tranform_webp_to_webp,
    case::<ImageHandler>::new()
        .set_input("lossless_vec_1_5.webp")
        .set_expected("lossless_vec_1_5_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandler::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
        )
);
wali_test!(
    test_lossless_vec_1_6_100h_100w_1e_1c,
    tranform_webp_to_webp,
    case::<ImageHandler>::new()
        .set_input("lossless_vec_1_6.webp")
        .set_expected("lossless_vec_1_6_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandler::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
        )
);
wali_test!(
    test_lossless_vec_1_7_100h_100w_1e_1c,
    tranform_webp_to_webp,
    case::<ImageHandler>::new()
        .set_input("lossless_vec_1_7.webp")
        .set_expected("lossless_vec_1_7_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandler::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
        )
);
wali_test!(
    test_lossless_vec_1_8_100h_100w_1e_1c,
    tranform_webp_to_webp,
    case::<ImageHandler>::new()
        .set_input("lossless_vec_1_8.webp")
        .set_expected("lossless_vec_1_8_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandler::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
        )
);
wali_test!(
    test_lossless_vec_1_9_100h_100w_1e_1c,
    tranform_webp_to_webp,
    case::<ImageHandler>::new()
        .set_input("lossless_vec_1_9.webp")
        .set_expected("lossless_vec_1_9_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandler::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
        )
);
wali_test!(
    test_lossless_vec_2_0_100h_100w_1e_1c,
    tranform_webp_to_webp,
    case::<ImageHandler>::new()
        .set_input("lossless_vec_2_0.webp")
        .set_expected("lossless_vec_2_0_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandler::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
        )
);
wali_test!(
    test_lossless_vec_2_1_100h_100w_1e_1c,
    tranform_webp_to_webp,
    case::<ImageHandler>::new()
        .set_input("lossless_vec_2_1.webp")
        .set_expected("lossless_vec_2_1_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandler::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
        )
);
wali_test!(
    test_lossless_vec_2_10_100h_100w_1e_1c,
    tranform_webp_to_webp,
    case::<ImageHandler>::new()
        .set_input("lossless_vec_2_10.webp")
        .set_expected("lossless_vec_2_10_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandler::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
        )
);
wali_test!(
    test_lossless_vec_2_11_100h_100w_1e_1c,
    tranform_webp_to_webp,
    case::<ImageHandler>::new()
        .set_input("lossless_vec_2_11.webp")
        .set_expected("lossless_vec_2_11_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandler::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
        )
);
wali_test!(
    test_lossless_vec_2_12_100h_100w_1e_1c,
    tranform_webp_to_webp,
    case::<ImageHandler>::new()
        .set_input("lossless_vec_2_12.webp")
        .set_expected("lossless_vec_2_12_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandler::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
        )
);
wali_test!(
    test_lossless_vec_2_13_100h_100w_1e_1c,
    tranform_webp_to_webp,
    case::<ImageHandler>::new()
        .set_input("lossless_vec_2_13.webp")
        .set_expected("lossless_vec_2_13_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandler::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
        )
);
wali_test!(
    test_lossless_vec_2_14_100h_100w_1e_1c,
    tranform_webp_to_webp,
    case::<ImageHandler>::new()
        .set_input("lossless_vec_2_14.webp")
        .set_expected("lossless_vec_2_14_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandler::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
        )
);
wali_test!(
    test_lossless_vec_2_15_100h_100w_1e_1c,
    tranform_webp_to_webp,
    case::<ImageHandler>::new()
        .set_input("lossless_vec_2_15.webp")
        .set_expected("lossless_vec_2_15_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandler::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
        )
);
wali_test!(
    test_lossless_vec_2_2_100h_100w_1e_1c,
    tranform_webp_to_webp,
    case::<ImageHandler>::new()
        .set_input("lossless_vec_2_2.webp")
        .set_expected("lossless_vec_2_2_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandler::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
        )
);
wali_test!(
    test_lossless_vec_2_3_100h_100w_1e_1c,
    tranform_webp_to_webp,
    case::<ImageHandler>::new()
        .set_input("lossless_vec_2_3.webp")
        .set_expected("lossless_vec_2_3_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandler::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
        )
);
wali_test!(
    test_lossless_vec_2_4_100h_100w_1e_1c,
    tranform_webp_to_webp,
    case::<ImageHandler>::new()
        .set_input("lossless_vec_2_4.webp")
        .set_expected("lossless_vec_2_4_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandler::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
        )
);
wali_test!(
    test_lossless_vec_2_5_100h_100w_1e_1c,
    tranform_webp_to_webp,
    case::<ImageHandler>::new()
        .set_input("lossless_vec_2_5.webp")
        .set_expected("lossless_vec_2_5_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandler::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
        )
);
wali_test!(
    test_lossless_vec_2_6_100h_100w_1e_1c,
    tranform_webp_to_webp,
    case::<ImageHandler>::new()
        .set_input("lossless_vec_2_6.webp")
        .set_expected("lossless_vec_2_6_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandler::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
        )
);
wali_test!(
    test_lossless_vec_2_7_100h_100w_1e_1c,
    tranform_webp_to_webp,
    case::<ImageHandler>::new()
        .set_input("lossless_vec_2_7.webp")
        .set_expected("lossless_vec_2_7_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandler::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
        )
);
wali_test!(
    test_lossless_vec_2_8_100h_100w_1e_1c,
    tranform_webp_to_webp,
    case::<ImageHandler>::new()
        .set_input("lossless_vec_2_8.webp")
        .set_expected("lossless_vec_2_8_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandler::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
        )
);
wali_test!(
    test_lossless_vec_2_9_100h_100w_1e_1c,
    tranform_webp_to_webp,
    case::<ImageHandler>::new()
        .set_input("lossless_vec_2_9.webp")
        .set_expected("lossless_vec_2_9_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandler::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
        )
);
wali_test!(
    test_lossy_alpha1_100h_100w_1e_1c,
    tranform_webp_to_webp,
    case::<ImageHandler>::new()
        .set_input("lossy_alpha1.webp")
        .set_expected("lossy_alpha1_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandler::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
        )
);
wali_test!(
    test_lossy_alpha2_100h_100w_1e_1c,
    tranform_webp_to_webp,
    case::<ImageHandler>::new()
        .set_input("lossy_alpha2.webp")
        .set_expected("lossy_alpha2_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandler::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
        )
);
wali_test!(
    test_lossy_alpha3_100h_100w_1e_1c,
    tranform_webp_to_webp,
    case::<ImageHandler>::new()
        .set_input("lossy_alpha3.webp")
        .set_expected("lossy_alpha3_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandler::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
        )
);
wali_test!(
    test_lossy_alpha4_100h_100w_1e_1c,
    tranform_webp_to_webp,
    case::<ImageHandler>::new()
        .set_input("lossy_alpha4.webp")
        .set_expected("lossy_alpha4_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandler::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
        )
);
wali_test!(
    test_lossy_extreme_probabilities_100h_100w_1e_1c,
    tranform_webp_to_webp,
    case::<ImageHandler>::new()
        .set_input("lossy_extreme_probabilities.webp")
        .set_expected("lossy_extreme_probabilities_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandler::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
        )
);
wali_test!(
    test_lossy_q0_f100_100h_100w_1e_1c,
    tranform_webp_to_webp,
    case::<ImageHandler>::new()
        .set_input("lossy_q0_f100.webp")
        .set_expected("lossy_q0_f100_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandler::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
        )
);
wali_test!(
    test_near_lossless_75_100h_100w_1e_1c,
    tranform_webp_to_webp,
    case::<ImageHandler>::new()
        .set_input("near_lossless_75.webp")
        .set_expected("near_lossless_75_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandler::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
        )
);
wali_test!(
    test_segment01_100h_100w_1e_1c,
    tranform_webp_to_webp,
    case::<ImageHandler>::new()
        .set_input("segment01.webp")
        .set_expected("segment01_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandler::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
        )
);
wali_test!(
    test_segment02_100h_100w_1e_1c,
    tranform_webp_to_webp,
    case::<ImageHandler>::new()
        .set_input("segment02.webp")
        .set_expected("segment02_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandler::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
        )
);
wali_test!(
    test_segment03_100h_100w_1e_1c,
    tranform_webp_to_webp,
    case::<ImageHandler>::new()
        .set_input("segment03.webp")
        .set_expected("segment03_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandler::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
        )
);
wali_test!(
    test_small_13x1_100h_100w_1e_1c,
    tranform_webp_to_webp,
    case::<ImageHandler>::new()
        .set_input("small_13x1.webp")
        .set_expected("small_13x1_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandler::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
        )
);
wali_test!(
    test_small_1x1_100h_100w_1e_1c,
    tranform_webp_to_webp,
    case::<ImageHandler>::new()
        .set_input("small_1x1.webp")
        .set_expected("small_1x1_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandler::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
        )
);
wali_test!(
    test_small_1x13_100h_100w_1e_1c,
    tranform_webp_to_webp,
    case::<ImageHandler>::new()
        .set_input("small_1x13.webp")
        .set_expected("small_1x13_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandler::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
        )
);
wali_test!(
    test_small_31x13_100h_100w_1e_1c,
    tranform_webp_to_webp,
    case::<ImageHandler>::new()
        .set_input("small_31x13.webp")
        .set_expected("small_31x13_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandler::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
        )
);
wali_test!(
    test_test_nostrong_100h_100w_1e_1c,
    tranform_webp_to_webp,
    case::<ImageHandler>::new()
        .set_input("test-nostrong.webp")
        .set_expected("test-nostrong_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandler::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
        )
);
wali_test!(
    test_test_100h_100w_1e_1c,
    tranform_webp_to_webp,
    case::<ImageHandler>::new()
        .set_input("test.webp")
        .set_expected("test_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandler::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
        )
);
wali_test!(
    test_very_short_100h_100w_1e_1c,
    tranform_webp_to_webp,
    case::<ImageHandler>::new()
        .set_input("very_short.webp")
        .set_expected("very_short_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandler::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
        )
);
wali_test!(
    test_vp80_00_comprehensive_001_100h_100w_1e_1c,
    tranform_webp_to_webp,
    case::<ImageHandler>::new()
        .set_input("vp80-00-comprehensive-001.webp")
        .set_expected("vp80-00-comprehensive-001_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandler::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
        )
);
wali_test!(
    test_vp80_00_comprehensive_002_100h_100w_1e_1c,
    tranform_webp_to_webp,
    case::<ImageHandler>::new()
        .set_input("vp80-00-comprehensive-002.webp")
        .set_expected("vp80-00-comprehensive-002_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandler::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
        )
);
wali_test!(
    test_vp80_00_comprehensive_003_100h_100w_1e_1c,
    tranform_webp_to_webp,
    case::<ImageHandler>::new()
        .set_input("vp80-00-comprehensive-003.webp")
        .set_expected("vp80-00-comprehensive-003_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandler::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
        )
);
wali_test!(
    test_vp80_00_comprehensive_004_100h_100w_1e_1c,
    tranform_webp_to_webp,
    case::<ImageHandler>::new()
        .set_input("vp80-00-comprehensive-004.webp")
        .set_expected("vp80-00-comprehensive-004_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandler::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
        )
);
wali_test!(
    test_vp80_00_comprehensive_005_100h_100w_1e_1c,
    tranform_webp_to_webp,
    case::<ImageHandler>::new()
        .set_input("vp80-00-comprehensive-005.webp")
        .set_expected("vp80-00-comprehensive-005_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandler::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
        )
);
wali_test!(
    test_vp80_00_comprehensive_006_100h_100w_1e_1c,
    tranform_webp_to_webp,
    case::<ImageHandler>::new()
        .set_input("vp80-00-comprehensive-006.webp")
        .set_expected("vp80-00-comprehensive-006_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandler::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
        )
);
wali_test!(
    test_vp80_00_comprehensive_007_100h_100w_1e_1c,
    tranform_webp_to_webp,
    case::<ImageHandler>::new()
        .set_input("vp80-00-comprehensive-007.webp")
        .set_expected("vp80-00-comprehensive-007_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandler::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
        )
);
wali_test!(
    test_vp80_00_comprehensive_008_100h_100w_1e_1c,
    tranform_webp_to_webp,
    case::<ImageHandler>::new()
        .set_input("vp80-00-comprehensive-008.webp")
        .set_expected("vp80-00-comprehensive-008_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandler::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
        )
);
wali_test!(
    test_vp80_00_comprehensive_009_100h_100w_1e_1c,
    tranform_webp_to_webp,
    case::<ImageHandler>::new()
        .set_input("vp80-00-comprehensive-009.webp")
        .set_expected("vp80-00-comprehensive-009_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandler::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
        )
);
wali_test!(
    test_vp80_00_comprehensive_010_100h_100w_1e_1c,
    tranform_webp_to_webp,
    case::<ImageHandler>::new()
        .set_input("vp80-00-comprehensive-010.webp")
        .set_expected("vp80-00-comprehensive-010_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandler::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
        )
);
wali_test!(
    test_vp80_00_comprehensive_011_100h_100w_1e_1c,
    tranform_webp_to_webp,
    case::<ImageHandler>::new()
        .set_input("vp80-00-comprehensive-011.webp")
        .set_expected("vp80-00-comprehensive-011_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandler::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
        )
);
wali_test!(
    test_vp80_00_comprehensive_012_100h_100w_1e_1c,
    tranform_webp_to_webp,
    case::<ImageHandler>::new()
        .set_input("vp80-00-comprehensive-012.webp")
        .set_expected("vp80-00-comprehensive-012_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandler::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
        )
);
wali_test!(
    test_vp80_00_comprehensive_013_100h_100w_1e_1c,
    tranform_webp_to_webp,
    case::<ImageHandler>::new()
        .set_input("vp80-00-comprehensive-013.webp")
        .set_expected("vp80-00-comprehensive-013_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandler::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
        )
);
wali_test!(
    test_vp80_00_comprehensive_014_100h_100w_1e_1c,
    tranform_webp_to_webp,
    case::<ImageHandler>::new()
        .set_input("vp80-00-comprehensive-014.webp")
        .set_expected("vp80-00-comprehensive-014_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandler::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
        )
);
wali_test!(
    test_vp80_00_comprehensive_015_100h_100w_1e_1c,
    tranform_webp_to_webp,
    case::<ImageHandler>::new()
        .set_input("vp80-00-comprehensive-015.webp")
        .set_expected("vp80-00-comprehensive-015_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandler::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
        )
);
wali_test!(
    test_vp80_00_comprehensive_016_100h_100w_1e_1c,
    tranform_webp_to_webp,
    case::<ImageHandler>::new()
        .set_input("vp80-00-comprehensive-016.webp")
        .set_expected("vp80-00-comprehensive-016_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandler::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
        )
);
wali_test!(
    test_vp80_00_comprehensive_017_100h_100w_1e_1c,
    tranform_webp_to_webp,
    case::<ImageHandler>::new()
        .set_input("vp80-00-comprehensive-017.webp")
        .set_expected("vp80-00-comprehensive-017_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandler::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
        )
);
wali_test!(
    test_vp80_01_intra_1400_100h_100w_1e_1c,
    tranform_webp_to_webp,
    case::<ImageHandler>::new()
        .set_input("vp80-01-intra-1400.webp")
        .set_expected("vp80-01-intra-1400_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandler::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
        )
);
wali_test!(
    test_vp80_01_intra_1411_100h_100w_1e_1c,
    tranform_webp_to_webp,
    case::<ImageHandler>::new()
        .set_input("vp80-01-intra-1411.webp")
        .set_expected("vp80-01-intra-1411_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandler::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
        )
);
wali_test!(
    test_vp80_01_intra_1416_100h_100w_1e_1c,
    tranform_webp_to_webp,
    case::<ImageHandler>::new()
        .set_input("vp80-01-intra-1416.webp")
        .set_expected("vp80-01-intra-1416_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandler::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
        )
);
wali_test!(
    test_vp80_01_intra_1417_100h_100w_1e_1c,
    tranform_webp_to_webp,
    case::<ImageHandler>::new()
        .set_input("vp80-01-intra-1417.webp")
        .set_expected("vp80-01-intra-1417_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandler::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
        )
);
wali_test!(
    test_vp80_02_inter_1402_100h_100w_1e_1c,
    tranform_webp_to_webp,
    case::<ImageHandler>::new()
        .set_input("vp80-02-inter-1402.webp")
        .set_expected("vp80-02-inter-1402_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandler::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
        )
);
wali_test!(
    test_vp80_02_inter_1412_100h_100w_1e_1c,
    tranform_webp_to_webp,
    case::<ImageHandler>::new()
        .set_input("vp80-02-inter-1412.webp")
        .set_expected("vp80-02-inter-1412_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandler::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
        )
);
wali_test!(
    test_vp80_02_inter_1418_100h_100w_1e_1c,
    tranform_webp_to_webp,
    case::<ImageHandler>::new()
        .set_input("vp80-02-inter-1418.webp")
        .set_expected("vp80-02-inter-1418_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandler::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
        )
);
wali_test!(
    test_vp80_02_inter_1424_100h_100w_1e_1c,
    tranform_webp_to_webp,
    case::<ImageHandler>::new()
        .set_input("vp80-02-inter-1424.webp")
        .set_expected("vp80-02-inter-1424_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandler::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
        )
);
wali_test!(
    test_vp80_03_segmentation_1401_100h_100w_1e_1c,
    tranform_webp_to_webp,
    case::<ImageHandler>::new()
        .set_input("vp80-03-segmentation-1401.webp")
        .set_expected("vp80-03-segmentation-1401_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandler::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
        )
);
wali_test!(
    test_vp80_03_segmentation_1403_100h_100w_1e_1c,
    tranform_webp_to_webp,
    case::<ImageHandler>::new()
        .set_input("vp80-03-segmentation-1403.webp")
        .set_expected("vp80-03-segmentation-1403_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandler::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
        )
);
wali_test!(
    test_vp80_03_segmentation_1407_100h_100w_1e_1c,
    tranform_webp_to_webp,
    case::<ImageHandler>::new()
        .set_input("vp80-03-segmentation-1407.webp")
        .set_expected("vp80-03-segmentation-1407_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandler::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
        )
);
wali_test!(
    test_vp80_03_segmentation_1408_100h_100w_1e_1c,
    tranform_webp_to_webp,
    case::<ImageHandler>::new()
        .set_input("vp80-03-segmentation-1408.webp")
        .set_expected("vp80-03-segmentation-1408_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandler::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
        )
);
wali_test!(
    test_vp80_03_segmentation_1409_100h_100w_1e_1c,
    tranform_webp_to_webp,
    case::<ImageHandler>::new()
        .set_input("vp80-03-segmentation-1409.webp")
        .set_expected("vp80-03-segmentation-1409_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandler::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
        )
);
wali_test!(
    test_vp80_03_segmentation_1410_100h_100w_1e_1c,
    tranform_webp_to_webp,
    case::<ImageHandler>::new()
        .set_input("vp80-03-segmentation-1410.webp")
        .set_expected("vp80-03-segmentation-1410_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandler::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
        )
);
wali_test!(
    test_vp80_03_segmentation_1413_100h_100w_1e_1c,
    tranform_webp_to_webp,
    case::<ImageHandler>::new()
        .set_input("vp80-03-segmentation-1413.webp")
        .set_expected("vp80-03-segmentation-1413_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandler::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
        )
);
wali_test!(
    test_vp80_03_segmentation_1414_100h_100w_1e_1c,
    tranform_webp_to_webp,
    case::<ImageHandler>::new()
        .set_input("vp80-03-segmentation-1414.webp")
        .set_expected("vp80-03-segmentation-1414_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandler::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
        )
);
wali_test!(
    test_vp80_03_segmentation_1415_100h_100w_1e_1c,
    tranform_webp_to_webp,
    case::<ImageHandler>::new()
        .set_input("vp80-03-segmentation-1415.webp")
        .set_expected("vp80-03-segmentation-1415_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandler::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
        )
);
wali_test!(
    test_vp80_03_segmentation_1425_100h_100w_1e_1c,
    tranform_webp_to_webp,
    case::<ImageHandler>::new()
        .set_input("vp80-03-segmentation-1425.webp")
        .set_expected("vp80-03-segmentation-1425_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandler::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
        )
);
wali_test!(
    test_vp80_03_segmentation_1426_100h_100w_1e_1c,
    tranform_webp_to_webp,
    case::<ImageHandler>::new()
        .set_input("vp80-03-segmentation-1426.webp")
        .set_expected("vp80-03-segmentation-1426_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandler::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
        )
);
wali_test!(
    test_vp80_03_segmentation_1427_100h_100w_1e_1c,
    tranform_webp_to_webp,
    case::<ImageHandler>::new()
        .set_input("vp80-03-segmentation-1427.webp")
        .set_expected("vp80-03-segmentation-1427_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandler::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
        )
);
wali_test!(
    test_vp80_03_segmentation_1432_100h_100w_1e_1c,
    tranform_webp_to_webp,
    case::<ImageHandler>::new()
        .set_input("vp80-03-segmentation-1432.webp")
        .set_expected("vp80-03-segmentation-1432_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandler::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
        )
);
wali_test!(
    test_vp80_03_segmentation_1435_100h_100w_1e_1c,
    tranform_webp_to_webp,
    case::<ImageHandler>::new()
        .set_input("vp80-03-segmentation-1435.webp")
        .set_expected("vp80-03-segmentation-1435_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandler::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
        )
);
wali_test!(
    test_vp80_03_segmentation_1436_100h_100w_1e_1c,
    tranform_webp_to_webp,
    case::<ImageHandler>::new()
        .set_input("vp80-03-segmentation-1436.webp")
        .set_expected("vp80-03-segmentation-1436_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandler::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
        )
);
wali_test!(
    test_vp80_03_segmentation_1437_100h_100w_1e_1c,
    tranform_webp_to_webp,
    case::<ImageHandler>::new()
        .set_input("vp80-03-segmentation-1437.webp")
        .set_expected("vp80-03-segmentation-1437_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandler::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
        )
);
wali_test!(
    test_vp80_03_segmentation_1441_100h_100w_1e_1c,
    tranform_webp_to_webp,
    case::<ImageHandler>::new()
        .set_input("vp80-03-segmentation-1441.webp")
        .set_expected("vp80-03-segmentation-1441_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandler::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
        )
);
wali_test!(
    test_vp80_03_segmentation_1442_100h_100w_1e_1c,
    tranform_webp_to_webp,
    case::<ImageHandler>::new()
        .set_input("vp80-03-segmentation-1442.webp")
        .set_expected("vp80-03-segmentation-1442_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandler::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
        )
);
wali_test!(
    test_vp80_04_partitions_1404_100h_100w_1e_1c,
    tranform_webp_to_webp,
    case::<ImageHandler>::new()
        .set_input("vp80-04-partitions-1404.webp")
        .set_expected("vp80-04-partitions-1404_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandler::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
        )
);
wali_test!(
    test_vp80_04_partitions_1405_100h_100w_1e_1c,
    tranform_webp_to_webp,
    case::<ImageHandler>::new()
        .set_input("vp80-04-partitions-1405.webp")
        .set_expected("vp80-04-partitions-1405_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandler::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
        )
);
wali_test!(
    test_vp80_04_partitions_1406_100h_100w_1e_1c,
    tranform_webp_to_webp,
    case::<ImageHandler>::new()
        .set_input("vp80-04-partitions-1406.webp")
        .set_expected("vp80-04-partitions-1406_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandler::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
        )
);
wali_test!(
    test_vp80_05_sharpness_1428_100h_100w_1e_1c,
    tranform_webp_to_webp,
    case::<ImageHandler>::new()
        .set_input("vp80-05-sharpness-1428.webp")
        .set_expected("vp80-05-sharpness-1428_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandler::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
        )
);
wali_test!(
    test_vp80_05_sharpness_1429_100h_100w_1e_1c,
    tranform_webp_to_webp,
    case::<ImageHandler>::new()
        .set_input("vp80-05-sharpness-1429.webp")
        .set_expected("vp80-05-sharpness-1429_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandler::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
        )
);
wali_test!(
    test_vp80_05_sharpness_1430_100h_100w_1e_1c,
    tranform_webp_to_webp,
    case::<ImageHandler>::new()
        .set_input("vp80-05-sharpness-1430.webp")
        .set_expected("vp80-05-sharpness-1430_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandler::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
        )
);
wali_test!(
    test_vp80_05_sharpness_1431_100h_100w_1e_1c,
    tranform_webp_to_webp,
    case::<ImageHandler>::new()
        .set_input("vp80-05-sharpness-1431.webp")
        .set_expected("vp80-05-sharpness-1431_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandler::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
        )
);
wali_test!(
    test_vp80_05_sharpness_1433_100h_100w_1e_1c,
    tranform_webp_to_webp,
    case::<ImageHandler>::new()
        .set_input("vp80-05-sharpness-1433.webp")
        .set_expected("vp80-05-sharpness-1433_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandler::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
        )
);
wali_test!(
    test_vp80_05_sharpness_1434_100h_100w_1e_1c,
    tranform_webp_to_webp,
    case::<ImageHandler>::new()
        .set_input("vp80-05-sharpness-1434.webp")
        .set_expected("vp80-05-sharpness-1434_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandler::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
        )
);
wali_test!(
    test_vp80_05_sharpness_1438_100h_100w_1e_1c,
    tranform_webp_to_webp,
    case::<ImageHandler>::new()
        .set_input("vp80-05-sharpness-1438.webp")
        .set_expected("vp80-05-sharpness-1438_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandler::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
        )
);
wali_test!(
    test_vp80_05_sharpness_1439_100h_100w_1e_1c,
    tranform_webp_to_webp,
    case::<ImageHandler>::new()
        .set_input("vp80-05-sharpness-1439.webp")
        .set_expected("vp80-05-sharpness-1439_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandler::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
        )
);
wali_test!(
    test_vp80_05_sharpness_1440_100h_100w_1e_1c,
    tranform_webp_to_webp,
    case::<ImageHandler>::new()
        .set_input("vp80-05-sharpness-1440.webp")
        .set_expected("vp80-05-sharpness-1440_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandler::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
        )
);
wali_test!(
    test_vp80_05_sharpness_1443_100h_100w_1e_1c,
    tranform_webp_to_webp,
    case::<ImageHandler>::new()
        .set_input("vp80-05-sharpness-1443.webp")
        .set_expected("vp80-05-sharpness-1443_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandler::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
        )
);
