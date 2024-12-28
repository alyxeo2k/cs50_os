#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(cs50_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use cs50_os::println;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");
    println!("HII!!!!");
    
    #[cfg(test)]
    test_main();

    loop {}
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    cs50_os::test_panic_handler(info)
}

#[test_case]
fn trivial_assertion1() {
    assert_eq!(1,1);
}
#[test_case]
fn trivial_assertion2() {
    assert_eq!(1,1);
}
#[test_case]
fn trivial_assertion3() {
    assert_eq!(1,1);
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