use clap::{App, Arg};
use crate::constants::{OFFSETS_LONG_HELP, SCALE_LONG_HELP};

pub struct Config {
    pub img_width: u32,
    pub img_height: u32,
    pub scale: f32,
    pub offsets: (f32, f32)
}

impl Config {
    pub fn new() -> Self {
        let matches = App::new("Fractal Generator")
            .arg(
                Arg::with_name("img_width")
                    .help("The image width to output")
                    .short("w")
                    .long("width")
                    .takes_value(true)
                    .value_name("width")
                    .default_value("1000"),
            )
            .arg(
                Arg::with_name("img_height")
                    .help("The image height to output")
                    .short("h")
                    .long("height")
                    .takes_value(true)
                    .value_name("height")
                    .default_value("1000"),
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
                    .value_names(&["offset X", "offset Y"])
            )
            .get_matches();

        let img_width: u32 = matches.value_of("img_width").unwrap().parse().unwrap();
        let img_height: u32 = matches.value_of("img_height").unwrap().parse().unwrap();
        let scale: f32 = matches.value_of("scale").unwrap().parse().unwrap();
        
        let offsets: (f32, f32) = match matches.is_present("offsets") {
            true => {
                let mut offsets = matches.values_of("offsets").unwrap();
                let offset_x: f32 = match offsets.next() {
                    Some(val) => val.parse().unwrap(),
                    None => 0.5
                };
                let offset_y: f32 = match offsets.next() {
                    Some(val) => val.parse().unwrap(),
                    None => 0.5
                };
                (offset_x, offset_y)
            },
            false => (0.5, 0.5)
        };

        Config {
            img_width,
            img_height,
            scale,
            offsets
        }
    }
}
