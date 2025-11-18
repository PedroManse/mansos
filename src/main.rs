#![no_std]
#![no_main]

use core::fmt::Write;
use core::panic::PanicInfo;
use mansos::*;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    write!(vga::VGA_WRITER.lock(), "Hello").unwrap();
    //vga::VGA_WRITER.lock().write_str("hello");

    loop {}
}

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
