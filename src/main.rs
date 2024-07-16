// Project wide features
#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(sunscreen_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

use bootloader::{entry_point, BootInfo};
use sunscreen_os::memory::{self, BootInfoFrameAllocator};
use x86_64::VirtAddr;

entry_point!(kernel_main);

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

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    println!("Sunscreen OS 0.1.0");
    sunscreen_os::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut _mapper = unsafe { memory::init(phys_mem_offset) };
    let mut _frame_allocator = unsafe {
        BootInfoFrameAllocator::init(&boot_info.memory_map)
    };


    #[cfg(test)]
    test_main();

    println!("It didnt crash!");
    sunscreen_os::hlt_loop();
}