#![no_std]
#![no_main]

#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]

mod vga;

use core::panic::PanicInfo;
use crate::vga::ColorCode;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[cfg(test)]
fn test_runner(tests: &[&dyn Fn()]) {
    println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    
    cprintln!(ColorCode::new(vga::Color::Magenta, vga::Color::Black), "Lunar OS v0.1");
    println!("Login");
    print!("\t>");
    
    loop {}
}
