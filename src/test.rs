use crate::*;

#[cfg(test)]
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    test_main();
    exit_qemu(QemuExitCode::Success)
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    use core::fmt::Write;
    if let Some(mut w) = vga::VGA_WRITER.try_lock() {
        let _ = write!(w, "{info:?}");
    }
    loop {}
}

#[cfg(test)]
pub fn test_runner(tests: &[&dyn Fn()]) {
    vga_println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
}

#[test_case]
fn trivial_assertion() {
    vga_print!("trivial assertion... ");
    assert_eq!(1, 1);
    vga_println!("[ok]");
}
