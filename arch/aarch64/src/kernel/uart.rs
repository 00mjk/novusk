use core::fmt::Write;

//gen_simpleuart!();

pub unsafe fn is_init() -> bool {
    /*if KERNEL_SIMPLEUART.output_addr == 0x0 as *mut u8 && KERNEL_SIMPLEUART.input_addr == 0x0 as *mut u8 {
        return false;
    } else {
        return true;
    }
    */
    true
}
