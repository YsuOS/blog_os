// disable standard library
#![no_std]
// disable all Rust-level entry points
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(blog_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use blog_os::println;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    blog_os::init();

    use x86_64::registers::control::Cr3;

    let (level_4_page_table, _) = Cr3::read();
    println!("Level 4 page table at: {:?}", level_4_page_table.start_address());

    let ptr = 0x204a80 as *mut u32;
    //let ptr = 0xdeadbeaf as *mut u32;
    unsafe { let x = *ptr; }
    println!("read worked");

    unsafe { *ptr = 42; }
    println!("write worked");

//    fn stack_overflow() {
//        stack_overflow();
//    }

    //stack_overflow();

//    // trigger a page fault
//    unsafe {
//        *(0xdeadbeaf as *mut u64) = 42;
//    };
//
//    x86_64::instructions::interrupts::int3();

    #[cfg(test)]
    test_main();

    println!("It did not crash!");
//    loop {
//        use blog_os::print;
//        print!("-");
//    }
    blog_os::hlt_loop();
}

/// This function is called on panic.
#[cfg(not(test))]  
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    blog_os::hlt_loop();
}

#[cfg(test)]  
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    blog_os::test_panic_handler(info)
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}
