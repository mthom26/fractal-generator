use clap::{App, Arg};

pub struct Config {
    pub img_width: u32,
    pub img_height: u32,
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
            .get_matches();

        let img_width: u32 = matches.value_of("img_width").unwrap().parse().unwrap();
        let img_height: u32 = matches.value_of("img_height").unwrap().parse().unwrap();

        Config {
            img_width,
            img_height,
        }
    }
}
