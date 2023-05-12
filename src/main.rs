#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(bridi_os_2::test_runner)]
#![reexport_test_harness_main = "test_main"]

use bridi_os_2::println;
use core::panic::PanicInfo;
use bootloader::{ BootInfo, entry_point };

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use x86_64::{ VirtAddr, structures::paging::Page };
    use bridi_os_2::memory;
    use bridi_os_2::memory::BootInfoFrameAllocator;

    println!("Hello World{}", "!");
    bridi_os_2::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };

    let page = Page::containing_address(VirtAddr::new(0));
    memory::create_example_mapping(page, &mut mapper, &mut frame_allocator);

    let page_ptr: *mut u64 = page.start_address().as_mut_ptr();
    unsafe { page_ptr.offset(400).write_volatile(0x_f021_f077_f065_f04e) };

    #[cfg(test)]
    test_main();

    println!("It did not crash!");
    bridi_os_2::hlt_loop();
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    bridi_os_2::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    bridi_os_2::test_panic_handler(info)
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}
