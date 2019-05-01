use crate::constants::{
    COMPLEX_NUM_LONG_HELP, CONFIG_LONG_HELP, DIMENSIONS_LONG_HELP, OFFSETS_LONG_HELP,
    SCALE_LONG_HELP,
};
use std::fs;

use clap::{App, Arg};
use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub dimensions: (u32, u32),
    pub scale: f32,
    pub offsets: (f32, f32),
    pub complex_num: (f32, f32),
}

impl Config {
    pub fn new() -> Self {
        let matches = App::new("Fractal Generator")
            .arg(
                Arg::with_name("dimensions")
                    .help("The image width and height")
                    .long_help(DIMENSIONS_LONG_HELP)
                    .short("d")
                    .long("dimensions")
                    .takes_value(true)
                    .number_of_values(2)
                    .value_names(&["width", "height"]),
            )
            .arg(
                Arg::with_name("scale")
                    .help("The scale of the fractal pattern")
                    .long_help(SCALE_LONG_HELP)
                    .short("s")
                    .long("scale")
                    .takes_value(true)
                    .value_name("scale")
                    .default_value("1.0"),
            )
            .arg(
                Arg::with_name("offsets")
                    .help("The offset of the fractal origin")
                    .long_help(OFFSETS_LONG_HELP)
                    .short("o")
                    .long("offset")
                    .takes_value(true)
                    .number_of_values(2)
                    .value_names(&["offset X", "offset Y"]),
            )
            .arg(
                Arg::with_name("complex_num")
                    .help("The complex number to use as a constant")
                    .long_help(COMPLEX_NUM_LONG_HELP)
                    .short("n")
                    .long("number")
                    .takes_value(true)
                    .number_of_values(2)
                    .allow_hyphen_values(true)
                    .value_names(&["real", "imaginary"]),
            )
            .arg(
                Arg::with_name("config")
                    .help("Use a config.json file")
                    .long_help(CONFIG_LONG_HELP)
                    .short("c")
                    .long("config")
                    .takes_value(true)
                    .value_name("config"),
            )
            .get_matches();

        if let Some(val) = matches.value_of("config") {
            let file_content = fs::read_to_string(val).expect("Could not read config file");
            let config: Config = serde_json::from_str(&file_content).unwrap();
            return config;
        }
        
        let scale: f32 = matches.value_of("scale").unwrap().parse().unwrap();

        let dimensions: (u32, u32) = match matches.is_present("dimensions") {
            true => {
                let mut dimensions = matches.values_of("dimensions").unwrap();
                let offset_x: u32 = match dimensions.next() {
                    Some(val) => val.parse().unwrap(),
                    None => 1000,
                };
                let offset_y: u32 = match dimensions.next() {
                    Some(val) => val.parse().unwrap(),
                    None => 1000,
                };
                (offset_x, offset_y)
            }
            false => (1000, 1000),
        };

        let offsets: (f32, f32) = match matches.is_present("offsets") {
            true => {
                let mut offsets = matches.values_of("offsets").unwrap();
                let offset_x: f32 = match offsets.next() {
                    Some(val) => val.parse().unwrap(),
                    None => 0.5,
                };
                let offset_y: f32 = match offsets.next() {
                    Some(val) => val.parse().unwrap(),
                    None => 0.5,
                };
                (offset_x, offset_y)
            }
            false => (0.5, 0.5),
        };

        let complex_num: (f32, f32) = match matches.is_present("complex_num") {
            true => {
                let mut offsets = matches.values_of("complex_num").unwrap();
                let re: f32 = match offsets.next() {
                    Some(val) => val.parse().unwrap(),
                    None => 0.4,
                };
                let im: f32 = match offsets.next() {
                    Some(val) => val.parse().unwrap(),
                    None => 0.6,
                };
                (re, im)
            }
            false => (-0.4, 0.6),
        };

        Config {
            dimensions,
            scale,
            offsets,
            complex_num,
        }
    }
}
