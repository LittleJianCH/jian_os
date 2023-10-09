#![no_std]
#![no_main]

#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

mod vga_buffer;
mod io_exit;
mod serial;


#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    #[cfg(test)]
    {
        serial_println!("ERROR: {}", _info);
        exit!(Failed);        
    }

    #[cfg(not(test))]
    {
        println!("ERROR: {}", _info);
    }

    loop {}
}

#[cfg(test)]
fn test_runner(tests: &[&dyn Fn()]) {
    serial_println!();
    serial_println!("Running {} tests...", tests.len());

    for test in tests {
        test();
    }

    exit!(Success);
}

static HELLO: &str = "HELLO JIAN_OS";

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("{}", HELLO);

    #[cfg(test)]
    test_main();

    loop {};
}

#[test_case]
fn assert_2_times_3_eq_6() {
    serial_print!("assert 2 * 3 = 6... ");
    assert_eq!(2 * 3, 6);
    serial_println!("[ok]");
}