use compressonator_sys as sys;

pub fn compress_block_bc7(input_block: &[u8], output_block: &mut [u8]) -> bool {
    unsafe {
        sys::CompressBlockBC7(
            input_block.as_ptr(),
            16,
            output_block.as_mut_ptr(),
            core::ptr::null(),
        ) == 0
    }
}

pub fn decompress_block_bc7(input_block: &[u8], output_block: &mut [u8]) -> bool {
    unsafe {
        sys::DecompressBlockBC7(
            input_block.as_ptr(),
            output_block.as_mut_ptr(),
            core::ptr::null(),
        ) == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compress_bc7() {
        let input_block = [0_u8; 64];
        let mut output_block = [0_u8; 16];
        assert!(compress_block_bc7(&input_block, &mut output_block));
    }

    #[test]
    fn decompress_bc7() {
        let input_block = [0_u8; 16];
        let mut output_block = [0_u8; 64];
        assert!(decompress_block_bc7(&input_block, &mut output_block));
    }
}
