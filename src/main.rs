// disable standard library
#![no_std]
// disable all Rust-level entry points
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(blog_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use blog_os::println;
use bootloader::{BootInfo, entry_point};
use alloc::{boxed::Box, vec, vec::Vec, rc::Rc};

extern crate alloc;

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
//    use x86_64::{structures::paging::Translate, VirtAddr};
    use x86_64::{structures::paging::Page, VirtAddr};
    use blog_os::allocator;
    use blog_os::memory::{self, BootInfoFrameAllocator};

    println!("Hello World{}", "!");
    blog_os::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
//    let mut frame_allocator = memory::EmptyFrameAllocator;
    let mut frame_allocator = unsafe {
        BootInfoFrameAllocator::init(&boot_info.memory_map)
    };

    // map an unused page
    //let page = Page::containing_address(VirtAddr::new(0));
    let page = Page::containing_address(VirtAddr::new(0xdeadbeaf000));
    memory::create_example_mapping(page, &mut mapper, &mut frame_allocator);

    // write the string `New!` to the screen through the new mapping
    let page_ptr: *mut u64 = page.start_address().as_mut_ptr();
    unsafe { page_ptr.offset(400).write_volatile(0x_f021_f077_f065_f04e)};

    allocator::init_heap(&mut mapper, &mut frame_allocator)
        .expect("heap initialization failed");

    let heap_value = Box::new(41);
    println!("heap_value at {:p}", heap_value);

    let mut vec = Vec::new();
    for i in 0..500 {
        vec.push(i);
    }
    println!("vec at {:p}", vec.as_slice());
    
    // create a reference counted vector -> will be free when count reaches 0
    let reference_counted = Rc::new(vec![1, 2, 3]);
    let cloned_reference = reference_counted.clone();
    println!("current reference count is {}", Rc::strong_count(&cloned_reference));
    core::mem::drop(reference_counted);
    println!("reference count is {} now", Rc::strong_count(&cloned_reference));

//    let addresses = [
//        // the identity-mapped vga buffer page
//        0xb8000,
//        // some code page
//        0x201008,
//        // some stack page
//        0x0100_0020_1a10,
//        // virtual address mapped to physical address 0
//        boot_info.physical_memory_offset,
//    ];
//
//    for &address in &addresses {
//        let virt = VirtAddr::new(address);
//        //let phys = unsafe { translate_addr(virt, phys_mem_offset) };
//        let phys = mapper.translate_addr(virt);
//        println!("{:?} -> {:?}", virt, phys);
//    }

//    let l4_table = unsafe { active_level_4_table(phys_mem_offset) };
//
//    for (i, entry) in l4_table.iter().enumerate() {
//        use x86_64::structures::paging::PageTable;
//
//        if !entry.is_unused() {
//            println!("L4 Entry {}: {:?}", i, entry);
//
//            // get the physical address from the entry and convert it
//            let phys = entry.frame().unwrap().start_address();
//            let virt = phys.as_u64() + boot_info.physical_memory_offset;
//            let ptr = VirtAddr::new(virt).as_mut_ptr();
//            let l3_table: &PageTable = unsafe { &*ptr };
//
//            // print non-empty entries of the level 3 table
//            for (i, entry) in l3_table.iter().enumerate() {
//                if !entry.is_unused() {
//                    println!("  L3 Entry {}: {:?}", i, entry);
//                }
//            }
//        }
//    }
//    use x86_64::registers::control::Cr3;
//
//    let (level_4_page_table, _) = Cr3::read();
//    println!("Level 4 page table at: {:?}", level_4_page_table.start_address());
//
//    let ptr = 0x204a80 as *mut u32;
//    //let ptr = 0xdeadbeaf as *mut u32;
//    unsafe { let x = *ptr; }
//    println!("read worked");
//
//    unsafe { *ptr = 42; }
//    println!("write worked");
//
////    fn stack_overflow() {
////        stack_overflow();
////    }
//
//    //stack_overflow();
//
////    // trigger a page fault
////    unsafe {
////        *(0xdeadbeaf as *mut u64) = 42;
////    };
////
////    x86_64::instructions::interrupts::int3();

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
