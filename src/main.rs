#![no_main]
#![no_std]


#[macro_use]
mod vga;
mod bitarray;
mod registers;
mod gdt;

use core::{panic::PanicInfo};



#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {};
}

#[no_mangle]
pub extern "C" fn _start() -> ! {

    loop {}
    
}
    



