#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(cs50_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use cs50_os::{println, task::{Task, executor::Executor, keyboard}};

extern crate alloc;

use alloc::{boxed::Box, rc::Rc, vec, vec::Vec};

entry_point!(main);

fn main(boot_info: &'static BootInfo) -> ! {
    use cs50_os::{allocator, memory, memory::BootInfoFrameAllocator};
    use x86_64::VirtAddr;

    println!("Hello World{}", "!");
    println!("HII!!!!");

    cs50_os::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };

    allocator::init_heap(&mut mapper, &mut frame_allocator).expect("Heap init failed");

    let heap_value = Box::new(41);
    println!("heap value at {:p}", heap_value);

    let mut vec = Vec::new();
    for i in 0..500 {
        vec.push(i);
    }
    println!("vec at {:p}", vec.as_slice());

    let reference_counted = Rc::new(vec![1, 2, 3]);
    let cloned_reference = reference_counted.clone();
    println!(
        "current reference count is {}",
        Rc::strong_count(&cloned_reference)
    );
    core::mem::drop(reference_counted);
    println!(
        "reference count is {} now",
        Rc::strong_count(&cloned_reference)
    );

    let mut executor = Executor::new();
    executor.spawn(Task::new(example_task()));
    executor.spawn(Task::new(keyboard::print_keypresses()));
    executor.run();

    #[cfg(test)]
    test_main();

    // println!("It did not crash");
    // cs50_os::hlt_loop();
}

async fn async_num() -> u32 {
    42
}

async fn example_task() {
    let number = async_num().await;
    println!("async num: {}", number);
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
