#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(cs50_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use cs50_os::println;

entry_point!(main);

fn main(_boot_info: &'static BootInfo) -> ! {
    println!("Hello World{}", "!");
    println!("HII!!!!");

    cs50_os::init();

    #[cfg(test)]
    test_main();

    println!("It did not crash");
    cs50_os::hlt_loop();
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    cs50_os::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    cs50_os::test_panic_handler(info)
}

#[test_case]
fn test_println_basic() {
    println!("test_println_basic output");
}

#[test_case]
fn test_println_many_lines() {
    for _ in 0..=200 {
        println!("test_println_many_lines output");
    }
}
