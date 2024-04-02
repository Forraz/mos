#![no_main]
#![no_std]

mod vga;

use core::panic::PanicInfo;



#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {};
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("i");
    println!("123");
    loop {}
}
    



