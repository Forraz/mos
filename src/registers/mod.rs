use core::arch::asm; 
use crate::bitarray::BitArray32;

 
pub struct CR0 {
    pub pe: bool,
    pub mp: bool,
    pub em: bool,
    pub ts: bool,
    pub et: bool,
    pub ne: bool,
    pub wp: bool,
    pub am: bool,
    pub nw: bool,
    pub cd: bool,
    pub pg: bool
}

impl CR0 {

    pub unsafe fn read_cr0() -> u32 {
        let mut cr0: u32;
        asm!("mov rax, cr0", out("rax") cr0);
        cr0
    }

    unsafe fn write_cr0(value: u32) {
        asm!("mov cr0, rax", in("rax") value)
    }

    pub fn init() -> Self {
        let cr0 = unsafe { CR0::read_cr0() };
        let bitarr = BitArray32::from_u32(cr0);
        CR0 {
            pe: bitarr.bits[31-0] != 0,
            mp: bitarr.bits[31-1] != 0,
            em: bitarr.bits[31-2] != 0,
            ts: bitarr.bits[31-3] != 0,
            et: bitarr.bits[31-4] != 0,
            ne: bitarr.bits[31-5] != 0,
            wp: bitarr.bits[31-16] != 0,
            am: bitarr.bits[31-18] != 0,
            nw: bitarr.bits[31-29] != 0,
            cd: bitarr.bits[31-30] != 0,
            pg: bitarr.bits[31-31] != 0,
        }
    }

    pub fn commit(&self) {
       let mut bitarr = BitArray32::from_u32(0);
       if self.pe { bitarr.bits[31-0] = 1}
       if self.mp { bitarr.bits[31-1] = 1}
       if self.em { bitarr.bits[31-2] = 1}
       if self.ts { bitarr.bits[31-3] = 1}
       if self.et { bitarr.bits[31-4] = 1}
       if self.ne { bitarr.bits[31-5] = 1}
       if self.wp { bitarr.bits[31-16] = 1}
       if self.am { bitarr.bits[31-18] = 1}
       if self.nw { bitarr.bits[31-29] = 1}
       if self.cd { bitarr.bits[31-30] = 1}
       if self.pg { bitarr.bits[31-31] = 1}

       let cr0 = bitarr.into_u32();
       unsafe { Self::write_cr0(cr0); } 


    }


}

