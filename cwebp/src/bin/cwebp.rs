use clap::{App, Arg};

mod util;

use std::fs;

use imagers::ImageFormat;

use crate::util::jpg2webp::jpg_encode_webp;
use crate::util::param::Crop;
use crate::util::param::RegionCrop;
use crate::util::param::Resize;
use crate::util::png2webp::png_encode_webp;
use crate::util::webp2webp::webp_encode_webp;
use crate::util::ImageHandler;

fn main() {
    let matches = App::new("cwebp")
        .author("hawkingrei <hawkingrei@gmail.com>")
        .arg(
            Arg::with_name("resize")
                .long("resize")
                .takes_value(true)
                .number_of_values(2)
                .help("-resize width height"),
        )
        .arg(
            Arg::with_name("crop")
                .long("crop")
                .takes_value(true)
                .number_of_values(4)
                .help("-crop x_position y_position width height"),
        )
        .arg(
            Arg::with_name("rc")
                .long("rc")
                .takes_value(true)
                .number_of_values(4)
                .help("-rc width height rc"),
        )
        .arg(Arg::with_name("c").help("auto crop").short("c"))
        .arg(Arg::with_name("e").long("e").takes_value(true).help("edge"))
        .arg(
            Arg::with_name("p")
                .long("p")
                .takes_value(true)
                .help("proportion"),
        )
        .arg(
            Arg::with_name("o")
                .long("o")
                .takes_value(true)
                .help("output"),
        )
        .arg(
            Arg::with_name("i")
                .long("i")
                .takes_value(true)
                .help("input"),
        )
        .get_matches();

    let proportion: i32 = matches.value_of("p").unwrap_or("0").parse::<i32>().unwrap();
    let edge: i32 = matches.value_of("e").unwrap_or("0").parse::<i32>().unwrap();

    let auto_crop: bool = matches.is_present("c");
    let crop: Vec<i32> = match matches.values_of("crop") {
        Some(data) => data.map(|param| param.parse().unwrap()).collect(),
        None => vec![],
    };
    let resize: Vec<i32> = match matches.values_of("resize") {
        Some(data) => data.map(|param| param.parse().unwrap()).collect(),
        None => vec![],
    };
    let region_crop: Vec<i32> = match matches.values_of("rc") {
        Some(data) => data.map(|param| param.parse().unwrap()).collect(),
        None => vec![],
    };

    let output = matches.value_of("o").unwrap_or("out.webp");
    let input = matches.value_of("i").unwrap();

    let mut param = ImageHandler::new()
        .set_proportion(proportion)
        .set_edge(edge)
        .set_auto_crop(auto_crop)
        .set_crop(if crop.len() == 4 {
            Some(Crop {
                x: crop[0],
                y: crop[1],
                width: crop[2],
                height: crop[3],
            })
        } else {
            None
        })
        .set_resize(if resize.len() == 2 {
            Some(Resize {
                width: resize[0],
                height: resize[1],
            })
        } else {
            None
        })
        .set_region_crop(if region_crop.len() == 3 {
            Some(RegionCrop {
                width: region_crop[0],
                height: region_crop[1],
                Region: region_crop[2],
            })
        } else {
            None
        });

    let data = fs::read(input).unwrap();
    let ptype = imagers::guess_format(&data).unwrap();
    match ptype {
        ImageFormat::PNG => {
            let result = png_encode_webp(&data.clone(), param).unwrap();
            fs::write(output, result);
        }
        ImageFormat::JPEG => {
            let result = jpg_encode_webp(&data.clone(), param).unwrap();
            fs::write(output, result);
        }
        ImageFormat::WEBP => {
            let result = webp_encode_webp(&data.clone(), param).unwrap();
            fs::write(output, result);
        }
        _ => println!("not support "),
    }
}

#[cfg(test)]
mod tests {
    use crate::util::ImageHandler;

    pub struct test_case {}

    use std::path::Path;
    #[test]
    fn test_png_to_webp() {
        let dateset_path = Path::new("./dataset");
    }
}
