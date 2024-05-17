#![no_main]
#![no_std]

#[macro_use]
pub mod framebuffer;
pub mod gdt;
pub mod init;
pub mod utils;

use bootloader_api::{entry_point, BootInfo, BootloaderConfig};
use noto_sans_mono_bitmap::{FontWeight, RasterHeight};

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
    init(boot_info.framebuffer.take().unwrap());    
       
    
    // println!("{}", RasterHeight::Size20.val());

    for i in 0..100000 {
        println!("{}", i);

    }
    
    loop {}
}

entry_point!(kernel_main, config=&CONFIG);
