#![no_std]
#![no_main]

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

static HELLO: &[u8] = b"HELLO JIAN_OS";

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let vga = 0xb8000 as *mut u8;

    for (i, &byte) in (0..).zip(HELLO.iter()) {
        unsafe {
            *vga.offset(i * 2) = byte;
            *vga.offset(i * 2 + 1) = [0xe, 0xa, 0xb, 0xd][i as usize % 4];
        }
    }

    loop {};
}