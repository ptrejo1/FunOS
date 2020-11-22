#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(fun_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use fun_os::println;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    fun_os::init();

    #[cfg(test)]
    test_main();

    println!("It did not crash!");
    fun_os::hlt_loop();
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    fun_os::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    fun_os::test_panic_handler(info)
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}
