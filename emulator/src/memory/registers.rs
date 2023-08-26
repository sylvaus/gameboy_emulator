const OFFSET_CARRY_FLAG: u8 = 0x4;
const OFFSET_HALF_CARRY_FLAG: u8 = 0x5;
const OFFSET_ADD_SUB_FLAG: u8 = 0x6;
const OFFSET_ZERO_FLAG: u8 = 0x7;

#[derive(Debug)]
pub struct Registers {
    pub f: u8,
    pub a: u8,
    pub c: u8,
    pub b: u8,
    pub e: u8,
    pub d: u8,
    pub l: u8,
    pub h: u8,
    pub sp: u16,
    pub pc: u16,

    pub halted: bool,
    pub stopped: bool,
    pub ime_flag: bool, // Interrupt Master Enable Flag
}

impl Registers {
    pub fn get_carry_flag(&self) -> bool {
        ((self.f >> OFFSET_CARRY_FLAG) & 0x1) == 0x1
    }
    pub fn get_half_carry_flag(&self) -> bool {
        ((self.f >> OFFSET_HALF_CARRY_FLAG) & 0x1) == 0x1
    }
    pub fn get_add_sub_flag(&self) -> bool {
        ((self.f >> OFFSET_ADD_SUB_FLAG) & 0x1) == 0x1
    }
    pub fn get_zero_flag(&self) -> bool {
        ((self.f >> OFFSET_ZERO_FLAG) & 0x1) == 0x1
    }

    pub fn get_af(&self) -> u16 {
        Registers::get_composite_register(self.a, self.f)
    }

    pub fn set_af(&mut self, value: u16) {
        Registers::set_composite_register(&mut self.a, &mut self.f, value)
    }

    pub fn get_bc(&self) -> u16 {
        Registers::get_composite_register(self.b, self.c)
    }

    pub fn set_bc(&mut self, value: u16) {
        Registers::set_composite_register(&mut self.b, &mut self.c, value)
    }

    pub fn get_de(&self) -> u16 {
        Registers::get_composite_register(self.d, self.e)
    }

    pub fn set_de(&mut self, value: u16) {
        Registers::set_composite_register(&mut self.d, &mut self.e, value)
    }

    pub fn get_hl(&self) -> u16 {
        Registers::get_composite_register(self.h, self.l)
    }

    pub fn set_hl(&mut self, value: u16) {
        Registers::set_composite_register(&mut self.h, &mut self.l, value)
    }

    fn get_composite_register(high: u8, low: u8) -> u16 {
        ((high as u16) << 8) + (low) as u16
    }

    fn set_composite_register(high: &mut u8, low: &mut u8, value: u16) {
        *low = (value & 0xFF) as u8;
        *high = (value >> 8) as u8;
    }
}
