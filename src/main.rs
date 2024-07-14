// Project wide features
#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(sunscreen_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

mod serial;
mod vga_buffer;

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    sunscreen_os::test_panic_handler(info);
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    sunscreen_os::hlt_loop();
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello OS dev, {}", "from rust!");
    sunscreen_os::init();

    #[cfg(test)]
    test_main();

    println!("It didnt crash!");
    sunscreen_os::hlt_loop();
}