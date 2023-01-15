use std::os::raw::c_void;

use compressonator_sys as sys;

use anyhow::Result;
use image::io::Reader as ImageReader;

fn compress_block_bc7(options: *mut c_void, input_block: &[u8], output_block: &mut [u8]) -> bool {
    unsafe {
        sys::CompressBlockBC7(input_block.as_ptr(), 16, output_block.as_mut_ptr(), options) == 0
    }
}

#[allow(dead_code)]
fn compress_block_bc5(options: *mut c_void, input_block: &[u8], output_block: &mut [u8]) -> bool {
    unsafe {
        sys::CompressBlockBC5(
            input_block.as_ptr(),
            4,
            input_block.as_ptr().add(16),
            4,
            output_block.as_mut_ptr(),
            options,
        ) == 0
    }
}

#[allow(dead_code)]
fn compress_block_bc4(input_block: &[u8], output_block: &mut [u8]) -> bool {
    unsafe {
        sys::CompressBlockBC4(
            input_block.as_ptr(),
            4,
            output_block.as_mut_ptr(),
            core::ptr::null(),
        ) == 0
    }
}

fn decompress_block_bc7(input_block: &[u8], output_block: &mut [u8]) -> bool {
    unsafe {
        sys::DecompressBlockBC7(
            input_block.as_ptr(),
            output_block.as_mut_ptr(),
            core::ptr::null(),
        ) == 0
    }
}

#[allow(dead_code)]
fn decompress_block_bc5(input_block: &[u8], output_block: &mut [u8]) -> bool {
    unsafe {
        sys::DecompressBlockBC5(
            input_block.as_ptr(),
            output_block.as_mut_ptr(),
            output_block.as_mut_ptr().add(16),
            core::ptr::null(),
        ) == 0
    }
}

#[allow(dead_code)]
fn decompress_block_bc4(input_block: &[u8], output_block: &mut [u8]) -> bool {
    unsafe {
        sys::DecompressBlockBC4(
            input_block.as_ptr(),
            output_block.as_mut_ptr(),
            core::ptr::null(),
        ) == 0
    }
}

pub struct RGBAImageData {
    pub width: u32,
    pub height: u32,
    pub pixels: Vec<u8>,
}

pub fn load_image_data(path: &str) -> Result<RGBAImageData> {
    let image = ImageReader::open(path)?.decode()?.into_rgba8();
    let width = image.width();
    let height = image.height();
    let pixels = image.into_raw();

    let image_data = RGBAImageData {
        width,
        height,
        pixels,
    };

    Ok(image_data)
}

pub fn compress_image_bc7(width: u32, height: u32, pixels: &[u8], quality: f32) -> Result<Vec<u8>> {
    assert!(width % 4 == 0);
    assert!(height % 4 == 0);

    let width_in_blocks = width / 4;
    let height_in_blocks = height / 4;
    let num_blocks = width_in_blocks * height_in_blocks;

    let mut input_block = [0_u8; 64];
    let mut output_block = [0_u8; 16];

    let mut options = core::ptr::null_mut();
    unsafe {
        sys::CreateOptionsBC7(&mut options);
        sys::SetQualityBC7(options, quality);
    }

    let mut output_blocks = Vec::with_capacity((num_blocks * 16) as usize);

    for block_y in 0..height_in_blocks {
        for block_x in 0..width_in_blocks {
            for y in 0..4 {
                for x in 0..4 {
                    let index =
                        (((block_y * 4 + y) * width + (block_x * 4 + x)) * 4) as usize;
                    let pixel = [
                        pixels[index],
                        pixels[index + 1],
                        pixels[index + 2],
                        pixels[index + 3],
                    ];
                    input_block[((y * 4 + x) * 4) as usize] = pixel[0];
                    input_block[(((y * 4 + x) * 4) + 1) as usize] = pixel[1];
                    input_block[(((y * 4 + x) * 4) + 2) as usize] = pixel[2];
                    input_block[(((y * 4 + x) * 4) + 3) as usize] = pixel[3];
                }
            }

            compress_block_bc7(options, &input_block, &mut output_block);
            output_blocks.extend_from_slice(&output_block);
        }
    }

    unsafe {
        sys::DestroyOptionsBC7(options);
    }

    Ok(output_blocks)
}

