#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(opp_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use opp_os::println;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World!");

    opp_os::init();

    #[allow(unconditional_recursion)]
    fn stack_overflow() {
        stack_overflow();
    }

    // stack_overflow();

    #[cfg(test)]
    test_main();

    println!("It did not crash!");
    opp_os::hlt_loop();
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    opp_os::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    opp_os::test_panic_handler(info)
}
