# CS50 OS
#### Video Demo: 
#### Description:
    ## CS50 OS is a simple, bare-metal compatable Operating System Kernel based entirely on Rust!
    ## (Made as a final project for the CS50x Edx course)
    Built on Rust, CS50 OS uses no standard libraries from rust_std, instead impletementing (most of) the most essential elements of an Operating System. Following the amazing tutorial by Philipp Oppermann ([Blog_os](https://os.phil-opp.com)), I wrote the entire Operating System Kernel with the help of his tutorial.

    CS50 OS Includes the following elements:
        - A bootable Binary disk image (You can even put it on a usb and boot it on bare-metal!)
        - VGA Text display for displaying text to the screen
        - Testing support using the rust testing framework
        - Panic handling
        - CPU Exceptions/Interrupts
        - Double/Page fault handling
        - Hardware Interrupts (Keyboard and timer)
        - Memory paging support
        - Heap allocation
        - Bump allocation
        - Linked list allocation
        - Fixed-size block allocation (All interchangeable)
        - Async/Await Multitasking support
        - Plus smaller things such as printing and serial printing support.
    All together making for quite a comprehensive set of features.

# File descriptions
## main.rs
> main.rs is what starts up everything else in the system, right after the hardware boot sequence finishes. For example it creates a panic handler for the case that something goes wrong anywhere (when panic! is called), It is also where the memory is initialized with allocator::init_heap, and where it is tested.
## lib.rs
> lib.rs is where everything is defined, like a .h header file for C++ users. More specifically, its where QemuExitCode is defined (So qemu can exit with the correct code, because qemu see any code that isnt 0, as a failure), and the test_runner is defined here, which is the function that automatically runs all the tests within the "tests" folder.
lib.rs also links to all the other modules that are a part of the kernel like the vga_buffer driver, interrupts handler, etc.
## memory.rs
> memory.rs is where all the system memory is managed. It initializes system memory within a level_4 cpu cache table by default, and is used for all memory operations.
## vga_buffer.rs
> vga_buffer.rs is where everything you see is rendered. It is the module that defines the "custom" print! and println! Rust macros, and enables the serial printing macros to work. The module uses a "Writer" class to directly write into the VGA buffer, writing the characters and color_codes of each one, making for very colorful text!
## serial.rs
> serial.rs is a small, but very useful module, implementing the serial_print! and serial_println! Rust macros, enabling easy and quick code testing, as it removes the need for QEMU opening, instead printing directly to the console.
        example: `should_panic::should_panic [ok]`
## interrupts.rs
> interrupts.rs is where hardware interrupts are all handled. CPU exceptions such as breakpoints, double_faults, timer_interrupts, keyboard_interrupts, and page_faults, are all handled and handed off to their respectable functions as needed. It is also where our custom IDT (InterruptDescriptorTable) is defined, for mapping the corresponding functions to each interrupt.
## allocator.rs
> allocator.rs is where our memory allocators are managed and their modules are defined. allocator.rs is like lib.rs, as it manages the multiple types of memory allocators CS50 OS, provides, each with their own pros and cons. It is also where the init_heap is defined, which is used in main.rs.
### Descriptions of the memory allocators
#### Bump:
    > The bump allocator is a fast, simple memory allocator, where memory is allocated by "bumping" a pointer forward. Its main disadvantage is it never frees memory until a reset or system restart.
#### Linked_list:
    > The linked list allocator is a less simple, but very effective way of managing memory in systems that need dynamic memory allocation and deallocation of memory blocks. it works by having a free list of available blocks and linking them together with pointers (Exactly like a Binary Tree). The main disadvantage of this allocator is that it may suffer from fragmentation and inefficiencies in certain cases.
#### Fixed_size_block:
    > The fixed size block allocator is a straightforward and efficient memory management technique that works well in systems where memory requests are consistent and are only of a fixed size (Like a modem). It works by dividing a large block of memory into equally-sized smaller blocks of memory. Each block is treated as an individual unit for allocation and deallocation. Its main disadvantage is its not a flexible allocator, as if the system needs varying blocks of memory, this allocator simply wont work.

## gdt.rs
> gdt.rs is the GlobalDescriptorTable for our Kernel. it is a custom implementation of a GDT, a data structure used in Kernels, for managing memory segments and for providing memory security and protection. Its critical in x86, as it helps the CPU understand how memory is organized and different regions of memory can be accessed.

# Folders
## cs50_os/src
> src contains all of the above files, plus the extra modules for allocator.rs, and task.
## cs50_os/src/allocator
> contains the above described memory allocators in separate files.
## cs50_os/src/task
> contains the Multitasking tools, such as for handling keyboard execution, and other task executions.
## cs50_os/tests
> contains various tests for testing heap_allocation, booting, panic handling, and stack_overflow handling.
## cs50_os/target/x86_64_cs50_os/debug
> contains the raw binary for booting the kernel (Called bootimage-cs50_os.bin)

# To build
> To run the kernel inside of QEMU (if installed), use `cargo run`
> To only build the boot image binary, use `cargo build`
> To perform all tests, use `cargo test`
> Or to perform a specific test, use `cargo test --test [test]`

# Personal note
> I really enjoyed following and making this Kernel, as it was a quite fun hardware programming experience. Shoutout to [Philipp Oppermann](https://github.com/phil-opp) for creating such an informative and easy to follow tutorial!
