#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(mansos::tests::test_runner)]

use core::panic::PanicInfo;

use mansos::{exit_qemu, serial, serial_println, vga, vga_println};

#[unsafe(no_mangle)] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    serial_println!("Testing serial Connection");
    vga_println!("Testing VGA Connection");

    exit_qemu(mansos::QemuExitCode::Success);
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    use core::fmt::Write;
    if let Some(mut w) = vga::VGA_WRITER.try_lock() {
        let _ = write!(w, "{info:?}");
    }
    if let Some(mut w) = serial::SERIAL_CONN.try_lock() {
        let _ = write!(w, "{info:?}");
    }
    exit_qemu(mansos::QemuExitCode::Failure)
}
