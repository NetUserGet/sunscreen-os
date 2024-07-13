#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(sunscreen_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

use sunscreen_os::Testable;
use sunscreen_os::println;

#[panic_handler]
fn panic_handler(info: &PanicInfo) -> ! {
    sunscreen_os::test_panic_handler(info)
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    test_main();

    loop {}
}

pub fn test_runner(tests: &[&dyn Testable]) {
    sunscreen_os::test_runner(tests);
}

#[test_case]
fn test_println() {
    println!("test_println output");
}
