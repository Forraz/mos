use core::{arch::asm, hint::black_box, ptr::addr_of};

const GDT_SIZE: usize = 6;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Descriptor {
    pub value: u64
}

pub struct GDTManager {
    pub gdt: [Descriptor; GDT_SIZE],
    current_index: usize
}

impl GDTManager {
    
    pub fn init(&mut self) {
        self.gdt_init();

    }

    pub fn gdt_init(&mut self) {
        self.set_kmcs_descriptor();
        self.set_kmds_descriptor();
        self.set_umcs_descriptor();
        self.set_umds_descriptor();
        self.set_tss_descriptor();
    }

    fn set_kmcs_descriptor(&mut self) { // Kernel Mode Code Segment setter (Flat mode)
        self.set_current_index(1);
        self.set_km_descriptor();
        self.set_is_code(true);

    }

    fn set_kmds_descriptor(&mut self) { // Kernel Mode Data Segment setter (Flat mode)
        self.set_current_index(2);
        self.set_km_descriptor();
        self.set_is_code(false);
    }
 
    fn set_km_descriptor(&mut self) {
        self.set_default_descriptor();
        self.set_dpl(0);
    }

    fn set_umcs_descriptor(&mut self) { // User Mode Code Segment setter (Flat mode)
        self.set_current_index(3);
        self.set_um_descriptor();
        self.set_is_code(true);
    }

    fn set_umds_descriptor(&mut self) { // User Mode Data Segment setter (Flat mode)
        self.set_current_index(4);
        self.set_um_descriptor();
        self.set_is_code(false);
    }

    fn set_um_descriptor(&mut self) {
        self.set_default_descriptor();
        self.set_dpl(3);
    }

    fn set_tss_descriptor(&mut self) { // Task State Segment setter (Flat mode)
        self.set_current_index(5)
    }

    fn set_default_descriptor(&mut self) {
        self.set_base(0);
        self.set_limit(0xFFFFF);
        self.set_g(true);
        self.set_db(true);
        self.set_l(false);
        self.set_avl(false);
        self.set_p(true);
        self.set_s(true);
        self.set_ec(false);
        self.set_wr(true);
        self.set_a(false);
    }

    pub fn set_current_index(&mut self, index: usize) {
        self.current_index = index;
    }

    pub fn get_descriptor(&mut self) -> &mut Descriptor {
        &mut self.gdt[self.current_index]
    }

    pub fn get_value(&mut self) -> u64 {
        self.get_descriptor().value
    }

    pub fn set_value(&mut self, value: u64) {
        let descriptor = self.get_descriptor(); 
        descriptor.value = value; 
    }

    pub fn get_bit(&mut self, position: usize) -> bool {
        ((self.get_value() >> position) & 1) != 0
    }

    pub fn set_bit(&mut self, position: usize, value: bool) {
        let mask: u64 = (1 as u64) << position;
        let new_value: u64;

        if value {
            new_value = self.get_value() | mask;
        } else {
            new_value = self.get_value() & !mask;
        }

        self.set_value(new_value);
    }

    pub fn get_limit(&mut self) -> u32 {
        let value = self.get_value();
        let first_part = (value & ((2 as u64).pow(16)-1)) as u32;
        let second_part = ((value & (15 << 32+16)) >> 32) as u32;

        first_part | second_part
    }

    pub fn set_limit(&mut self, value: u32) {
        let mut descriptor_value = self.get_value();
        let first_part = (value & ((2 as u32).pow(16)-1)) as u64; 
        let second_part = (((value as u64) >> 15) & 15) << 48; 
        
        descriptor_value |= first_part;
        descriptor_value |= second_part;

        self.set_value(descriptor_value);    
    }

    pub fn get_base(&mut self) -> u32 {
        let value = self.get_value();
        let first_part = ((value >> 16) & ((2 as u64).pow(24)-1)) as u32;
        let second_part = (((value >> 56) & ((2 as u64).pow(8)-1)) << 24) as u32;
        
        first_part | second_part
    }   

    pub fn set_base(&mut self, value: u32) {
        let mut descriptor_value = self.get_value();
        let first_part = ((value as u64) & (2 as u64).pow(24)-1) << 16;
        let second_part = (((value as u64) >> 24) & (u32::max_value() as u64)) << 56;

        descriptor_value |= first_part;
        descriptor_value |= second_part;

        self.set_value(descriptor_value);  
    }

    pub fn get_g(&mut self) -> bool  {
        self.get_bit(32+23)
    }
    
    pub fn set_g(&mut self, value: bool) {
        self.set_bit(32+23, value)
    }
    
    pub fn get_db(&mut self) -> bool {
        self.get_bit(32+22)
    }
    
    pub fn set_db(&mut self, value: bool) {
        self.set_bit(32+22, value)
    }
    
    pub fn get_l(&mut self) -> bool {
        self.get_bit(32+21)
    }
    
    pub fn set_l(&mut self, value: bool) {
        self.set_bit(32+21, value)
    } 
     
    pub fn get_avl(&mut self) -> bool {
        self.get_bit(32+20)
    }
    
    pub fn set_avl(&mut self, value: bool) {
        self.set_bit(32+20, value);
    }
    
    pub fn get_p(&mut self) -> bool {
        self.get_bit(32+15)
    }
    
    pub fn set_p(&mut self, value: bool) {
        self.set_bit(32+15, value)
    }

    pub fn get_dpl(&mut self) -> u8 {
        (self.get_bit(45) as u8) + (self.get_bit(46) as u8) * 2 
    }

    pub fn set_dpl(&mut self, value: u8) {
        self.set_bit(45, (value & 1) == 1);
        self.set_bit(46, ((value >> 1) & 1) == 1);
    }
    
    pub fn get_s(&mut self) -> bool {
        self.get_bit(32+12)
    }
    
    pub fn set_s(&mut self, value: bool) {
        self.set_bit(32+12, value)
    }
    
    pub fn get_is_code(&mut self) -> bool { // executable bit
        self.get_bit(32+11)
    }
    
    pub fn set_is_code(&mut self, value: bool) {
        self.set_bit(32+11, value);
    }
    
    pub fn get_ec(&mut self) -> bool { // expand-down/conforming bit
        self.get_bit(32+10) 
    }
    
    pub fn set_ec(&mut self, value: bool) {
        self.set_bit(32+10, value)
    }
    
    pub fn get_wr(&mut self) -> bool { // writable/readable bit
        self.get_bit(32+9)
    }
    
    pub fn set_wr(&mut self, value: bool) { 
        self.set_bit(32+9, value);
    }
    
    pub fn get_a(&mut self) -> bool {
        self.get_bit(32+8)
    }
    
    pub fn set_a(&mut self, value: bool) {
        self.set_bit(32+8, value)
    }    
}

pub fn set_gdtr() {
    pub static mut GDT: GDTManager = GDTManager { gdt: [Descriptor {value: 0}; 6], current_index: 0 };
    unsafe { GDT.init() };

    let gdtr_base = black_box(unsafe { addr_of!(GDT) } as u32);
    let gdtr_limit = black_box(GDT_SIZE as u16);
    
    unsafe { 
        asm!("mov [rsp], rax", in("rax") gdtr_limit);
        asm!("mov [rsp+2], rax", in("rax") gdtr_base);
        asm!("lgdt [rsp]");
    }
    
}
