use crate::bitarray::{BitArray32, BitArray64};

#[repr(C)]
pub struct Descriptor {
    pub value: u64
}


pub struct DescriptorManager {
    pub index: u32,
    descriptor: &'static mut Descriptor,
    bitarray: BitArray64
    

}

impl DescriptorManager {

    pub fn new(gdtr: u64, index: u32) -> Self {
        let gdtr_base = gdtr as u32;
        Self {
            bitarray: BitArray64::from_u64(0),
            index,
            descriptor: unsafe { &mut *((gdtr_base + (index * 8)) as *mut Descriptor)}
        }
    }

    pub fn set_value(&mut self) {
        self.descriptor.value = self.bitarray.into_u64();
    }

    fn set_by_index(&mut self, index: u8, value: bool) {
        self.bitarray.bits[index as usize] = value.into(); 
        self.set_value();
    }

    pub fn get_limit(&self) -> u32 {
        let bits = &self.bitarray.bits;
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
        let bits = &mut self.bitarray.bits;
        let limit_bitarray = BitArray32::from_u32(value);

        for i in 0..16 {
            bits[63-i] = limit_bitarray.bits[31-i]
        }

        for i in 16..20 {
            bits[31-i] = limit_bitarray.bits[31-i]

        }

        self.set_value();
    }

    pub fn get_base(&self) -> u32 {
       let bits = &self.bitarray.bits;
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
        let bits = &mut self.bitarray.bits;
        let limit_bitarray = BitArray32::from_u32(value);

        for i in 0..24 {
            bits[47-i] = limit_bitarray.bits[31-i]
        }

        for i in 24..32 {
            bits[31-i] = limit_bitarray.bits[31-i]

        }

        self.set_value();
    }

    pub fn get_g(&self) -> bool  {
        self.bitarray.bits[8] != 0
    }

    pub fn set_g(&mut self, value: bool) {
        self.set_by_index(8, value)
    }

    pub fn get_db(&self) -> bool {
        self.bitarray.bits[9] != 0
    }

    pub fn set_db(&mut self, value: bool) {
        self.set_by_index(9, value)
    }

    pub fn get_l(&self) -> bool {
        self.bitarray.bits[10] != 0
    }

    pub fn set_l(&mut self, value: bool) {
        self.set_by_index(10, value)
    } 
    
    pub fn get_avl(&self) -> bool {
        self.bitarray.bits[11] != 0
    }

    pub fn set_avl(&mut self, value: bool) {
        self.set_by_index(11, value);
    }

    pub fn get_p(&self) -> bool {
        self.bitarray.bits[15] != 0
    }

    pub fn set_p(&mut self, value: bool) {
        self.set_by_index(15, value)
    }

    pub fn get_dpl(&self) -> u8 {
        self.bitarray.bits[16] + self.bitarray.bits[17] * 2 
    }

    pub fn set_dpl(&mut self, value: u8) {
        let bits = &mut self.bitarray.bits[0..2];
        let val_array = BitArray32::from_u32(value.into());
        for i in 0..2 {
            bits[i] = val_array.bits[i]; 
        }

        self.set_value();
    }

    pub fn get_s(&self) -> bool {
        self.bitarray.bits[18] != 0
    }
    
    pub fn set_s(&mut self, value: bool) {
        self.set_by_index(18, value)
    }

    pub fn get_is_code(&self) -> bool { // executable bit
        self.bitarray.bits[19] != 0
    }

    pub fn set_is_code(&mut self, value: bool) {
        self.set_by_index(19, value);
    }

    pub fn get_e(&self) -> bool {
        self.bitarray.bits[20] != 0 
    }

    pub fn set_e(&mut self, value: bool) {
        self.set_by_index(20, value)
    }

    pub fn get_w(&self) -> bool {
        self.bitarray.bits[21] != 0
    }

    pub fn set_w(&mut self, value: bool) {
        self.set_by_index(21, value);
    }

    pub fn get_a(&self) -> bool {
        self.bitarray.bits[22] != 0
    }

    pub fn set_a(&mut self, value: bool) {
        self.set_by_index(22, value)
    }


}

pub struct GDTManager {
    gdtr: u64
    
}


impl GDTManager {
    
    pub fn init() -> GDTManager {
        let gdtr: u64 = 0;  
        let gdt_manager = GDTManager { gdtr };
        gdt_manager.gdt_init();
        gdt_manager

    }

    pub fn gdt_init(&self) {
        self.set_null_entry();
    }

    fn set_null_entry(&self) {
        // let entry_manager = DescriptorManager::new(0);
        
    }

}


