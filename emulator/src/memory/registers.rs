pub const OFFSET_CARRY_FLAG: u8 = 0x4;
pub const OFFSET_HALF_CARRY_FLAG: u8 = 0x5;
pub const OFFSET_ADD_SUB_FLAG: u8 = 0x6;
pub const OFFSET_ZERO_FLAG: u8 = 0x7;

pub const CARRY_FLAG_MASK: u8 = 0b1 << OFFSET_CARRY_FLAG;
pub const HALF_CARRY_FLAG_MASK: u8 = 0b1 << OFFSET_HALF_CARRY_FLAG;
pub const ADD_SUB_FLAG_MASK: u8 = 0b1 << OFFSET_ADD_SUB_FLAG;
pub const ZERO_FLAG_MASK: u8 = 0b1 << OFFSET_ZERO_FLAG;

pub const CARRY_FLAG_INV_MASK: u8 = 0xFF - CARRY_FLAG_MASK;
pub const HALF_CARRY_FLAG_INV_MASK: u8 = 0xFF - HALF_CARRY_FLAG_MASK;
pub const ADD_SUB_FLAG_INV_MASK: u8 = 0xFF - ADD_SUB_FLAG_MASK;
pub const ZERO_FLAG_INV_MASK: u8 = 0xFF - ZERO_FLAG_MASK;

#[derive(Debug, PartialEq, Clone)]
pub struct Registers {
    pub flags: u8,
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
    pub fn new() -> Self {
        Self {
            flags: 0,
            a: 0,
            c: 0,
            b: 0,
            e: 0,
            d: 0,
            l: 0,
            h: 0,
            sp: 0,
            pc: 0,
            halted: false,
            stopped: false,
            ime_flag: false,
        }
    }

    pub fn get_carry_flag(&self) -> bool {
        ((self.flags >> OFFSET_CARRY_FLAG) & 0x1) == 0x1
    }
    pub fn get_half_carry_flag(&self) -> bool {
        ((self.flags >> OFFSET_HALF_CARRY_FLAG) & 0x1) == 0x1
    }
    pub fn get_add_sub_flag(&self) -> bool {
        ((self.flags >> OFFSET_ADD_SUB_FLAG) & 0x1) == 0x1
    }
    pub fn get_zero_flag(&self) -> bool {
        ((self.flags >> OFFSET_ZERO_FLAG) & 0x1) == 0x1
    }
    pub fn get_non_carry_flag(&self) -> bool {
        !self.get_carry_flag()
    }
    pub fn get_non_zero_flag(&self) -> bool {
        !self.get_zero_flag()
    }

    pub fn set_carry_flag(&mut self, value: bool) {
        if value {
            self.flags |= CARRY_FLAG_MASK;
        } else {
            self.flags &= CARRY_FLAG_INV_MASK;
        }
    }
    pub fn set_half_carry_flag(&mut self, value: bool) {
        if value {
            self.flags |= HALF_CARRY_FLAG_MASK;
        } else {
            self.flags &= HALF_CARRY_FLAG_INV_MASK;
        }
    }
    pub fn set_add_sub_flag(&mut self, value: bool) {
        if value {
            self.flags |= ADD_SUB_FLAG_MASK;
        } else {
            self.flags &= ADD_SUB_FLAG_INV_MASK;
        }
    }
    pub fn set_zero_flag(&mut self, value: bool) {
        if value {
            self.flags |= ZERO_FLAG_MASK;
        } else {
            self.flags &= ZERO_FLAG_INV_MASK;
        }
    }

    pub fn set_non_carry_flag(&mut self, value: bool) {
        self.set_carry_flag(!value)
    }
    pub fn set_non_zero_flag(&mut self, value: bool) {
        self.set_zero_flag(!value)
    }

    pub fn get_af(&self) -> u16 {
        Registers::get_composite_register(self.a, self.flags)
    }

    pub fn set_af(&mut self, value: u16) {
        Registers::set_composite_register(&mut self.a, &mut self.flags, value)
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
