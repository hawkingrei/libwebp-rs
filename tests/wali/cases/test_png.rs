use imagers::ImageHandler;
use imagers::ImageHandlerBuilder;

use crate::case;
use crate::test_config;
use crc::{crc32, Hasher32};
use imagers::png_encode_webp;
use imagers::Resize;
use std::fs;
use std::path::Path;

fn tranform_png_to_webp(config: &test_config, case: case<ImageHandler>) -> Result<(), String> {
    let mut input: String = case.input;
    let mut expected: String = case.expected;
    let mut is_corrupted: bool = case.is_corrupted;
    let mut im: ImageHandler = case.param;
    let mut fact_input = config.input.clone();
    fact_input.push_str("png/");
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

    let result = match png_encode_webp(&data.clone(), im) {
        Ok(result) => result,
        Err(err) => {
            if is_corrupted {
                return Ok(());
            } else {
                return Err(format!("{} fail at png_encode_webp. {}", input, err));
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
    digest_output.write(result.pic.as_slice());

    fs::write(Path::new(Path::new(&fact_output)), result.pic);

    if digest_expected.sum32() != digest_output.sum32() {
        return Err(format!("{} fail to test", input));
    }
    return Ok(());
}

// archive/080fec32d3e0eeb407aafa1fbad7637c21e51601.png@472w_265h_1e_1c.webp
wali_test!(
    test_1e51601_472w_265h_1e_1c,
    tranform_png_to_webp,
    case::<ImageHandler>::new()
        .set_input("080fec32d3e0eeb407aafa1fbad7637c21e51601.png")
        .set_expected("080fec32d3e0eeb407aafa1fbad7637c21e51601_472w_265h_1e_1c.webp")
        .set_param(
            ImageHandlerBuilder::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 472,
                    height: 265,
                }))
                .finish()
        )
);
wali_test!(
    test_xs1n0g01_png,
    tranform_png_to_webp,
    case::<ImageHandler>::new()
        .set_input("xs1n0g01.png")
        .corrupted()
);
wali_test!(
    test_xs2n0g01_png,
    tranform_png_to_webp,
    case::<ImageHandler>::new()
        .set_input("xs2n0g01.png")
        .corrupted()
);
wali_test!(
    test_xs4n0g01_png,
    tranform_png_to_webp,
    case::<ImageHandler>::new()
        .set_input("xs4n0g01.png")
        .corrupted()
);
wali_test!(
    test_xs7n0g01_png,
    tranform_png_to_webp,
    case::<ImageHandler>::new()
        .set_input("xs7n0g01.png")
        .corrupted()
);
wali_test!(
    test_xcrn0g04_png,
    tranform_png_to_webp,
    case::<ImageHandler>::new()
        .set_input("xcrn0g04.png")
        .corrupted()
);
wali_test!(
    test_xlfn0g04_png,
    tranform_png_to_webp,
    case::<ImageHandler>::new()
        .set_input("xlfn0g04.png")
        .corrupted()
);
wali_test!(
    test_xhdn0g08_png,
    tranform_png_to_webp,
    case::<ImageHandler>::new()
        .set_input("xhdn0g08.png")
        .corrupted()
);
wali_test!(
    test_xc1n0g08_png,
    tranform_png_to_webp,
    case::<ImageHandler>::new()
        .set_input("xc1n0g08.png")
        .corrupted()
);
wali_test!(
    test_xc9n2c08_png,
    tranform_png_to_webp,
    case::<ImageHandler>::new()
        .set_input("xc9n2c08.png")
        .corrupted()
);
wali_test!(
    test_xd0n2c08_png,
    tranform_png_to_webp,
    case::<ImageHandler>::new()
        .set_input("xd0n2c08.png")
        .corrupted()
);
wali_test!(
    test_xd3n2c08_png,
    tranform_png_to_webp,
    case::<ImageHandler>::new()
        .set_input("xd3n2c08.png")
        .corrupted()
);
wali_test!(
    test_xd9n2c08_png,
    tranform_png_to_webp,
    case::<ImageHandler>::new()
        .set_input("xd9n2c08.png")
        .corrupted()
);
wali_test!(
    test_basn0g01_100h_100w_1e_1c,
    tranform_png_to_webp,
    case::<ImageHandler>::new()
        .set_input("basn0g01.png")
        .set_expected("basn0g01_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandlerBuilder::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
                .finish()
        )
);
wali_test!(
    test_basi4a08_100h_100w_1e_1c,
    tranform_png_to_webp,
    case::<ImageHandler>::new()
        .set_input("basi4a08.png")
        .set_expected("basi4a08_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandlerBuilder::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
                .finish()
        )
);
wali_test!(
    test_basn0g02_100h_100w_1e_1c,
    tranform_png_to_webp,
    case::<ImageHandler>::new()
        .set_input("basn0g02.png")
        .set_expected("basn0g02_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandlerBuilder::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
                .finish()
        )
);
wali_test!(
    test_basn0g04_100h_100w_1e_1c,
    tranform_png_to_webp,
    case::<ImageHandler>::new()
        .set_input("basn0g04.png")
        .set_expected("basn0g04_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandlerBuilder::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
                .finish()
        )
);
wali_test!(
    test_basn0g08_100h_100w_1e_1c,
    tranform_png_to_webp,
    case::<ImageHandler>::new()
        .set_input("basn0g08.png")
        .set_expected("basn0g08_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandlerBuilder::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
                .finish()
        )
);
wali_test!(
    test_basn0g16_100h_100w_1e_1c,
    tranform_png_to_webp,
    case::<ImageHandler>::new()
        .set_input("basn0g16.png")
        .set_expected("basn0g16_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandlerBuilder::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
                .finish()
        )
);
wali_test!(
    test_basn2c08_100h_100w_1e_1c,
    tranform_png_to_webp,
    case::<ImageHandler>::new()
        .set_input("basn2c08.png")
        .set_expected("basn2c08_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandlerBuilder::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
                .finish()
        )
);
wali_test!(
    test_basn2c16_100h_100w_1e_1c,
    tranform_png_to_webp,
    case::<ImageHandler>::new()
        .set_input("basn2c16.png")
        .set_expected("basn2c16_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandlerBuilder::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
                .finish()
        )
);
wali_test!(
    test_basn3p01_100h_100w_1e_1c,
    tranform_png_to_webp,
    case::<ImageHandler>::new()
        .set_input("basn3p01.png")
        .set_expected("basn3p01_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandlerBuilder::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
                .finish()
        )
);
wali_test!(
    test_basn3p02_100h_100w_1e_1c,
    tranform_png_to_webp,
    case::<ImageHandler>::new()
        .set_input("basn3p02.png")
        .set_expected("basn3p02_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandlerBuilder::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
                .finish()
        )
);
wali_test!(
    test_basn3p04_100h_100w_1e_1c,
    tranform_png_to_webp,
    case::<ImageHandler>::new()
        .set_input("basn3p04.png")
        .set_expected("basn3p04_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandlerBuilder::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
                .finish()
        )
);
wali_test!(
    test_basn3p08_100h_100w_1e_1c,
    tranform_png_to_webp,
    case::<ImageHandler>::new()
        .set_input("basn3p08.png")
        .set_expected("basn3p08_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandlerBuilder::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
                .finish()
        )
);
wali_test!(
    test_basn4a08_100h_100w_1e_1c,
    tranform_png_to_webp,
    case::<ImageHandler>::new()
        .set_input("basn4a08.png")
        .set_expected("basn4a08_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandlerBuilder::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
                .finish()
        )
);
wali_test!(
    test_basn4a16_100h_100w_1e_1c,
    tranform_png_to_webp,
    case::<ImageHandler>::new()
        .set_input("basn4a16.png")
        .set_expected("basn4a16_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandlerBuilder::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
                .finish()
        )
);
wali_test!(
    test_basn6a08_100h_100w_1e_1c,
    tranform_png_to_webp,
    case::<ImageHandler>::new()
        .set_input("basn6a08.png")
        .set_expected("basn6a08_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandlerBuilder::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
                .finish()
        )
);
wali_test!(
    test_basn6a16_100h_100w_1e_1c,
    tranform_png_to_webp,
    case::<ImageHandler>::new()
        .set_input("basn6a16.png")
        .set_expected("basn6a16_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandlerBuilder::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
                .finish()
        )
);
wali_test!(
    test_basi0g01_100h_100w_1e_1c,
    tranform_png_to_webp,
    case::<ImageHandler>::new()
        .set_input("basi0g01.png")
        .set_expected("basi0g01_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandlerBuilder::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
                .finish()
        )
);
wali_test!(
    test_basi0g02_100h_100w_1e_1c,
    tranform_png_to_webp,
    case::<ImageHandler>::new()
        .set_input("basi0g02.png")
        .set_expected("basi0g02_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandlerBuilder::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
                .finish()
        )
);
wali_test!(
    test_basi0g04_100h_100w_1e_1c,
    tranform_png_to_webp,
    case::<ImageHandler>::new()
        .set_input("basi0g04.png")
        .set_expected("basi0g04_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandlerBuilder::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
                .finish()
        )
);
wali_test!(
    test_basi0g08_100h_100w_1e_1c,
    tranform_png_to_webp,
    case::<ImageHandler>::new()
        .set_input("basi0g08.png")
        .set_expected("basi0g08_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandlerBuilder::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
                .finish()
        )
);
wali_test!(
    test_basi0g16_100h_100w_1e_1c,
    tranform_png_to_webp,
    case::<ImageHandler>::new()
        .set_input("basi0g16.png")
        .set_expected("basi0g16_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandlerBuilder::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
                .finish()
        )
);
wali_test!(
    test_basi2c08_100h_100w_1e_1c,
    tranform_png_to_webp,
    case::<ImageHandler>::new()
        .set_input("basi2c08.png")
        .set_expected("basi2c08_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandlerBuilder::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
                .finish()
        )
);
wali_test!(
    test_basi2c16_100h_100w_1e_1c,
    tranform_png_to_webp,
    case::<ImageHandler>::new()
        .set_input("basi2c16.png")
        .set_expected("basi2c16_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandlerBuilder::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
                .finish()
        )
);
wali_test!(
    test_basi3p01_100h_100w_1e_1c,
    tranform_png_to_webp,
    case::<ImageHandler>::new()
        .set_input("basi3p01.png")
        .set_expected("basi3p01_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandlerBuilder::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
                .finish()
        )
);
wali_test!(
    test_basi3p02_100h_100w_1e_1c,
    tranform_png_to_webp,
    case::<ImageHandler>::new()
        .set_input("basi3p02.png")
        .set_expected("basi3p02_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandlerBuilder::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
                .finish()
        )
);
wali_test!(
    test_basi3p04_100h_100w_1e_1c,
    tranform_png_to_webp,
    case::<ImageHandler>::new()
        .set_input("basi3p04.png")
        .set_expected("basi3p04_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandlerBuilder::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
                .finish()
        )
);
wali_test!(
    test_basi3p08_100h_100w_1e_1c,
    tranform_png_to_webp,
    case::<ImageHandler>::new()
        .set_input("basi3p08.png")
        .set_expected("basi3p08_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandlerBuilder::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
                .finish()
        )
);
wali_test!(
    test_basi4a16_100h_100w_1e_1c,
    tranform_png_to_webp,
    case::<ImageHandler>::new()
        .set_input("basi4a16.png")
        .set_expected("basi4a16_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandlerBuilder::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
                .finish()
        )
);
wali_test!(
    test_basi6a08_100h_100w_1e_1c,
    tranform_png_to_webp,
    case::<ImageHandler>::new()
        .set_input("basi6a08.png")
        .set_expected("basi6a08_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandlerBuilder::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
                .finish()
        )
);
wali_test!(
    test_basi6a16_100h_100w_1e_1c,
    tranform_png_to_webp,
    case::<ImageHandler>::new()
        .set_input("basi6a16.png")
        .set_expected("basi6a16_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandlerBuilder::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
                .finish()
        )
);
wali_test!(
    test_s01i3p01_100h_100w_1e_1c,
    tranform_png_to_webp,
    case::<ImageHandler>::new()
        .set_input("s01i3p01.png")
        .set_expected("s01i3p01_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandlerBuilder::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
                .finish()
        )
);
wali_test!(
    test_s01n3p01_100h_100w_1e_1c,
    tranform_png_to_webp,
    case::<ImageHandler>::new()
        .set_input("s01n3p01.png")
        .set_expected("s01n3p01_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandlerBuilder::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
                .finish()
        )
);
wali_test!(
    test_s02i3p01_100h_100w_1e_1c,
    tranform_png_to_webp,
    case::<ImageHandler>::new()
        .set_input("s02i3p01.png")
        .set_expected("s02i3p01_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandlerBuilder::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
                .finish()
        )
);
wali_test!(
    test_s02n3p01_100h_100w_1e_1c,
    tranform_png_to_webp,
    case::<ImageHandler>::new()
        .set_input("s02n3p01.png")
        .set_expected("s02n3p01_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandlerBuilder::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
                .finish()
        )
);
wali_test!(
    test_s03i3p01_100h_100w_1e_1c,
    tranform_png_to_webp,
    case::<ImageHandler>::new()
        .set_input("s03i3p01.png")
        .set_expected("s03i3p01_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandlerBuilder::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
                .finish()
        )
);
wali_test!(
    test_s03n3p01_100h_100w_1e_1c,
    tranform_png_to_webp,
    case::<ImageHandler>::new()
        .set_input("s03n3p01.png")
        .set_expected("s03n3p01_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandlerBuilder::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
                .finish()
        )
);
wali_test!(
    test_s04i3p01_100h_100w_1e_1c,
    tranform_png_to_webp,
    case::<ImageHandler>::new()
        .set_input("s04i3p01.png")
        .set_expected("s04i3p01_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandlerBuilder::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
                .finish()
        )
);
wali_test!(
    test_s04n3p01_100h_100w_1e_1c,
    tranform_png_to_webp,
    case::<ImageHandler>::new()
        .set_input("s04n3p01.png")
        .set_expected("s04n3p01_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandlerBuilder::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
                .finish()
        )
);
wali_test!(
    test_s05i3p02_100h_100w_1e_1c,
    tranform_png_to_webp,
    case::<ImageHandler>::new()
        .set_input("s05i3p02.png")
        .set_expected("s05i3p02_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandlerBuilder::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
                .finish()
        )
);
wali_test!(
    test_s05n3p02_100h_100w_1e_1c,
    tranform_png_to_webp,
    case::<ImageHandler>::new()
        .set_input("s05n3p02.png")
        .set_expected("s05n3p02_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandlerBuilder::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
                .finish()
        )
);
wali_test!(
    test_s06i3p02_100h_100w_1e_1c,
    tranform_png_to_webp,
    case::<ImageHandler>::new()
        .set_input("s06i3p02.png")
        .set_expected("s06i3p02_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandlerBuilder::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
                .finish()
        )
);
wali_test!(
    test_s06n3p02_100h_100w_1e_1c,
    tranform_png_to_webp,
    case::<ImageHandler>::new()
        .set_input("s06n3p02.png")
        .set_expected("s06n3p02_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandlerBuilder::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
                .finish()
        )
);
wali_test!(
    test_s07i3p02_100h_100w_1e_1c,
    tranform_png_to_webp,
    case::<ImageHandler>::new()
        .set_input("s07i3p02.png")
        .set_expected("s07i3p02_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandlerBuilder::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
                .finish()
        )
);
wali_test!(
    test_s07n3p02_100h_100w_1e_1c,
    tranform_png_to_webp,
    case::<ImageHandler>::new()
        .set_input("s07n3p02.png")
        .set_expected("s07n3p02_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandlerBuilder::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
                .finish()
        )
);
wali_test!(
    test_s08i3p02_100h_100w_1e_1c,
    tranform_png_to_webp,
    case::<ImageHandler>::new()
        .set_input("s08i3p02.png")
        .set_expected("s08i3p02_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandlerBuilder::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
                .finish()
        )
);
wali_test!(
    test_s08n3p02_100h_100w_1e_1c,
    tranform_png_to_webp,
    case::<ImageHandler>::new()
        .set_input("s08n3p02.png")
        .set_expected("s08n3p02_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandlerBuilder::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
                .finish()
        )
);
wali_test!(
    test_s09i3p02_100h_100w_1e_1c,
    tranform_png_to_webp,
    case::<ImageHandler>::new()
        .set_input("s09i3p02.png")
        .set_expected("s09i3p02_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandlerBuilder::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
                .finish()
        )
);
wali_test!(
    test_s09n3p02_100h_100w_1e_1c,
    tranform_png_to_webp,
    case::<ImageHandler>::new()
        .set_input("s09n3p02.png")
        .set_expected("s09n3p02_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandlerBuilder::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
                .finish()
        )
);
wali_test!(
    test_s32i3p04_100h_100w_1e_1c,
    tranform_png_to_webp,
    case::<ImageHandler>::new()
        .set_input("s32i3p04.png")
        .set_expected("s32i3p04_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandlerBuilder::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
                .finish()
        )
);
wali_test!(
    test_s32n3p04_100h_100w_1e_1c,
    tranform_png_to_webp,
    case::<ImageHandler>::new()
        .set_input("s32n3p04.png")
        .set_expected("s32n3p04_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandlerBuilder::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
                .finish()
        )
);
wali_test!(
    test_s33i3p04_100h_100w_1e_1c,
    tranform_png_to_webp,
    case::<ImageHandler>::new()
        .set_input("s33i3p04.png")
        .set_expected("s33i3p04_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandlerBuilder::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
                .finish()
        )
);
wali_test!(
    test_s33n3p04_100h_100w_1e_1c,
    tranform_png_to_webp,
    case::<ImageHandler>::new()
        .set_input("s33n3p04.png")
        .set_expected("s33n3p04_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandlerBuilder::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
                .finish()
        )
);
wali_test!(
    test_s34i3p04_100h_100w_1e_1c,
    tranform_png_to_webp,
    case::<ImageHandler>::new()
        .set_input("s34i3p04.png")
        .set_expected("s34i3p04_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandlerBuilder::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
                .finish()
        )
);
wali_test!(
    test_s34n3p04_100h_100w_1e_1c,
    tranform_png_to_webp,
    case::<ImageHandler>::new()
        .set_input("s34n3p04.png")
        .set_expected("s34n3p04_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandlerBuilder::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
                .finish()
        )
);
wali_test!(
    test_s35i3p04_100h_100w_1e_1c,
    tranform_png_to_webp,
    case::<ImageHandler>::new()
        .set_input("s35i3p04.png")
        .set_expected("s35i3p04_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandlerBuilder::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
                .finish()
        )
);
wali_test!(
    test_s35n3p04_100h_100w_1e_1c,
    tranform_png_to_webp,
    case::<ImageHandler>::new()
        .set_input("s35n3p04.png")
        .set_expected("s35n3p04_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandlerBuilder::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
                .finish()
        )
);
wali_test!(
    test_s36i3p04_100h_100w_1e_1c,
    tranform_png_to_webp,
    case::<ImageHandler>::new()
        .set_input("s36i3p04.png")
        .set_expected("s36i3p04_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandlerBuilder::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
                .finish()
        )
);
wali_test!(
    test_s36n3p04_100h_100w_1e_1c,
    tranform_png_to_webp,
    case::<ImageHandler>::new()
        .set_input("s36n3p04.png")
        .set_expected("s36n3p04_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandlerBuilder::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
                .finish()
        )
);
wali_test!(
    test_s37i3p04_100h_100w_1e_1c,
    tranform_png_to_webp,
    case::<ImageHandler>::new()
        .set_input("s37i3p04.png")
        .set_expected("s37i3p04_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandlerBuilder::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
                .finish()
        )
);
wali_test!(
    test_s37n3p04_100h_100w_1e_1c,
    tranform_png_to_webp,
    case::<ImageHandler>::new()
        .set_input("s37n3p04.png")
        .set_expected("s37n3p04_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandlerBuilder::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
                .finish()
        )
);
wali_test!(
    test_s38i3p04_100h_100w_1e_1c,
    tranform_png_to_webp,
    case::<ImageHandler>::new()
        .set_input("s38i3p04.png")
        .set_expected("s38i3p04_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandlerBuilder::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
                .finish()
        )
);
wali_test!(
    test_s38n3p04_100h_100w_1e_1c,
    tranform_png_to_webp,
    case::<ImageHandler>::new()
        .set_input("s38n3p04.png")
        .set_expected("s38n3p04_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandlerBuilder::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
                .finish()
        )
);
wali_test!(
    test_s39i3p04_100h_100w_1e_1c,
    tranform_png_to_webp,
    case::<ImageHandler>::new()
        .set_input("s39i3p04.png")
        .set_expected("s39i3p04_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandlerBuilder::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
                .finish()
        )
);
wali_test!(
    test_s39n3p04_100h_100w_1e_1c,
    tranform_png_to_webp,
    case::<ImageHandler>::new()
        .set_input("s39n3p04.png")
        .set_expected("s39n3p04_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandlerBuilder::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
                .finish()
        )
);
wali_test!(
    test_s40i3p04_100h_100w_1e_1c,
    tranform_png_to_webp,
    case::<ImageHandler>::new()
        .set_input("s40i3p04.png")
        .set_expected("s40i3p04_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandlerBuilder::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
                .finish()
        )
);
wali_test!(
    test_s40n3p04_100h_100w_1e_1c,
    tranform_png_to_webp,
    case::<ImageHandler>::new()
        .set_input("s40n3p04.png")
        .set_expected("s40n3p04_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandlerBuilder::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
                .finish()
        )
);
wali_test!(
    test_bgai4a08_100h_100w_1e_1c,
    tranform_png_to_webp,
    case::<ImageHandler>::new()
        .set_input("bgai4a08.png")
        .set_expected("bgai4a08_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandlerBuilder::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
                .finish()
        )
);
wali_test!(
    test_bgai4a16_100h_100w_1e_1c,
    tranform_png_to_webp,
    case::<ImageHandler>::new()
        .set_input("bgai4a16.png")
        .set_expected("bgai4a16_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandlerBuilder::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
                .finish()
        )
);
wali_test!(
    test_bgan6a08_100h_100w_1e_1c,
    tranform_png_to_webp,
    case::<ImageHandler>::new()
        .set_input("bgan6a08.png")
        .set_expected("bgan6a08_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandlerBuilder::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
                .finish()
        )
);
wali_test!(
    test_bgan6a16_100h_100w_1e_1c,
    tranform_png_to_webp,
    case::<ImageHandler>::new()
        .set_input("bgan6a16.png")
        .set_expected("bgan6a16_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandlerBuilder::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
                .finish()
        )
);
wali_test!(
    test_bgbn4a08_100h_100w_1e_1c,
    tranform_png_to_webp,
    case::<ImageHandler>::new()
        .set_input("bgbn4a08.png")
        .set_expected("bgbn4a08_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandlerBuilder::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
                .finish()
        )
);
wali_test!(
    test_bggn4a16_100h_100w_1e_1c,
    tranform_png_to_webp,
    case::<ImageHandler>::new()
        .set_input("bggn4a16.png")
        .set_expected("bggn4a16_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandlerBuilder::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
                .finish()
        )
);
wali_test!(
    test_bgwn6a08_100h_100w_1e_1c,
    tranform_png_to_webp,
    case::<ImageHandler>::new()
        .set_input("bgwn6a08.png")
        .set_expected("bgwn6a08_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandlerBuilder::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
                .finish()
        )
);
wali_test!(
    test_bgyn6a16_100h_100w_1e_1c,
    tranform_png_to_webp,
    case::<ImageHandler>::new()
        .set_input("bgyn6a16.png")
        .set_expected("bgyn6a16_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandlerBuilder::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
                .finish()
        )
);
wali_test!(
    test_tbbn0g04_100h_100w_1e_1c,
    tranform_png_to_webp,
    case::<ImageHandler>::new()
        .set_input("tbbn0g04.png")
        .set_expected("tbbn0g04_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandlerBuilder::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
                .finish()
        )
);
wali_test!(
    test_tbbn2c16_100h_100w_1e_1c,
    tranform_png_to_webp,
    case::<ImageHandler>::new()
        .set_input("tbbn2c16.png")
        .set_expected("tbbn2c16_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandlerBuilder::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
                .finish()
        )
);
wali_test!(
    test_tbbn3p08_100h_100w_1e_1c,
    tranform_png_to_webp,
    case::<ImageHandler>::new()
        .set_input("tbbn3p08.png")
        .set_expected("tbbn3p08_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandlerBuilder::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
                .finish()
        )
);
wali_test!(
    test_tbgn2c16_100h_100w_1e_1c,
    tranform_png_to_webp,
    case::<ImageHandler>::new()
        .set_input("tbgn2c16.png")
        .set_expected("tbgn2c16_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandlerBuilder::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
                .finish()
        )
);
wali_test!(
    test_tbgn3p08_100h_100w_1e_1c,
    tranform_png_to_webp,
    case::<ImageHandler>::new()
        .set_input("tbgn3p08.png")
        .set_expected("tbgn3p08_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandlerBuilder::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
                .finish()
        )
);
wali_test!(
    test_tbrn2c08_100h_100w_1e_1c,
    tranform_png_to_webp,
    case::<ImageHandler>::new()
        .set_input("tbrn2c08.png")
        .set_expected("tbrn2c08_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandlerBuilder::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
                .finish()
        )
);
wali_test!(
    test_tbwn0g16_100h_100w_1e_1c,
    tranform_png_to_webp,
    case::<ImageHandler>::new()
        .set_input("tbwn0g16.png")
        .set_expected("tbwn0g16_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandlerBuilder::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
                .finish()
        )
);
wali_test!(
    test_tbwn3p08_100h_100w_1e_1c,
    tranform_png_to_webp,
    case::<ImageHandler>::new()
        .set_input("tbwn3p08.png")
        .set_expected("tbwn3p08_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandlerBuilder::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
                .finish()
        )
);
wali_test!(
    test_tbyn3p08_100h_100w_1e_1c,
    tranform_png_to_webp,
    case::<ImageHandler>::new()
        .set_input("tbyn3p08.png")
        .set_expected("tbyn3p08_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandlerBuilder::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
                .finish()
        )
);
wali_test!(
    test_tm3n3p02_100h_100w_1e_1c,
    tranform_png_to_webp,
    case::<ImageHandler>::new()
        .set_input("tm3n3p02.png")
        .set_expected("tm3n3p02_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandlerBuilder::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
                .finish()
        )
);
wali_test!(
    test_tp0n0g08_100h_100w_1e_1c,
    tranform_png_to_webp,
    case::<ImageHandler>::new()
        .set_input("tp0n0g08.png")
        .set_expected("tp0n0g08_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandlerBuilder::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
                .finish()
        )
);
wali_test!(
    test_tp0n2c08_100h_100w_1e_1c,
    tranform_png_to_webp,
    case::<ImageHandler>::new()
        .set_input("tp0n2c08.png")
        .set_expected("tp0n2c08_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandlerBuilder::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
                .finish()
        )
);
wali_test!(
    test_tp0n3p08_100h_100w_1e_1c,
    tranform_png_to_webp,
    case::<ImageHandler>::new()
        .set_input("tp0n3p08.png")
        .set_expected("tp0n3p08_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandlerBuilder::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
                .finish()
        )
);
wali_test!(
    test_tp1n3p08_100h_100w_1e_1c,
    tranform_png_to_webp,
    case::<ImageHandler>::new()
        .set_input("tp1n3p08.png")
        .set_expected("tp1n3p08_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandlerBuilder::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
                .finish()
        )
);
wali_test!(
    test_g03n0g16_100h_100w_1e_1c,
    tranform_png_to_webp,
    case::<ImageHandler>::new()
        .set_input("g03n0g16.png")
        .set_expected("g03n0g16_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandlerBuilder::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
                .finish()
        )
);
wali_test!(
    test_g03n2c08_100h_100w_1e_1c,
    tranform_png_to_webp,
    case::<ImageHandler>::new()
        .set_input("g03n2c08.png")
        .set_expected("g03n2c08_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandlerBuilder::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
                .finish()
        )
);
wali_test!(
    test_g03n3p04_100h_100w_1e_1c,
    tranform_png_to_webp,
    case::<ImageHandler>::new()
        .set_input("g03n3p04.png")
        .set_expected("g03n3p04_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandlerBuilder::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
                .finish()
        )
);
wali_test!(
    test_g04n0g16_100h_100w_1e_1c,
    tranform_png_to_webp,
    case::<ImageHandler>::new()
        .set_input("g04n0g16.png")
        .set_expected("g04n0g16_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandlerBuilder::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
                .finish()
        )
);
wali_test!(
    test_g04n2c08_100h_100w_1e_1c,
    tranform_png_to_webp,
    case::<ImageHandler>::new()
        .set_input("g04n2c08.png")
        .set_expected("g04n2c08_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandlerBuilder::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
                .finish()
        )
);
wali_test!(
    test_g04n3p04_100h_100w_1e_1c,
    tranform_png_to_webp,
    case::<ImageHandler>::new()
        .set_input("g04n3p04.png")
        .set_expected("g04n3p04_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandlerBuilder::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
                .finish()
        )
);
wali_test!(
    test_g05n0g16_100h_100w_1e_1c,
    tranform_png_to_webp,
    case::<ImageHandler>::new()
        .set_input("g05n0g16.png")
        .set_expected("g05n0g16_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandlerBuilder::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
                .finish()
        )
);
wali_test!(
    test_g05n2c08_100h_100w_1e_1c,
    tranform_png_to_webp,
    case::<ImageHandler>::new()
        .set_input("g05n2c08.png")
        .set_expected("g05n2c08_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandlerBuilder::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
                .finish()
        )
);
wali_test!(
    test_g05n3p04_100h_100w_1e_1c,
    tranform_png_to_webp,
    case::<ImageHandler>::new()
        .set_input("g05n3p04.png")
        .set_expected("g05n3p04_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandlerBuilder::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
                .finish()
        )
);
wali_test!(
    test_g07n0g16_100h_100w_1e_1c,
    tranform_png_to_webp,
    case::<ImageHandler>::new()
        .set_input("g07n0g16.png")
        .set_expected("g07n0g16_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandlerBuilder::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
                .finish()
        )
);
wali_test!(
    test_g07n2c08_100h_100w_1e_1c,
    tranform_png_to_webp,
    case::<ImageHandler>::new()
        .set_input("g07n2c08.png")
        .set_expected("g07n2c08_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandlerBuilder::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
                .finish()
        )
);
wali_test!(
    test_g07n3p04_100h_100w_1e_1c,
    tranform_png_to_webp,
    case::<ImageHandler>::new()
        .set_input("g07n3p04.png")
        .set_expected("g07n3p04_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandlerBuilder::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
                .finish()
        )
);
wali_test!(
    test_g10n0g16_100h_100w_1e_1c,
    tranform_png_to_webp,
    case::<ImageHandler>::new()
        .set_input("g10n0g16.png")
        .set_expected("g10n0g16_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandlerBuilder::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
                .finish()
        )
);
wali_test!(
    test_g10n2c08_100h_100w_1e_1c,
    tranform_png_to_webp,
    case::<ImageHandler>::new()
        .set_input("g10n2c08.png")
        .set_expected("g10n2c08_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandlerBuilder::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
                .finish()
        )
);
wali_test!(
    test_g10n3p04_100h_100w_1e_1c,
    tranform_png_to_webp,
    case::<ImageHandler>::new()
        .set_input("g10n3p04.png")
        .set_expected("g10n3p04_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandlerBuilder::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
                .finish()
        )
);
wali_test!(
    test_g25n0g16_100h_100w_1e_1c,
    tranform_png_to_webp,
    case::<ImageHandler>::new()
        .set_input("g25n0g16.png")
        .set_expected("g25n0g16_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandlerBuilder::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
                .finish()
        )
);
wali_test!(
    test_g25n2c08_100h_100w_1e_1c,
    tranform_png_to_webp,
    case::<ImageHandler>::new()
        .set_input("g25n2c08.png")
        .set_expected("g25n2c08_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandlerBuilder::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
                .finish()
        )
);
wali_test!(
    test_g25n3p04_100h_100w_1e_1c,
    tranform_png_to_webp,
    case::<ImageHandler>::new()
        .set_input("g25n3p04.png")
        .set_expected("g25n3p04_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandlerBuilder::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
                .finish()
        )
);
wali_test!(
    test_z00n2c08_100h_100w_1e_1c,
    tranform_png_to_webp,
    case::<ImageHandler>::new()
        .set_input("z00n2c08.png")
        .set_expected("z00n2c08_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandlerBuilder::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
                .finish()
        )
);
wali_test!(
    test_z03n2c08_100h_100w_1e_1c,
    tranform_png_to_webp,
    case::<ImageHandler>::new()
        .set_input("z03n2c08.png")
        .set_expected("z03n2c08_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandlerBuilder::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
                .finish()
        )
);
wali_test!(
    test_z06n2c08_100h_100w_1e_1c,
    tranform_png_to_webp,
    case::<ImageHandler>::new()
        .set_input("z06n2c08.png")
        .set_expected("z06n2c08_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandlerBuilder::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
                .finish()
        )
);
wali_test!(
    test_z09n2c08_100h_100w_1e_1c,
    tranform_png_to_webp,
    case::<ImageHandler>::new()
        .set_input("z09n2c08.png")
        .set_expected("z09n2c08_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandlerBuilder::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
                .finish()
        )
);
wali_test!(
    test_oi1n0g16_100h_100w_1e_1c,
    tranform_png_to_webp,
    case::<ImageHandler>::new()
        .set_input("oi1n0g16.png")
        .set_expected("oi1n0g16_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandlerBuilder::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
                .finish()
        )
);
wali_test!(
    test_oi1n2c16_100h_100w_1e_1c,
    tranform_png_to_webp,
    case::<ImageHandler>::new()
        .set_input("oi1n2c16.png")
        .set_expected("oi1n2c16_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandlerBuilder::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
                .finish()
        )
);
wali_test!(
    test_oi2n0g16_100h_100w_1e_1c,
    tranform_png_to_webp,
    case::<ImageHandler>::new()
        .set_input("oi2n0g16.png")
        .set_expected("oi2n0g16_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandlerBuilder::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
                .finish()
        )
);
wali_test!(
    test_oi2n2c16_100h_100w_1e_1c,
    tranform_png_to_webp,
    case::<ImageHandler>::new()
        .set_input("oi2n2c16.png")
        .set_expected("oi2n2c16_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandlerBuilder::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
                .finish()
        )
);
wali_test!(
    test_oi4n0g16_100h_100w_1e_1c,
    tranform_png_to_webp,
    case::<ImageHandler>::new()
        .set_input("oi4n0g16.png")
        .set_expected("oi4n0g16_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandlerBuilder::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
                .finish()
        )
);
wali_test!(
    test_oi4n2c16_100h_100w_1e_1c,
    tranform_png_to_webp,
    case::<ImageHandler>::new()
        .set_input("oi4n2c16.png")
        .set_expected("oi4n2c16_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandlerBuilder::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
                .finish()
        )
);
wali_test!(
    test_oi9n0g16_100h_100w_1e_1c,
    tranform_png_to_webp,
    case::<ImageHandler>::new()
        .set_input("oi9n0g16.png")
        .set_expected("oi9n0g16_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandlerBuilder::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
                .finish()
        )
);
wali_test!(
    test_oi9n2c16_100h_100w_1e_1c,
    tranform_png_to_webp,
    case::<ImageHandler>::new()
        .set_input("oi9n2c16.png")
        .set_expected("oi9n2c16_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandlerBuilder::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
                .finish()
        )
);
wali_test!(
    test_ccwn2c08_100h_100w_1e_1c,
    tranform_png_to_webp,
    case::<ImageHandler>::new()
        .set_input("ccwn2c08.png")
        .set_expected("ccwn2c08_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandlerBuilder::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
                .finish()
        )
);
wali_test!(
    test_ccwn3p08_100h_100w_1e_1c,
    tranform_png_to_webp,
    case::<ImageHandler>::new()
        .set_input("ccwn3p08.png")
        .set_expected("ccwn3p08_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandlerBuilder::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
                .finish()
        )
);
wali_test!(
    test_cdfn2c08_100h_100w_1e_1c,
    tranform_png_to_webp,
    case::<ImageHandler>::new()
        .set_input("cdfn2c08.png")
        .set_expected("cdfn2c08_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandlerBuilder::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
                .finish()
        )
);
wali_test!(
    test_cdhn2c08_100h_100w_1e_1c,
    tranform_png_to_webp,
    case::<ImageHandler>::new()
        .set_input("cdhn2c08.png")
        .set_expected("cdhn2c08_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandlerBuilder::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
                .finish()
        )
);
wali_test!(
    test_cdsn2c08_100h_100w_1e_1c,
    tranform_png_to_webp,
    case::<ImageHandler>::new()
        .set_input("cdsn2c08.png")
        .set_expected("cdsn2c08_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandlerBuilder::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
                .finish()
        )
);
wali_test!(
    test_cdun2c08_100h_100w_1e_1c,
    tranform_png_to_webp,
    case::<ImageHandler>::new()
        .set_input("cdun2c08.png")
        .set_expected("cdun2c08_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandlerBuilder::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
                .finish()
        )
);
wali_test!(
    test_ch1n3p04_100h_100w_1e_1c,
    tranform_png_to_webp,
    case::<ImageHandler>::new()
        .set_input("ch1n3p04.png")
        .set_expected("ch1n3p04_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandlerBuilder::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
                .finish()
        )
);
wali_test!(
    test_ch2n3p08_100h_100w_1e_1c,
    tranform_png_to_webp,
    case::<ImageHandler>::new()
        .set_input("ch2n3p08.png")
        .set_expected("ch2n3p08_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandlerBuilder::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
                .finish()
        )
);
wali_test!(
    test_cm0n0g04_100h_100w_1e_1c,
    tranform_png_to_webp,
    case::<ImageHandler>::new()
        .set_input("cm0n0g04.png")
        .set_expected("cm0n0g04_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandlerBuilder::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
                .finish()
        )
);
wali_test!(
    test_cm7n0g04_100h_100w_1e_1c,
    tranform_png_to_webp,
    case::<ImageHandler>::new()
        .set_input("cm7n0g04.png")
        .set_expected("cm7n0g04_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandlerBuilder::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
                .finish()
        )
);
wali_test!(
    test_cm9n0g04_100h_100w_1e_1c,
    tranform_png_to_webp,
    case::<ImageHandler>::new()
        .set_input("cm9n0g04.png")
        .set_expected("cm9n0g04_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandlerBuilder::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
                .finish()
        )
);
wali_test!(
    test_cs3n2c16_100h_100w_1e_1c,
    tranform_png_to_webp,
    case::<ImageHandler>::new()
        .set_input("cs3n2c16.png")
        .set_expected("cs3n2c16_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandlerBuilder::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
                .finish()
        )
);
wali_test!(
    test_cs3n3p08_100h_100w_1e_1c,
    tranform_png_to_webp,
    case::<ImageHandler>::new()
        .set_input("cs3n3p08.png")
        .set_expected("cs3n3p08_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandlerBuilder::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
                .finish()
        )
);
wali_test!(
    test_cs5n2c08_100h_100w_1e_1c,
    tranform_png_to_webp,
    case::<ImageHandler>::new()
        .set_input("cs5n2c08.png")
        .set_expected("cs5n2c08_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandlerBuilder::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
                .finish()
        )
);
wali_test!(
    test_cs5n3p08_100h_100w_1e_1c,
    tranform_png_to_webp,
    case::<ImageHandler>::new()
        .set_input("cs5n3p08.png")
        .set_expected("cs5n3p08_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandlerBuilder::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
                .finish()
        )
);
wali_test!(
    test_cs8n2c08_100h_100w_1e_1c,
    tranform_png_to_webp,
    case::<ImageHandler>::new()
        .set_input("cs8n2c08.png")
        .set_expected("cs8n2c08_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandlerBuilder::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
                .finish()
        )
);
wali_test!(
    test_cs8n3p08_100h_100w_1e_1c,
    tranform_png_to_webp,
    case::<ImageHandler>::new()
        .set_input("cs8n3p08.png")
        .set_expected("cs8n3p08_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandlerBuilder::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
                .finish()
        )
);
wali_test!(
    test_ct0n0g04_100h_100w_1e_1c,
    tranform_png_to_webp,
    case::<ImageHandler>::new()
        .set_input("ct0n0g04.png")
        .set_expected("ct0n0g04_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandlerBuilder::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
                .finish()
        )
);
wali_test!(
    test_ct1n0g04_100h_100w_1e_1c,
    tranform_png_to_webp,
    case::<ImageHandler>::new()
        .set_input("ct1n0g04.png")
        .set_expected("ct1n0g04_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandlerBuilder::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
                .finish()
        )
);
wali_test!(
    test_cten0g04_100h_100w_1e_1c,
    tranform_png_to_webp,
    case::<ImageHandler>::new()
        .set_input("cten0g04.png")
        .set_expected("cten0g04_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandlerBuilder::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
                .finish()
        )
);
wali_test!(
    test_ctfn0g04_100h_100w_1e_1c,
    tranform_png_to_webp,
    case::<ImageHandler>::new()
        .set_input("ctfn0g04.png")
        .set_expected("ctfn0g04_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandlerBuilder::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
                .finish()
        )
);
wali_test!(
    test_ctgn0g04_100h_100w_1e_1c,
    tranform_png_to_webp,
    case::<ImageHandler>::new()
        .set_input("ctgn0g04.png")
        .set_expected("ctgn0g04_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandlerBuilder::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
                .finish()
        )
);
wali_test!(
    test_cthn0g04_100h_100w_1e_1c,
    tranform_png_to_webp,
    case::<ImageHandler>::new()
        .set_input("cthn0g04.png")
        .set_expected("cthn0g04_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandlerBuilder::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
                .finish()
        )
);
wali_test!(
    test_ctjn0g04_100h_100w_1e_1c,
    tranform_png_to_webp,
    case::<ImageHandler>::new()
        .set_input("ctjn0g04.png")
        .set_expected("ctjn0g04_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandlerBuilder::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
                .finish()
        )
);
wali_test!(
    test_ctzn0g04_100h_100w_1e_1c,
    tranform_png_to_webp,
    case::<ImageHandler>::new()
        .set_input("ctzn0g04.png")
        .set_expected("ctzn0g04_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandlerBuilder::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
                .finish()
        )
);
wali_test!(
    test_pp0n2c16_100h_100w_1e_1c,
    tranform_png_to_webp,
    case::<ImageHandler>::new()
        .set_input("pp0n2c16.png")
        .set_expected("pp0n2c16_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandlerBuilder::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
                .finish()
        )
);
wali_test!(
    test_pp0n6a08_100h_100w_1e_1c,
    tranform_png_to_webp,
    case::<ImageHandler>::new()
        .set_input("pp0n6a08.png")
        .set_expected("pp0n6a08_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandlerBuilder::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
                .finish()
        )
);
wali_test!(
    test_ps1n0g08_100h_100w_1e_1c,
    tranform_png_to_webp,
    case::<ImageHandler>::new()
        .set_input("ps1n0g08.png")
        .set_expected("ps1n0g08_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandlerBuilder::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
                .finish()
        )
);
wali_test!(
    test_ps1n2c16_100h_100w_1e_1c,
    tranform_png_to_webp,
    case::<ImageHandler>::new()
        .set_input("ps1n2c16.png")
        .set_expected("ps1n2c16_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandlerBuilder::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
                .finish()
        )
);
wali_test!(
    test_ps2n0g08_100h_100w_1e_1c,
    tranform_png_to_webp,
    case::<ImageHandler>::new()
        .set_input("ps2n0g08.png")
        .set_expected("ps2n0g08_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandlerBuilder::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
                .finish()
        )
);
wali_test!(
    test_ps2n2c16_100h_100w_1e_1c,
    tranform_png_to_webp,
    case::<ImageHandler>::new()
        .set_input("ps2n2c16.png")
        .set_expected("ps2n2c16_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandlerBuilder::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
                .finish()
        )
);
wali_test!(
    test_f00n0g08_100h_100w_1e_1c,
    tranform_png_to_webp,
    case::<ImageHandler>::new()
        .set_input("f00n0g08.png")
        .set_expected("f00n0g08_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandlerBuilder::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
                .finish()
        )
);
wali_test!(
    test_f00n2c08_100h_100w_1e_1c,
    tranform_png_to_webp,
    case::<ImageHandler>::new()
        .set_input("f00n2c08.png")
        .set_expected("f00n2c08_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandlerBuilder::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
                .finish()
        )
);
wali_test!(
    test_f01n0g08_100h_100w_1e_1c,
    tranform_png_to_webp,
    case::<ImageHandler>::new()
        .set_input("f01n0g08.png")
        .set_expected("f01n0g08_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandlerBuilder::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
                .finish()
        )
);
wali_test!(
    test_f01n2c08_100h_100w_1e_1c,
    tranform_png_to_webp,
    case::<ImageHandler>::new()
        .set_input("f01n2c08.png")
        .set_expected("f01n2c08_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandlerBuilder::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
                .finish()
        )
);
wali_test!(
    test_f02n0g08_100h_100w_1e_1c,
    tranform_png_to_webp,
    case::<ImageHandler>::new()
        .set_input("f02n0g08.png")
        .set_expected("f02n0g08_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandlerBuilder::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
                .finish()
        )
);
wali_test!(
    test_f02n2c08_100h_100w_1e_1c,
    tranform_png_to_webp,
    case::<ImageHandler>::new()
        .set_input("f02n2c08.png")
        .set_expected("f02n2c08_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandlerBuilder::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
                .finish()
        )
);
wali_test!(
    test_f03n0g08_100h_100w_1e_1c,
    tranform_png_to_webp,
    case::<ImageHandler>::new()
        .set_input("f03n0g08.png")
        .set_expected("f03n0g08_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandlerBuilder::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
                .finish()
        )
);
wali_test!(
    test_f03n2c08_100h_100w_1e_1c,
    tranform_png_to_webp,
    case::<ImageHandler>::new()
        .set_input("f03n2c08.png")
        .set_expected("f03n2c08_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandlerBuilder::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
                .finish()
        )
);
wali_test!(
    test_f04n0g08_100h_100w_1e_1c,
    tranform_png_to_webp,
    case::<ImageHandler>::new()
        .set_input("f04n0g08.png")
        .set_expected("f04n0g08_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandlerBuilder::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
                .finish()
        )
);
wali_test!(
    test_f04n2c08_100h_100w_1e_1c,
    tranform_png_to_webp,
    case::<ImageHandler>::new()
        .set_input("f04n2c08.png")
        .set_expected("f04n2c08_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandlerBuilder::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
                .finish()
        )
);
wali_test!(
    test_f99n0g04_100h_100w_1e_1c,
    tranform_png_to_webp,
    case::<ImageHandler>::new()
        .set_input("f99n0g04.png")
        .set_expected("f99n0g04_100h_100w_1e_1c.webp")
        .set_param(
            ImageHandlerBuilder::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
                .finish()
        )
);
