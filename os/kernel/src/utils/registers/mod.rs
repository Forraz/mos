use core::arch::asm; 
use crate::utils::bitarray::BitArray32;


 
pub struct CR0 {
    bits: [u8; 32]

}

impl CR0 {

    unsafe fn read_cr0() -> u32 {
        let mut cr0: u32;
        asm!("mov rax, cr0", out("rax") cr0);
        cr0
    }

    unsafe fn write_cr0(value: u32) {
        asm!("mov cr0, rax", in("rax") value)
    }

    fn to_bits() -> BitArray32 {
        BitArray32::from_u32(unsafe { Self::read_cr0() })
    }

    fn write_from_bits(bits: BitArray32) {
        unsafe { Self::write_cr0(bits.into_u32()) }
    }

    fn get_by_index(index: u8) -> bool {
        Self::to_bits().bits[index as usize] != 0
    }

    fn set_by_index(index: u8, value: bool)  {
        let bit = match value {
            true => 1,
            false => 0
        };

        let mut bitarray = Self::to_bits();
        bitarray.bits[index as usize] = bit;

        Self::write_from_bits(bitarray)
    }

    pub fn get_pe() -> bool {
        Self::get_by_index(31)
    }

    pub fn set_pe(value: bool) {
        Self::set_by_index(31, value)
    }

    pub fn get_mp() -> bool {
        Self::get_by_index(31-1)
    }

    pub fn set_mp(value: bool) {
        Self::set_by_index(31-1, value)
    }

    pub fn get_em() -> bool {
        Self::get_by_index(31-2)
    }

    pub fn set_em(value: bool) {
        Self::set_by_index(31-2, value)
    }

    pub fn get_ts() -> bool {
        Self::get_by_index(31-3)
    }

    pub fn set_ts(value: bool) {
        Self::set_by_index(31-3, value)
    }

    pub fn get_et() -> bool {
        Self::get_by_index(31-4)
    }

    pub fn set_et(value: bool) {
        Self::set_by_index(31-4, value)
    }

    pub fn get_ne() -> bool {
        Self::get_by_index(31-5)
    }

    pub fn set_ne(value: bool) {
        Self::set_by_index(31-5, value)
    }

    pub fn get_wp() -> bool {
        Self::get_by_index(31-16)
    }

    pub fn set_wp(value: bool) {
        Self::set_by_index(31-16, value)
    }

    pub fn get_am() -> bool {
        Self::get_by_index(31-18)
    }

    pub fn set_am(value: bool) {
        Self::set_by_index(31-18, value)
    }

    pub fn get_nw() -> bool {
        Self::get_by_index(31-29)
    }

    pub fn set_nw(value: bool) {
        Self::set_by_index(31-29, value)
    }

    pub fn get_cd() -> bool {
        Self::get_by_index(31-30)
    }

    pub fn set_cd(value: bool) {
        Self::set_by_index(31-30, value)
    }

    pub fn get_pg() -> bool {
        Self::get_by_index(0)
    }

    pub fn set_pg(value: bool) {
        Self::set_by_index(0, value)
    }



}

