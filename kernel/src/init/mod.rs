
use bootloader_api::info::FrameBuffer;

use crate::gdt::set_gdtr;
use crate::framebuffer::{set_writer, WRITER};


pub fn init(framebuffer: FrameBuffer) {
    set_gdtr();
    set_writer(framebuffer);
    unsafe { WRITER.get_mut() }.as_mut().unwrap().clear();
}
