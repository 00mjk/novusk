#![no_std]
#![feature(asm, global_asm, llvm_asm)]

#[path = "../aarch64.rs"]
pub mod arch;

pub use arch::*;
