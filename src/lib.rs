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

// FIXME: This is almost definitely broken
pub fn compress_block_bc5(input_block: &[u8], output_block: &mut [u8]) -> bool {
    unsafe {
        sys::CompressBlockBC5(
            input_block.as_ptr(),
            8,
            input_block.as_ptr().add(4),
            8,
            output_block.as_mut_ptr(),
            core::ptr::null(),
        ) == 0
    }
}

pub fn compress_block_bc4(input_block: &[u8], output_block: &mut [u8]) -> bool {
    unsafe {
        sys::CompressBlockBC4(
            input_block.as_ptr(),
            4,
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

// FIXME: This is almost definitely broken
pub fn decompress_block_bc5(input_block: &[u8], output_block: &mut [u8]) -> bool {
    unsafe {
        sys::DecompressBlockBC5(
            input_block.as_ptr(),
            output_block.as_mut_ptr(),
            output_block.as_mut_ptr().add(4),
            core::ptr::null(),
        ) == 0
    }
}

pub fn decompress_block_bc4(input_block: &[u8], output_block: &mut [u8]) -> bool {
    unsafe {
        sys::DecompressBlockBC4(
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

    #[test]
    fn compress_bc5() {
        let input_block = [0_u8; 64];
        let mut output_block = [0_u8; 16];
        assert!(compress_block_bc5(&input_block, &mut output_block));
    }

    #[test]
    fn decompress_bc5() {
        let input_block = [0_u8; 16];
        let mut output_block = [0_u8; 64];
        assert!(decompress_block_bc5(&input_block, &mut output_block));
    }

    #[test]
    fn compress_bc4() {
        let input_block = [0_u8; 64];
        let mut output_block = [0_u8; 16];
        assert!(compress_block_bc4(&input_block, &mut output_block));
    }

    #[test]
    fn decompress_bc4() {
        let input_block = [0_u8; 16];
        let mut output_block = [0_u8; 64];
        assert!(decompress_block_bc4(&input_block, &mut output_block));
    }
}
