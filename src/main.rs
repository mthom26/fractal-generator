use image;

fn main() {
    let img_width = 1000;
    let img_height = 1000;

    let mut img_buffer = image::ImageBuffer::new(img_width, img_height);

    // Fill image background
    for (_, _, pixel) in img_buffer.enumerate_pixels_mut() {
        *pixel = image::Rgb([25, 25, 25]);
    }

    img_buffer.save("output.jpeg").unwrap();
}
