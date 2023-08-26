use std::collections::HashMap;

use crate::memory::registers::Registers;
use crate::memory::Memory;

#[derive(Debug)]
pub struct FakeMemory {
    memory: HashMap<u16, u8>,
}

impl FakeMemory {
    pub fn new() -> Self {
        Self {
            memory: HashMap::new(),
        }
    }

    pub fn assert_value(&self, address: u16, value: u8) {
        assert_eq!(
            *self.memory.get(&address).expect("value was not set"),
            value
        )
    }
}

impl Memory for FakeMemory {
    fn get(&self, address: u16) -> u8 {
        *self.memory.get(&address).unwrap()
    }

    fn get_signed(&self, address: u16) -> i8 {
        *self.memory.get(&address).unwrap() as i8
    }

    fn set(&mut self, address: u16, value: u8) {
        self.memory.insert(address, value);
    }
}

pub struct Register8BitsInfo {
    pub name: &'static str,
    pub index: u8,
    pub getter: fn(&Registers) -> u8,
    pub setter: fn(&mut Registers, u8),
}

impl PartialEq for Register8BitsInfo {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
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
    pub index: u8,
    pub getter: fn(&Registers) -> u16,
    pub setter: fn(&mut Registers, u16),
}

impl PartialEq for Register16BitsInfo {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
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

pub const REGISTERS_16_BITS: &[Register16BitsInfo] = &[
    REGISTER_BC, REGISTER_DE, REGISTER_HL, REGISTER_SP,
];
pub const REGISTERS_16_BITS_WITHOUT_HL: &[Register16BitsInfo] = &[
    REGISTER_BC, REGISTER_DE, REGISTER_SP,
];
