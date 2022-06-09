use compressonator_sys as sys;

pub fn test_func() -> i32 {
    let input_block = [0_u8; 64];
    let input_stride = 16;
    let mut output_block = [0_u8; 64];
    let options = core::ptr::null();
    let result = unsafe { sys::CompressBlockBC7(input_block.as_ptr(), input_stride, output_block.as_mut_ptr(), options) };
    dbg!(input_block);
    dbg!(output_block);
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = test_func();
        assert_eq!(result, 0);
    }
}
