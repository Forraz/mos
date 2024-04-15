use crate::bitarray::{BitArray32, BitArray64};



#[repr(C)]
pub struct Descriptor {
    pub value: u64
}

impl Descriptor {
    
    pub fn get_limit(&self) -> u32 {
        let mut bitarray = BitArray64::from_u64(self.value);
        let bits = &mut bitarray.bits;
        let mut limit_bitarray = BitArray32::from_u32(0);

        let first_part = &bits[63-15..];
        let second_part = &bits[63-51..63-47];

        for i in 0..16 {
            limit_bitarray.bits[31-i] = first_part[15-i];
        }

        for i in 16..20 {
            limit_bitarray.bits[31-i] = second_part[3-(i-16)];
        }

        limit_bitarray.into_u32()
    }

    pub fn set_limit(&mut self, value: u32) {
        let mut bitarray = BitArray64::from_u64(self.value);
        let bits = &mut bitarray.bits;
        let limit_bitarray = BitArray32::from_u32(value);

        for i in 0..16 {
            bits[63-i] = limit_bitarray.bits[31-i]
        }

        for i in 16..20 {
            bits[31-i] = limit_bitarray.bits[31-i]

        }

        self.value = bitarray.into_u64();
    }

    pub fn get_base(&self) -> u32 {
       let mut bitarray = BitArray64::from_u64(self.value);
       let bits = &mut bitarray.bits;
       let mut base_bitarray = BitArray32::from_u32(0);
        
       let first_part = &bits[63-31..63-15];
       let second_part = &bits[63-39..63-31];
       let third_part = &bits[0..63-55];

        for i in 0..16 {
            base_bitarray.bits[31-i] = first_part[15-i];
        }

        for i in 16..24 {
            base_bitarray.bits[31-i] = second_part[7-(i-16)];
        }

        for i in 24..32 {
            base_bitarray.bits[31-i] = third_part[7-(i-24)]
        }

        base_bitarray.into_u32() 
    }

    pub fn set_base(&mut self, value: u32) {
        let mut bitarray = BitArray64::from_u64(self.value);
        let bits = &mut bitarray.bits;
        let limit_bitarray = BitArray32::from_u32(value);

        for i in 0..24 {
            bits[47-i] = limit_bitarray.bits[31-i]
        }

        for i in 24..32 {
            bits[31-i] = limit_bitarray.bits[31-i]

        }

        self.value = bitarray.into_u64();
    }
}


