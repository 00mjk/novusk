use core::arch::asm;
use core::panic::PanicInfo;
use crate::early_printk;

#[panic_handler]
unsafe fn _panic(_info: &PanicInfo) -> ! {
    early_printk!("\n\nx86_64 kernel panic\n");
    early_printk!("    Message: {}\n", _info.message().unwrap());
    early_printk!("    Location: {}\n", _info.location().unwrap());
    early_printk!("    Timer value: {}\n", crate::kernel::time::TIMER_VALUE);

    loop { asm!("hlt"); }
}
