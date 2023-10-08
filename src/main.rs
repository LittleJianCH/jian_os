#![no_std]
#![no_main]

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    println!("ERROR: {}", _info);
    loop {}
}

mod vga_buffer;

use vga_buffer::{Color, ColorCode, WRITER};

static HELLO: &str = "HELLO JIAN_OS\nThis is a newline";

#[no_mangle]
pub extern "C" fn _start() -> ! {
    set_color!(Pink, Black);

    print!("{}\n", HELLO);

    set_color!(White, Blue);

    println!("print some numbers: {} {}", 42, 1.337);

    loop {};
}