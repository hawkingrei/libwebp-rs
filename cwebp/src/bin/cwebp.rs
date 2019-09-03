use clap::{App, Arg};

use std::fs;

use imagers::ImageFormat;

use imagers::gif_encode_webp;
use imagers::gif_info;
use imagers::jpg_encode_webp;
use imagers::png_encode_webp;
use imagers::webp_encode_webp;
use imagers::Crop;
use imagers::ImageHandlerBuilder;
use imagers::RegionCrop;
use imagers::Resize;

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
        .arg(Arg::with_name("fc").long("fc").help("-rc width height rc"))
        .arg(Arg::with_name("c").help("auto crop").short("c"))
        .arg(Arg::with_name("f").help("first frame").short("f"))
        .arg(Arg::with_name("profile").help("profile"))
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
    let get_frame_count: bool = matches.is_present("fc");
    let auto_crop: bool = matches.is_present("c");
    let first_frame: bool = matches.is_present("f");
    let profile: bool = matches.is_present("profile");
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

    let param = ImageHandlerBuilder::new()
        .set_proportion(proportion)
        .set_edge(edge)
        .set_first_frame(first_frame)
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
            if resize[0] == 0 && resize[1] == 0 {
                None
            } else {
                Some(Resize {
                    width: resize[0],
                    height: resize[1],
                })
            }
        } else {
            None
        })
        .set_region_crop(if region_crop.len() == 3 {
            Some(RegionCrop {
                width: region_crop[0],
                height: region_crop[1],
                region: region_crop[2],
            })
        } else {
            None
        })
        .finish();
    let data = fs::read(input).unwrap();
    let ptype = imagers::guess_format(&data).unwrap();
    loop {
        match ptype {
            ImageFormat::PNG => {
                let result = png_encode_webp(&data.clone(), param).unwrap();
                println!("height: {}", result.height);
                println!("width: {}", result.width);
                if !profile {
                    fs::write(output, result.pic).unwrap();
                }
            }
            ImageFormat::JPEG => {
                let result = jpg_encode_webp(&data.clone(), param).unwrap();
                println!("height: {}", result.height);
                println!("width: {}", result.width);
                if !profile {
                    fs::write(output, result.pic).unwrap();
                }
            }
            ImageFormat::WEBP => {
                let result = webp_encode_webp(&data.clone(), param).unwrap();
                println!("height: {}", result.height);
                println!("width: {}", result.width);
                if !profile {
                    fs::write(output, result.pic).unwrap();
                }
            }
            ImageFormat::GIF => {
                if get_frame_count {
                    dbg!("yes");
                    match gif_info(&mut data.clone()) {
                        Ok(result) => {
                            println!("frame count: {:?}", result);
                        }
                        Err(e) => println!("{}", e),
                    }
                    return;
                }
                let result = gif_encode_webp(&mut data.clone(), param).unwrap();
                if !profile {
                    fs::write(output, result.pic).unwrap();
                }
            }
            _ => println!("not support"),
        }
        if !profile {
            break;
        }
    }
}
