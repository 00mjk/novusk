#![no_std]

#[cfg(not(feature = "soc_selected"))]
compile_error!("SOC name not selected, add '--feature bcm(XXXX)' to DEVICE_DRIVER");

#[macro_use] extern crate novuskinc;

use core::panic::PanicInfo;

#[cfg(feature = "bcm2837")]
pub mod bcm2837;
