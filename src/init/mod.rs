
use crate::gdt::set_gdtr;

pub fn init() {
    set_gdtr();
}
