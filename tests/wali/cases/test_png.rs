use imagers::ImageHandler;

use crate::case;
use crate::test_config;
use crate::wali;
use crc::{crc32, Hasher32};
use imagers::png_encode_webp;
use imagers::Resize;
use std::fs;
use std::path::Path;

fn tranform_png_to_webp(
    config: &test_config,
    input: String,
    expected: String,
    is_corrupted: bool,
    im: ImageHandler,
) -> Result<(), String> {
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
    digest_output.write(result.as_slice());

    fs::write(Path::new(Path::new(&fact_output)), result);

    if digest_expected.sum32() != digest_output.sum32() {
        return Err(format!("{} fail to test", input));
    }
    return Ok(());
}

#[test]
fn test_png_to_webp() {
    let conf: test_config = Default::default();
    let testor: wali<ImageHandler> = wali {
        config: conf,
        case: vec![],
        test_fn: Box::new(tranform_png_to_webp),
    };
    testor
        .insert_case(case::<ImageHandler> {
            input: "080fec32d3e0eeb407aafa1fbad7637c21e51601.png".to_string(),
            expected: "080fec32d3e0eeb407aafa1fbad7637c21e51601_472w_265h_1e_1c.webp".to_string(),
            is_corrupted: false,
            param: ImageHandler::new()
                .set_edge(1)
                .set_auto_crop(true)
                .set_resize(Some(Resize {
                    width: 472,
                    height: 265,
                })),
        })
        .insert_case(case::<ImageHandler> {
            input: "xs1n0g01.png".to_string(),
            expected: "".to_string(),
            is_corrupted: true,
            param: ImageHandler::new(),
        })
        .insert_case(case::<ImageHandler> {
            input: "xs2n0g01.png".to_string(),
            expected: "".to_string(),
            is_corrupted: true,
            param: ImageHandler::new(),
        })
        .insert_case(case::<ImageHandler> {
            input: "xs4n0g01.png".to_string(),
            expected: "".to_string(),
            is_corrupted: true,
            param: ImageHandler::new(),
        })
        .insert_case(case::<ImageHandler> {
            input: "xs4n0g01.png".to_string(),
            expected: "".to_string(),
            is_corrupted: true,
            param: ImageHandler::new(),
        })
        .insert_case(case::<ImageHandler> {
            input: "xs7n0g01.png".to_string(),
            expected: "".to_string(),
            is_corrupted: true,
            param: ImageHandler::new(),
        })
        .insert_case(case::<ImageHandler> {
            input: "xcrn0g04.png".to_string(),
            expected: "".to_string(),
            is_corrupted: true,
            param: ImageHandler::new(),
        })
        .insert_case(case::<ImageHandler> {
            input: "xlfn0g04.png".to_string(),
            expected: "".to_string(),
            is_corrupted: true,
            param: ImageHandler::new(),
        })
        .insert_case(case::<ImageHandler> {
            input: "xhdn0g08.png".to_string(),
            expected: "".to_string(),
            is_corrupted: true,
            param: ImageHandler::new(),
        })
        .insert_case(case::<ImageHandler> {
            input: "xc1n0g08.png".to_string(),
            expected: "".to_string(),
            is_corrupted: true,
            param: ImageHandler::new(),
        })
        .insert_case(case::<ImageHandler> {
            input: "xc9n2c08.png".to_string(),
            expected: "".to_string(),
            is_corrupted: true,
            param: ImageHandler::new(),
        })
        .insert_case(case::<ImageHandler> {
            input: "xd0n2c08.png".to_string(),
            expected: "".to_string(),
            is_corrupted: true,
            param: ImageHandler::new(),
        })
        .insert_case(case::<ImageHandler> {
            input: "xd0n2c08.png".to_string(),
            expected: "".to_string(),
            is_corrupted: true,
            param: ImageHandler::new(),
        })
        .insert_case(case::<ImageHandler> {
            input: "xd3n2c08.png".to_string(),
            expected: "".to_string(),
            is_corrupted: true,
            param: ImageHandler::new(),
        })
        .insert_case(case::<ImageHandler> {
            input: "xd9n2c08.png".to_string(),
            expected: "".to_string(),
            is_corrupted: true,
            param: ImageHandler::new(),
        })
        .verify();
}
