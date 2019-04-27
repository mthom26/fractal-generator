use image;
use num_complex;

fn main() {
    let img_width = 1000;
    let img_height = 1000;

    let scale = 1.0;
    let aspect_ratio = img_width as f32 / img_height as f32;

    let offset_x = scale * 0.5;
    let offset_y = scale * 0.5 * aspect_ratio;

    let scale_x = scale * aspect_ratio / img_width as f32;
    let scale_y = scale / img_height as f32;

    let mut img_buffer = image::ImageBuffer::new(img_width, img_height);

    // Fill image background
    for (_, _, pixel) in img_buffer.enumerate_pixels_mut() {
        *pixel = image::Rgb([25, 25, 25]);
    }

    // Constant complex number to use for each iteration
    let c = num_complex::Complex::new(-0.4, 0.6);

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
    }
    
    let filename = format!("output_r{}_i{}.png", c.re, c.im);
    img_buffer.save(filename).unwrap();
}
