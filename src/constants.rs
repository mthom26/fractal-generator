#[cfg_attr(rustfmt, rustfmt_skip)]
pub const SCALE_LONG_HELP: &str = "The scale of the fractal pattern. A higher \
value will result in a smaller fractal pattern.\n";

#[cfg_attr(rustfmt, rustfmt_skip)]
pub const OFFSETS_LONG_HELP: &str = "The x and y offsets of the fractal pattern. \
Values of 0.5 and 0.5 will place the pattern in the middle of the image. Values \
greater than 1.0 or less than 0.0 will shift the pattern outside the image. Too \
large or small a value may place the pattern completely outside the visible space.";

#[cfg_attr(rustfmt, rustfmt_skip)]
pub const DIMENSIONS_LONG_HELP: &str = "The dimensions of the output image in \
pixels. The first value is the width. The second value is the height. If no \
args are supplied the default is 1000 by 1000 pixels.";

#[cfg_attr(rustfmt, rustfmt_skip)]
pub const CONFIG_LONG_HELP: &str = "The config file to use. Should be valid json.";
