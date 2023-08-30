use std::collections::HashMap;

use crate::memory::registers::Registers;
use crate::memory::Memory;

#[derive(Debug)]
pub struct FakeMemory {
    memory: HashMap<u16, u8>,
    num_writes: u32,
}

impl FakeMemory {
    pub fn new() -> Self {
        Self {
            memory: HashMap::new(),
            num_writes: 0,
        }
    }

    pub fn assert_value(&self, address: u16, value: u8) {
        assert_eq!(
            *self.memory.get(&address).expect("value was not set"),
            value
        )
    }

    pub fn assert_never_updated(&self) {
        assert_eq!(self.num_writes, 0)
    }
}

impl Memory for FakeMemory {
    fn read(&self, address: u16) -> u8 {
        *self.memory.get(&address).unwrap()
    }

    fn read_signed(&self, address: u16) -> i8 {
        *self.memory.get(&address).unwrap() as i8
    }

    fn write(&mut self, address: u16, value: u8) {
        self.num_writes += 1;
        self.memory.insert(address, value);
    }
}

pub struct Register8BitsInfo {
    pub name: &'static str,
    pub index: u16,
    pub getter: fn(&Registers) -> u8,
    pub setter: fn(&mut Registers, u8),
}

impl PartialEq for Register8BitsInfo {
    fn eq(&self, other: &Self) -> bool {
        self.index == other.index
    }
}

pub const REGISTER_A: Register8BitsInfo = Register8BitsInfo {
    name: "Register A",
    index: 0b111,
    getter: |r| r.a,
    setter: |r, value| {
        r.a = value;
    },
};
pub const REGISTER_B: Register8BitsInfo = Register8BitsInfo {
    name: "Register B",
    index: 0b000,
    getter: |r| r.b,
    setter: |r, value| {
        r.b = value;
    },
};
pub const REGISTER_C: Register8BitsInfo = Register8BitsInfo {
    name: "Register C",
    index: 0b001,
    getter: |r| r.c,
    setter: |r, value| {
        r.c = value;
    },
};
pub const REGISTER_D: Register8BitsInfo = Register8BitsInfo {
    name: "Register D",
    index: 0b010,
    getter: |r| r.d,
    setter: |r, value| {
        r.d = value;
    },
};
pub const REGISTER_E: Register8BitsInfo = Register8BitsInfo {
    name: "Register E",
    index: 0b011,
    getter: |r| r.e,
    setter: |r, value| {
        r.e = value;
    },
};
pub const REGISTER_H: Register8BitsInfo = Register8BitsInfo {
    name: "Register H",
    index: 0b100,
    getter: |r| r.h,
    setter: |r, value| {
        r.h = value;
    },
};
pub const REGISTER_L: Register8BitsInfo = Register8BitsInfo {
    name: "Register L",
    index: 0b101,
    getter: |r| r.l,
    setter: |r, value| {
        r.l = value;
    },
};

pub const REGISTERS_8_BITS: &[Register8BitsInfo] = &[
    REGISTER_A, REGISTER_B, REGISTER_C, REGISTER_D, REGISTER_E, REGISTER_H, REGISTER_L,
];
pub const REGISTERS_8_BITS_WITHOUT_A: &[Register8BitsInfo] = &[
    REGISTER_B, REGISTER_C, REGISTER_D, REGISTER_E, REGISTER_H, REGISTER_L,
];

pub struct Register16BitsInfo {
    pub name: &'static str,
    pub index: u16,
    pub getter: fn(&Registers) -> u16,
    pub setter: fn(&mut Registers, u16),
}

impl PartialEq for Register16BitsInfo {
    fn eq(&self, other: &Self) -> bool {
        self.index == other.index
    }
}

pub const REGISTER_BC: Register16BitsInfo = Register16BitsInfo {
    name: "Register BC",
    index: 0,
    getter: |r| r.get_bc(),
    setter: |r, value| {
        r.set_bc(value);
    },
};

pub const REGISTER_DE: Register16BitsInfo = Register16BitsInfo {
    name: "Register DE",
    index: 0b01,
    getter: |r| r.get_de(),
    setter: |r, value| {
        r.set_de(value);
    },
};

pub const REGISTER_HL: Register16BitsInfo = Register16BitsInfo {
    name: "Register HL",
    index: 0b10,
    getter: |r| r.get_hl(),
    setter: |r, value| {
        r.set_hl(value);
    },
};

pub const REGISTER_SP: Register16BitsInfo = Register16BitsInfo {
    name: "Register SP",
    index: 0b11,
    getter: |r| r.sp,
    setter: |r, value| {
        r.sp = value;
    },
};

pub const REGISTER_AF: Register16BitsInfo = Register16BitsInfo {
    name: "Register AF",
    index: 0b11,
    getter: |r| r.get_af(),
    setter: |r, value| {
        r.set_af(value);
    },
};

pub const REGISTERS_16_BITS: &[Register16BitsInfo] =
    &[REGISTER_BC, REGISTER_DE, REGISTER_HL, REGISTER_SP];
pub const REGISTERS_16_BITS_WITHOUT_HL: &[Register16BitsInfo] =
    &[REGISTER_BC, REGISTER_DE, REGISTER_SP];

pub const PUSH_POP_REGISTERS_16_BITS: &[Register16BitsInfo] =
    &[REGISTER_BC, REGISTER_DE, REGISTER_HL, REGISTER_AF];

/// Flags defined page 108: https://ia803208.us.archive.org/9/items/GameBoyProgManVer1.1/GameBoyProgManVer1.1.pdf
pub struct FlagInfo {
    pub name: &'static str,
    pub index: u16,
    pub getter: fn(&Registers) -> bool,
    pub setter: fn(&mut Registers, bool),
}

impl PartialEq for FlagInfo {
    fn eq(&self, other: &Self) -> bool {
        self.index == other.index
    }
}

pub const FLAG_NON_ZERO: FlagInfo = FlagInfo {
    name: "Flag non zero",
    index: 0b00,
    getter: |r| r.get_non_zero_flag(),
    setter: |r, value| {
        r.set_non_zero_flag(value);
    },
};

pub const FLAG_ZERO: FlagInfo = FlagInfo {
    name: "Flag zero",
    index: 0b01,
    getter: |r| r.get_zero_flag(),
    setter: |r, value| {
        r.set_zero_flag(value);
    },
};

pub const FLAG_NON_CARRY: FlagInfo = FlagInfo {
    name: "Flag non carry",
    index: 0b10,
    getter: |r| r.get_non_carry_flag(),
    setter: |r, value| {
        r.set_non_carry_flag(value);
    },
};

pub const FLAG_CARRY: FlagInfo = FlagInfo {
    name: "Flag carry",
    index: 0b11,
    getter: |r| r.get_carry_flag(),
    setter: |r, value| {
        r.set_carry_flag(value);
    },
};

pub const FLAG_INFOS: &[FlagInfo] = &[FLAG_NON_ZERO, FLAG_ZERO, FLAG_NON_CARRY, FLAG_CARRY];
