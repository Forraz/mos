#![no_main]
#![no_std]

#[macro_use]
pub mod framebuffer;
pub mod gdt;
pub mod init;
pub mod utils;

use bootloader_api::{entry_point, BootInfo, BootloaderConfig};

use core::panic::PanicInfo;
use init::init;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {};
}
    
const CONFIG: BootloaderConfig = {
    let mut config = BootloaderConfig::new_default();
    config.kernel_stack_size = 1000 * 1024;
    config
};


fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    
}

entry_point!(kernel_main, config=&CONFIG);
