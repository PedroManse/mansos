#![no_std]
#![no_main]

use core::fmt::Write;
use mansos::*;
use volatile::Volatile;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    vga_print!("Hello");

    loop {}
}

fn wait() -> usize {
    let mut u = Volatile::new(0);
    for y in 0..1_000_000 {
        u.write(u.read() + y);
    }
    u.read()
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    if let Some(mut w) = vga::VGA_WRITER.try_lock() {
        let _ = write!(w, "{info:?}");
    }
    loop {}
}
