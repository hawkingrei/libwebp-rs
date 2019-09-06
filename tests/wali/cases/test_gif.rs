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

    let result = match gif_encode_webp(&data.clone(), im) {
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
    Ok(())
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
        .set_param(ImageHandlerBuilder::new().finish())
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
// https://i0.hdslb.com/bfs/album/bc49f10ae946117a4ba9e96916f14839d0d7f1a2.gif@1080w_1080h_1c_1e_1s.webp
wali_test!(
    test_bc49f10ae946117a4ba9e96916f14839d0d7f1a2_702w_212h_1e_1c,
    tranform_gif_to_webp,
    Case::<ImageHandler>::new()
        .set_input("bc49f10ae946117a4ba9e96916f14839d0d7f1a2.gif")
        .set_expected("bc49f10ae946117a4ba9e96916f14839d0d7f1a2_1080w_1080h_1c_1e.webp")
        .set_param(
            ImageHandlerBuilder::new()
                .set_resize(Some(Resize {
                    width: 1080,
                    height: 1080,
                }))
                .set_edge(1)
                .set_auto_crop(true)
                .finish()
        )
);
wali_test!(
    test_bc49f10ae946117a4ba9e96916f14839d0d7f1a2,
    tranform_gif_to_webp,
    Case::<ImageHandler>::new()
        .set_input("bc49f10ae946117a4ba9e96916f14839d0d7f1a2.gif")
        .set_expected("bc49f10ae946117a4ba9e96916f14839d0d7f1a2.webp")
        .set_param(ImageHandlerBuilder::new().finish())
);
// /bfs/album/608782fe6fe8eaf6105f41f3131157877af9de2b.gif@400w_400h_1c_1e_1s.webp
wali_test!(
    test_608782fe6fe8eaf6105f41f3131157877af9de2b,
    tranform_gif_to_webp,
    Case::<ImageHandler>::new()
        .set_input("608782fe6fe8eaf6105f41f3131157877af9de2b.gif")
        .set_expected("608782fe6fe8eaf6105f41f3131157877af9de2b.webp")
        .set_param(ImageHandlerBuilder::new().finish())
);
// /bfs/face/b8a58f3bbf8f926ec10c042ea997f01f9fe926ab.gif@90w_90h_1e_1c_85q
wali_test!(
    test_b8a58f3bbf8f926ec10c042ea997f01f9fe926ab,
    tranform_gif_to_webp,
    Case::<ImageHandler>::new()
        .set_input("b8a58f3bbf8f926ec10c042ea997f01f9fe926ab.gif")
        .set_expected("b8a58f3bbf8f926ec10c042ea997f01f9fe926ab.webp")
        .set_param(ImageHandlerBuilder::new().finish())
);
