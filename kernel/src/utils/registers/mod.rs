use core::arch::asm; 
 
pub struct CR0 {}

impl CR0 {

    unsafe fn read_cr0() -> u32 {
        let mut cr0: u32;
        asm!("mov rax, cr0", out("rax") cr0);
        cr0
    }

    unsafe fn write_cr0(value: u32) {
        asm!("mov cr0, rax", in("rax") value)
    }

    fn get_value() -> u32 {
        unsafe { Self::read_cr0() }
    }
    
    fn set_value(value: u32) {
        unsafe { Self::write_cr0(value) }
    }
 
    pub fn get_bit(position: usize) -> bool {
        ((Self::get_value() >> position) & 1) != 0
    }

    pub fn set_bit(position: usize, value: bool) {
        let mask = (1u32) << position;
        let new_value: u32;

        if value {
            new_value = Self::get_value() | mask;
        } else {
            new_value = Self::get_value() & !mask;
        }

        Self::set_value(new_value);
    } 

    pub fn get_pe() -> bool {
        Self::get_bit(0)
    }

    pub fn set_pe(value: bool) {
        Self::set_bit(0, value)
    }

    pub fn get_mp() -> bool {
        Self::get_bit(1)
    }

    pub fn set_mp(value: bool) {
        Self::set_bit(1, value)
    }

    pub fn get_em() -> bool {
        Self::get_bit(2)
    }

    pub fn set_em(value: bool) {
        Self::set_bit(2, value)
    }

    pub fn get_ts() -> bool {
        Self::get_bit(3)
    }

    pub fn set_ts(value: bool) {
        Self::set_bit(3, value)
    }

    pub fn get_et() -> bool {
        Self::get_bit(4)
    }

    pub fn set_et(value: bool) {
        Self::set_bit(4, value)
    }

    pub fn get_ne() -> bool {
        Self::get_bit(5)
    }

    pub fn set_ne(value: bool) {
        Self::set_bit(5, value)
    }

    pub fn get_wp() -> bool {
        Self::get_bit(16)
    }

    pub fn set_wp(value: bool) {
        Self::set_bit(16, value)
    }

    pub fn get_am() -> bool {
        Self::get_bit(18)
    }

    pub fn set_am(value: bool) {
        Self::set_bit(18, value)
    }

    pub fn get_nw() -> bool {
        Self::get_bit(29)
    }

    pub fn set_nw(value: bool) {
        Self::set_bit(29, value)
    }

    pub fn get_cd() -> bool {
        Self::get_bit(30)
    }

    pub fn set_cd(value: bool) {
        Self::set_bit(30, value)
    }

    pub fn get_pg() -> bool {
        Self::get_bit(31)
    }

    pub fn set_pg(value: bool) {
        Self::set_bit(31, value)
    }


}

