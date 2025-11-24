#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(tests::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::mem::transmute;
use volatile::Volatile;

pub mod serial;
pub mod tests;
pub mod vga;
pub use tests::test_runner;

#[allow(clippy::must_use_candidate)]
pub fn wait() -> usize {
    let mut u = Volatile::new(0);
    for y in 0..1_000_000 {
        u.write(u.read() + y);
    }
    u.read()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failure = 0x11,
}

pub fn exit_qemu(exit_code: QemuExitCode) -> ! {
    use x86_64::instructions::port::Port;

    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    };
    #[allow(invalid_value)]
    unsafe {
        transmute(())
    }
}
