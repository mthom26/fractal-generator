use image;
use indicatif::{ProgressBar, ProgressStyle};
use num_complex;
mod constants;

mod config;
use config::Config;

fn main() {
    let cfg = Config::new();
    println!("{:#?}", cfg);
    let img_width = cfg.dimensions.0;
    let img_height = cfg.dimensions.1;

    let scale = cfg.scale;
    let aspect_ratio = img_width as f32 / img_height as f32;

    let offset_x = scale * cfg.offsets.0;
    let offset_y = scale * cfg.offsets.1 * aspect_ratio;

    let scale_x = scale * aspect_ratio / img_width as f32;
    let scale_y = scale / img_height as f32;

    let mut img_buffer = image::ImageBuffer::new(img_width, img_height);

    // Fill image background
    for (_, _, pixel) in img_buffer.enumerate_pixels_mut() {
        *pixel = image::Rgb([25, 25, 25]);
    }

    // Constant complex number to use for each iteration
    let c = num_complex::Complex::new(cfg.complex_num[0].0, cfg.complex_num[0].1);

    println!("Building pattern...");
    let progress = ProgressBar::new(img_width as u64);
    progress.set_style(
        ProgressStyle::default_bar()
            .template("{bar:80.green.dim/white.dim} {pos:>4}/{len} [{elapsed}]"),
    );

    for x in 0..img_width {
        for y in 0..img_height {
            let zx = y as f32 * scale_x - offset_x;
            let zy = x as f32 * scale_y - offset_y;

            let mut z = num_complex::Complex::new(zx, zy);

            let mut i = 0;
            while i < 255 && z.norm() <= 2.0 {
                z = z * z + c;
                i += 1;
            }

            let pixel = img_buffer.get_pixel_mut(x, y);
            *pixel = image::Rgb([25, i, 25]);
        }
        progress.inc(1);
    }
    progress.finish();

    let filename = format!("output_{}_{}i.png", c.re, c.im);
    img_buffer.save(filename).unwrap();
}
