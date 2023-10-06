#![no_std]
#![no_main]

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

mod vga_buffer;

use core::fmt::Write;
use vga_buffer::{Color, ColorCode, Writer};

static HELLO: &str = "HELLO JIAN_OS\nThis is a newline";

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let mut writer = Writer::new();

    writer.set_color(ColorCode::new(Color::Pink, Color::Black));
    
    writer.write_string(HELLO);

    writer.set_color(ColorCode::new(Color::White, Color::Blue));

    write!(writer, "\nprint some numbers: {} {}", 42, 1.337).unwrap();

    loop {};
}