#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(bridi_os_2::test_runner)]
#![reexport_test_harness_main = "test_main"]

use bridi_os_2::println;
use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    use x86_64::registers::control::Cr3;
    println!("Hello World{}", "!");

    bridi_os_2::init();

    let ptr = 0xdeadbeaf as *mut u32;
    unsafe { *ptr = 42; }

    let (level_4_page_table, _) = Cr3::read();
    println!(
        "Level 4 page table at: {:?}",
        level_4_page_table.start_address()
    );

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
