

pub struct BitArray32 {
    pub bits: [u8; 32]
}


impl BitArray32 {

    pub fn from_u32(i: u32) -> Self {
        let mut bits = [0 as u8; 32];
        
        let i = i.reverse_bits();

        for offset in 0..u32::BITS {
            let bit = ((i >> offset) & 1) as u8;
            bits[offset as usize] = bit;
               
        }

        Self { bits }
    }

    pub fn into_u32(&self) -> u32 {
        let mut res: u32 = 0;
        let base: u32 = 2;
        for i in 0..u32::BITS {
            res = res + ((self.bits[31-(i as usize)] as u32) * (base.pow(i)));

        }
        res
    }

    // pub fn extend(&self, )
}


pub struct BitArray64 {
    pub bits: [u8; 64]
}


impl BitArray64 {

    pub fn from_u64(i: u64) -> Self {
        let mut bits = [0 as u8; 64];
        
        let i = i.reverse_bits();

        for offset in 0..64 {
            let bit = ((i >> offset) & 1) as u8;
            bits[offset as usize] = bit;
               
        }

        Self { bits }
    }

    pub fn into_u64(&self) -> u64 {
        let mut res: u64 = 0;
        let base: u64 = 2;
        for i in 0..64 {
            res = res + ((self.bits[63-(i as usize)] as u64) * (base.pow(i)));

        }
        res
    }
}
