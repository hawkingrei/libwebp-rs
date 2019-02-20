use clap::{App, Arg};

mod util;

use std::fs;

use imagers::ImageFormat;

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
    let crop: Vec<i32> = match matches.values_of("crop") {
        Some(data) => data.map(|param| param.parse().unwrap()).collect(),
        None => vec![],
    };
    let resize: Vec<i32> = match matches.values_of("resize") {
        Some(data) => data.map(|param| param.parse().unwrap()).collect(),
        None => vec![],
    };

    let output = matches.value_of("o").unwrap_or("out.webp");
    let input = matches.value_of("i").unwrap();

    let data = fs::read(input).unwrap();
    let ptype = imagers::guess_format(data).unwrap();
    match ptype {
        ImageFormat::PNG => println!("png"),
        ImageFormat::JPEG => println!("jpeg"),
        _ => println!("not support "),
    }
}
