use core::cell::LazyCell;

use spin::Mutex;
use uart_16550::SerialPort;

pub static SERIAL_CONN: Mutex<LazyCell<SerialPort>> = Mutex::new(unsafe {
    LazyCell::new(|| {
        let mut sp = SerialPort::new(0x3F8);
        sp.init();
        sp
    })
});

#[doc(hidden)]
pub fn print(args: ::core::fmt::Arguments) {
    use core::fmt::Write;
    SERIAL_CONN
        .lock()
        .write_fmt(args)
        .expect("Printing to serial failed");
}

#[macro_export]
macro_rules! serial_print {
    ($($arg:tt)*) => ($crate::serial::print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! serial_println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::serial_print!("{}\n", format_args!($($arg)*)));
}
