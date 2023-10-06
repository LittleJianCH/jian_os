#![no_std]
#![no_main]

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

mod vga_buffer;

use core::fmt::Write;
use vga_buffer::{Color, ColorCode, WRITER};

static HELLO: &str = "HELLO JIAN_OS\nThis is a newline";

#[no_mangle]
pub extern "C" fn _start() -> ! {
    WRITER.lock().set_color(ColorCode::new(Color::Pink, Color::Black));
    
    WRITER.lock().write_string(HELLO);

    WRITER.lock().set_color(ColorCode::new(Color::White, Color::Blue));

    write!(WRITER.lock(), "\nprint some numbers: {} {}", 42, 1.337).unwrap();

    loop {};
}