#![no_std]
#![no_main]

use core::fmt::Write;
#[cfg(test)]
use mansos::exit_qemu;
use mansos::{vga, vga_print};

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    #[cfg(test)]
    exit_qemu(mansos::QemuExitCode::Success);
    #[cfg(not(test))]
    main()
}

fn main() -> ! {
    let mut x = 0;
    loop {
        x+=1;
        vga_print!("#{x}");
    }
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    if let Some(mut w) = vga::VGA_WRITER.try_lock() {
        let _ = write!(w, "{info:?}");
    }
    loop {}
}
