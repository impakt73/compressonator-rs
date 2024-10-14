use std::fs;

use compressonator::{compress_image_bc7, decompress_image_bc7, load_image_data};
use image::{Rgb, RgbImage};

pub fn main() {
    let image_data = load_image_data("test.jpg").unwrap();
    let compressed_data = compress_image_bc7(image_data.width, image_data.height, &image_data.pixels, 1.0).unwrap();
    let decompressed_data =
        decompress_image_bc7(image_data.width, image_data.height, &compressed_data).unwrap();
    let mut output_img = RgbImage::new(decompressed_data.width, decompressed_data.height);
    for y in 0..(decompressed_data.height) {
        for x in 0..(decompressed_data.width) {
            let offset = ((y * decompressed_data.width + x) * 4) as usize;
            let r = decompressed_data.pixels[offset];
            let g = decompressed_data.pixels[offset + 1];
            let b = decompressed_data.pixels[offset + 2];
            output_img.put_pixel(x, y, Rgb([r, g, b]));
        }
    }
    fs::write("output.tex", &compressed_data).unwrap();
    output_img.save("output.jpg").unwrap();
}
