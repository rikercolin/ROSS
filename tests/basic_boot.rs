#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(ross::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![test_runner(ross::test_runner)]

use core::panic::PanicInfo;
use ross::println;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    #[cfg(test)]
    test_main();

    loop {}
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    ross::test_panic_handler(info)
}

#[test_case]
fn test_println() {
    println!("test_println output");
}