pub fn decompress_image_bc7(width: u32, height: u32, data: &[u8]) -> Result<RGBAImageData> {
    assert!(width % 4 == 0);
    assert!(height % 4 == 0);

    let width_in_blocks = width / 4;
    let height_in_blocks = height / 4;
    let num_blocks = width_in_blocks * height_in_blocks;

    let mut input_block = [0_u8; 64];
    let mut output_block = [0_u8; 16];

    let mut pixels = vec![0; (num_blocks * 64) as usize];

    for block_y in 0..height_in_blocks {
        for block_x in 0..width_in_blocks {
            let offset = ((block_y * width_in_blocks + block_x) * 16) as usize;
            output_block.copy_from_slice(&data[offset..(offset + 16)]);

            decompress_block_bc7(&output_block, &mut input_block);

            for y in 0..4 {
                for x in 0..4 {
                    let index = (((block_y * 4 + y) * width + (block_x * 4 + x)) * 4) as usize;
                    pixels[index] = input_block[((y * 4 + x) * 4) as usize];
                    pixels[index + 1] = input_block[((y * 4 + x) * 4 + 1) as usize];
                    pixels[index + 2] = input_block[((y * 4 + x) * 4 + 2) as usize];
                    pixels[index + 3] = input_block[((y * 4 + x) * 4 + 3) as usize];
                }
            }
        }
    }

    let image_data = RGBAImageData {
        width,
        height,
        pixels,
    };

    Ok(image_data)
}

pub fn compress_image_bc5(width: u32, height: u32, pixels: &[u8], quality: f32) -> Result<Vec<u8>> {
    assert!(width % 4 == 0);
    assert!(height % 4 == 0);

    let width_in_blocks = width / 4;
    let height_in_blocks = height / 4;
    let num_blocks = width_in_blocks * height_in_blocks;

    let mut input_block = [0_u8; 32];
    let mut output_block = [0_u8; 16];

    let mut options = core::ptr::null_mut();
    unsafe {
        sys::CreateOptionsBC5(&mut options);
        sys::SetQualityBC5(options, quality);
    }

    let mut output_blocks = Vec::with_capacity((num_blocks * 16) as usize);

    for block_y in 0..height_in_blocks {
        for block_x in 0..width_in_blocks {
            for y in 0..4 {
                for x in 0..4 {
                    let index =
                        (((block_y * 4 + y) * width + (block_x * 4 + x)) * 4) as usize;
                    let pixel = [pixels[index], pixels[index + 1]];
                    input_block[(y * 4 + x) as usize] = pixel[0];
                    input_block[(16 + (y * 4 + x)) as usize] = pixel[1];
                }
            }

            compress_block_bc5(options, &input_block, &mut output_block);
            output_blocks.extend_from_slice(&output_block);
        }
    }

    unsafe {
        sys::DestroyOptionsBC5(options);
    }

    Ok(output_blocks)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compress_bc7() {
        let input_block = [0_u8; 64];
        let mut output_block = [0_u8; 16];
        assert!(compress_block_bc7(
            core::ptr::null_mut(),
            &input_block,
            &mut output_block
        ));
    }

    #[test]
    fn decompress_bc7() {
        let input_block = [0_u8; 16];
        let mut output_block = [0_u8; 64];
        assert!(decompress_block_bc7(&input_block, &mut output_block));
    }

    #[test]
    fn compress_bc5() {
        let input_block = [0_u8; 32];
        let mut output_block = [0_u8; 16];
        assert!(compress_block_bc5(
            core::ptr::null_mut(),
            &input_block,
            &mut output_block
        ));
    }

    #[test]
    fn decompress_bc5() {
        let input_block = [0_u8; 16];
        let mut output_block = [0_u8; 32];
        assert!(decompress_block_bc5(&input_block, &mut output_block));
    }

    #[test]
    fn compress_bc4() {
        let input_block = [0_u8; 16];
        let mut output_block = [0_u8; 8];
        assert!(compress_block_bc4(&input_block, &mut output_block));
    }

    #[test]
    fn decompress_bc4() {
        let input_block = [0_u8; 8];
        let mut output_block = [0_u8; 16];
        assert!(decompress_block_bc4(&input_block, &mut output_block));
    }
}
