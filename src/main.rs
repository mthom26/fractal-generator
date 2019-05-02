use std::thread;

use image;
use indicatif::{ProgressBar, ProgressStyle, MultiProgress};
use num_complex;
mod constants;

mod config;
use config::Config;

fn main() {
    let cfg = Config::new();
    // println!("{:#?}", cfg);
    let img_width = cfg.dimensions.0;
    let img_height = cfg.dimensions.1;

    let scale = cfg.scale;
    let aspect_ratio = img_width as f32 / img_height as f32;

    let offset_x = scale * cfg.offsets.0;
    let offset_y = scale * cfg.offsets.1 * aspect_ratio;

    let scale_x = scale * aspect_ratio / img_width as f32;
    let scale_y = scale / img_height as f32;

    let p_manager = MultiProgress::new();
    let p_style = ProgressStyle::default_bar()
        .template("{msg} {bar:80.green/white} {pos:>4}/{len} [{elapsed}]")
        .progress_chars("=>-");

    let mut workers = Vec::new();
    for num in cfg.complex_num.iter() {
        let mut img_buffer = image::ImageBuffer::new(img_width, img_height);

        // Fill image background
        for (_, _, pixel) in img_buffer.enumerate_pixels_mut() {
            *pixel = image::Rgb([25, 25, 25]);
        }

        let p_bar = p_manager.add(ProgressBar::new(img_width as u64));
        p_bar.set_style(p_style.clone());
        p_bar.set_message("Running   ");
        let c = num_complex::Complex::new(num.0, num.1);

        let worker = thread::spawn(move || {
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
                p_bar.inc(1);
            }
            p_bar.finish_with_message("Finished  ");
            // TODO - Make sure no two files have the same name 
            // for example two identical complex numbers are passed
            let filename = format!("output_{}_{}i.png", c.re, c.im);
            img_buffer.save(filename).unwrap();
        });
        workers.push(worker);
    }

    p_manager.join().unwrap();
    // Wait for threads to finish working
    for t in workers.into_iter() {
        t.join().unwrap();
    }
}
