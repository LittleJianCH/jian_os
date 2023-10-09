use uart_16550::SerialPort;
use spin::Mutex;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref COM1: Mutex<SerialPort> = Mutex::new({
        let mut port = unsafe { SerialPort::new(0x3F8) };
        port.init();
        port
    });
}

#[doc(hidden)]
pub fn _print(args: ::core::fmt::Arguments) {
    use core::fmt::Write;
    COM1.lock().write_fmt(args).expect("Printing to COM1 failed");
}

#[macro_export]
macro_rules! serial_print {
    ($($arg:tt)*) => {
        $crate::serial::_print(format_args!($($arg)*));
    };
}

#[macro_export]
macro_rules! serial_println {
    () => ($crate::serial_print!("\n"));
    ($($arg:tt)*) => {
        serial_print!($($arg)*);
        serial_println!();
    }
}