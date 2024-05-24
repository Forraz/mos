
use bootloader_api::info::FrameBuffer;

use crate::gdt::set_gdt;
use crate::framebuffer::{set_writer, WRITER};


pub fn init(framebuffer: FrameBuffer) {
    set_gdt();
    framebuffer_init(framebuffer);
}

fn framebuffer_init(framebuffer: FrameBuffer) {
    set_writer(framebuffer);
    unsafe { WRITER.get_mut() }.as_mut().unwrap().clear();

}
