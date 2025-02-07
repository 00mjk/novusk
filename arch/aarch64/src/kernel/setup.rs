use core::ptr::write_volatile;
use printk::printk_init;
use crate::early_printk;
use super::cpu::irq::aarch64_irq_setup;
use super::early_printk::aarch64_setup_early_printk;
use setup::{ArchKernelSetup, SetupReturn};
use crate::include::dif::DIF;

struct Aarch64Kernel;

impl Aarch64Kernel {
    pub fn new() -> Self {
        return Aarch64Kernel;
    }
    
    pub fn setup(&self) {
        self.test_memory();
        let irq = self.irq_setup();
        let dev = self.device_init();

        if irq.0.is_err() {
            panic!("{}", irq.1);
        } else if dev.0.is_err() {
            panic!("{}", dev.1);
        }

        early_printk!("{}\n", irq.1);
        early_printk!("{}\n", dev.1);
    }

    fn test_memory(&self) {
        let mut test_vec = vec![0];
        test_vec.push(1);

        for i in 0..1024 {
            test_vec.push(i);
        }
    }
}

impl ArchKernelSetup for Aarch64Kernel {
    fn irq_setup(&self) -> SetupReturn {
        unsafe { aarch64_irq_setup(); }
        return (Ok(()), "IRQ setup successfully");
    }
}

#[no_mangle]
pub unsafe extern "C" fn setup_arch() {
    let aarch64_setup = Aarch64Kernel::new();

    aarch64_setup_early_printk();

    early_printk!("\nStarting Aarch64 kernel...\n");
    early_printk!("Early and main kernel printing for Aarch64 initialized\n");
    early_printk!("\nSetting up kernel...\n");

    aarch64_setup.setup();
}
