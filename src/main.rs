#![no_std]
#![no_main]

mod vga;

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World!");
    println!("{} + {} = {}", 2, 2, 2 + 2);

    for i in 0..100 {
        println!("Row {}", i);
    }

    print!("Hey!");

    loop {}
}
