#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(cs50_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use cs50_os::println;

#[no_mangle]
pub extern "C" fn _start() -> ! {
	test_main();

	loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
	cs50_os::test_panic_handler(info)
}

#[test_case]
fn test_println() {
	println!("test_println output");
}

#[test_case]
#[allow(unconditional_panic)]
fn test_divide_by_zero() {
	assert_eq!(1/0, 0);
}