

pub struct BitArray32 {
    pub bits: [u8; 32]
}


impl BitArray32 {

    pub fn from_u32(i: u32) -> Self {
        let mut bits = [0 as u8; 32];
        
        let i = i.reverse_bits();

        for offset in 0..i32::BITS {
            let bit = ((i >> offset) & 1) as u8;
            bits[offset as usize] = bit;
               
        }

        Self { bits }
    }

    pub fn into_u32(&self) -> u32 {
        let mut res: u32 = 0;
        let base: u32 = 2;
        for i in 0..32 {
            res = res + ((self.bits[31-(i as usize)] as u32) * (base.pow(i)));

        }
        res
    }
}

