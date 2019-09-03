use imagers::gif_encode_webp;
use imagers::ImageHandler;
use imagers::ImageHandlerBuilder;
use imagers::Resize;

use crate::Case;
use crate::TestConfig;
use crc::{crc32, Hasher32};
use std::fs;
use std::path::Path;

fn tranform_gif_to_webp(config: &TestConfig, case: Case<ImageHandler>) -> Result<(), String> {
    let input: String = case.input;
    let expected: String = case.expected;
    let is_corrupted: bool = case.is_corrupted;
    let im: ImageHandler = case.param;
    let mut fact_input = config.input.clone();
    fact_input.push_str("gif/");
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

    let result = match gif_encode_webp(&mut data.clone(), im) {
        Ok(result) => result,
        Err(err) => {
            if is_corrupted {
                return Ok(());
            } else {
                return Err(format!("{} fail at gif_encode_webp. {}", input, err));
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

    fs::write(Path::new(Path::new(&fact_output)), result.pic).unwrap();

    if digest_expected.sum32() != digest_output.sum32() {
        return Err(format!("{} fail to test", input));
    }
    return Ok(());
}

wali_test!(
    test_giphy_1s,
    tranform_gif_to_webp,
    Case::<ImageHandler>::new()
        .set_input("giphy.gif")
        .set_expected("giphy_1s.webp")
        .set_param(ImageHandlerBuilder::new().set_first_frame(true).finish())
);
wali_test!(
    test_giphy_100h_100w_1s,
    tranform_gif_to_webp,
    Case::<ImageHandler>::new()
        .set_input("giphy.gif")
        .set_expected("giphy_100h_100w_1s.webp")
        .set_param(
            ImageHandlerBuilder::new()
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
                .set_first_frame(true)
                .finish()
        )
);
wali_test!(
    test_giphy,
    tranform_gif_to_webp,
    Case::<ImageHandler>::new()
        .set_input("giphy.gif")
        .set_expected("giphy.webp")
        .set_param(
            ImageHandlerBuilder::new()
                .finish()
        )
);

wali_test!(
    test_giphy_100h_100w,
    tranform_gif_to_webp,
    Case::<ImageHandler>::new()
        .set_input("giphy.gif")
        .set_expected("giphy_100h_100w.webp")
        .set_param(
            ImageHandlerBuilder::new()
                .set_resize(Some(Resize {
                    width: 100,
                    height: 100,
                }))
                .set_first_frame(true)
                .finish()
        )
);