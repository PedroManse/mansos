#![no_std]
#![no_main]

use core::fmt::Write;
#[cfg(test)]
use mansos::exit_qemu;
use mansos::{serial, serial_println, vga};

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    #[cfg(test)]
    exit_qemu(mansos::QemuExitCode::Success);
    #[cfg(not(test))]
    main()
}

fn main() -> ! {
    serial_println!("Hello");
    #[allow(clippy::empty_loop)]
    loop {}
}

#[panic_handler]
pub fn panic(info: &core::panic::PanicInfo) -> ! {
    if let Some(mut w) = vga::VGA_WRITER.try_lock() {
        let _ = write!(w, "{info:?}");
    }
    if let Some(mut w) = serial::SERIAL_CONN.try_lock() {
        let _ = write!(w, "{info:?}");
    }
    loop {}
}
