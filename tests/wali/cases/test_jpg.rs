use imagers::jpg_encode_webp;
use imagers::ImageHandler;
use imagers::ImageHandlerBuilder;
use imagers::Resize;

use crate::Case;
use crate::TestConfig;
use crc::{crc32, Hasher32};
use std::fs;
use std::path::Path;

fn tranform_jpg_to_webp(config: &TestConfig, case: Case<ImageHandler>) -> Result<(), String> {
    let input: String = case.input;
    let expected: String = case.expected;
    let is_corrupted: bool = case.is_corrupted;
    let im: ImageHandler = case.param;
    let mut fact_input = config.input.clone();
    fact_input.push_str("jpg/");
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

    let result = match jpg_encode_webp(&data.clone(), im) {
        Ok(result) => result,
        Err(err) => {
            if is_corrupted {
                return Ok(());
            } else {
                return Err(format!("{} fail at jpg_encode_webp. {}", input, err));
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
    Ok(())
}

// article/f946c1d2884e16301d5d43f3ccf917cc14015619.jpg@702w_212h_1e_1c.webp
wali_test!(
    test_f946c1d2884e16301d5d43f3ccf917cc14015619_702w_212h_1e_1c,
    tranform_jpg_to_webp,
    Case::<ImageHandler>::new()
        .set_input("f946c1d2884e16301d5d43f3ccf917cc14015619.jpg")
        .set_expected("f946c1d2884e16301d5d43f3ccf917cc14015619_702w_212h_1e_1c.webp")
        .set_param(
            ImageHandlerBuilder::new()
                .set_resize(Some(Resize {
                    width: 702,
                    height: 212,
                }))
                .set_edge(1)
                .set_auto_crop(true)
                .finish()
        )
);
// face/fc0c7f707fcc4266ab074037f5a9d8fd028d702a.jpg
wali_test!(
    test_fc0c7f707fcc4266ab074037f5a9d8fd028d702a_80w_80h,
    tranform_jpg_to_webp,
    Case::<ImageHandler>::new()
        .set_input("fc0c7f707fcc4266ab074037f5a9d8fd028d702a.jpg")
        .set_expected("fc0c7f707fcc4266ab074037f5a9d8fd028d702a_80w_80h.webp")
        .set_param(
            ImageHandlerBuilder::new()
                .set_resize(Some(Resize {
                    width: 80,
                    height: 80,
                }))
                .finish()
        )
);
// /bfs/article/f62362ca926c3c4807128309fd1e56729835e9ab.jpg@400w_400h_1e_1c.webp
wali_test!(
    test_f62362ca926c3c4807128309fd1e56729835e9ab_400w_400h_1e_1c,
    tranform_jpg_to_webp,
    Case::<ImageHandler>::new()
        .set_input("f62362ca926c3c4807128309fd1e56729835e9ab.jpg")
        .set_expected("f62362ca926c3c4807128309fd1e56729835e9ab_400w_400h_1e_1c.webp")
        .set_param(
            ImageHandlerBuilder::new()
                .set_resize(Some(Resize {
                    width: 400,
                    height: 400,
                }))
                .set_edge(1)
                .set_auto_crop(true)
                .finish()
        )
);