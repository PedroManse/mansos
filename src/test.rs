use crate::*;

pub trait Test {
    fn run(&self) -> ();
}

impl<F> Test for F
where
    F: Fn() -> (),
{
    fn run(&self) {
        serial_print!("Test [{}]: ", core::any::type_name::<Self>());
        self();
        serial_println!("Ok");
    }
}

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
    if let Some(mut w) = serial::SERIAL_CONN.try_lock() {
        let _ = write!(w, "{info:?}");
    }
    exit_qemu(QemuExitCode::Failure)
}

#[cfg(test)]
pub fn test_runner(tests: &[&dyn Test]) {
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test.run()
    }
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(0, 1);
}
