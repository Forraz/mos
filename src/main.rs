#![no_main]
#![no_std]

#[macro_use]
mod vga;
mod bitarray;
mod registers;
mod gdt;
mod init;

use core::panic::PanicInfo;
use init::init;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {};
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    init();
    loop {}
    
}
    



