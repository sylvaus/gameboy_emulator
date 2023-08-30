/// Tests for the ADD instruction
/// The ADD instruction is described in chapter 4: page 102and 107 of https://ia803208.us.archive.org/9/items/GameBoyProgManVer1.1/GameBoyProgManVer1.1.pdf
use crate::generated::instructions::tests::fixture::{
    FakeMemory, REGISTERS_16_BITS, REGISTERS_8_BITS, REGISTER_A, REGISTER_HL,
};
use crate::generated::instructions::{get_instruction, ImmediateArgumentType};
use crate::memory::argument::Argument;
use crate::memory::registers::Registers;
use crate::memory::Memory;

const ADD_8_BITS_BASE_OPCODE: u16 = 0b10000000;
const ADD_16_BITS_BASE_OPCODE: u16 = 0b1001;

#[test]
fn test_add_16bits() {
    for register_info in REGISTERS_16_BITS {
        let opcode = (register_info.index << 4) + ADD_16_BITS_BASE_OPCODE;
        let mut register = Registers::new();
        let mut memory = FakeMemory::new();
        let argument = Argument::new_empty();
        register.set_hl(0x456);
        // In the case of register A it just double the value.
        if register_info != &REGISTER_HL {
            (register_info.setter)(&mut register, 0x123);
        }
        let mut expected = register.clone();

        let (instruction, argument_type) = get_instruction(opcode);
        let nb_cycle = instruction(&mut register, &mut memory, &argument);

        assert_eq!(nb_cycle, 8);

        if register_info == &REGISTER_HL {
            expected.set_hl(0x456 * 2);
        } else {
            expected.set_hl(0x456 + 0x123);
        }
        expected.pc = 1;
        assert_eq!(register, expected);
        assert_eq!(argument_type, ImmediateArgumentType::None);
    }
}

/// Half carry happens if there is a carry from bit 11
#[test]
fn test_add_16bits_half_carry() {
    for register_info in REGISTERS_16_BITS {
        let opcode = (register_info.index << 4) + ADD_16_BITS_BASE_OPCODE;
        let mut register = Registers::new();
        let mut memory = FakeMemory::new();
        let argument = Argument::new_empty();
        register.set_hl(0x7A23);
        // In the case of register A it just double the value.
        if register_info != &REGISTER_HL {
            (register_info.setter)(&mut register, 0x605);
        }
        let mut expected = register.clone();

        let (instruction, argument_type) = get_instruction(opcode);
        let nb_cycle = instruction(&mut register, &mut memory, &argument);

        assert_eq!(nb_cycle, 8);

        expected.set_half_carry_flag(true);
        if register_info == &REGISTER_HL {
            expected.set_hl(0x7A23 * 2);
        } else {
            expected.set_hl(0x7A23 + 0x605);
        }
        expected.pc = 1;
        assert_eq!(register, expected);
        assert_eq!(argument_type, ImmediateArgumentType::None);
    }
}

/// Carry happens if there is a carry from bit 15
#[test]
fn test_add_16bits_carry() {
    for register_info in REGISTERS_16_BITS {
        let opcode = (register_info.index << 4) + ADD_16_BITS_BASE_OPCODE;
        let mut register = Registers::new();
        let mut memory = FakeMemory::new();
        let argument = Argument::new_empty();
        register.set_hl(0x8023);
        // In the case of register A it just double the value.
        if register_info != &REGISTER_HL {
            (register_info.setter)(&mut register, 0x9446);
        }
        let mut expected = register.clone();

        let (instruction, argument_type) = get_instruction(opcode);
        let nb_cycle = instruction(&mut register, &mut memory, &argument);

        assert_eq!(nb_cycle, 8);

        expected.set_carry_flag(true);
        if register_info == &REGISTER_HL {
            expected.set_hl(0x8023u16.wrapping_mul(2));
        } else {
            expected.set_hl(0x8023u16.wrapping_add(0x9446));
        }
        expected.pc = 1;
        assert_eq!(register, expected);
        assert_eq!(argument_type, ImmediateArgumentType::None);
    }
}

#[test]
fn test_add_8bits() {
    for register_info in REGISTERS_8_BITS {
        let opcode = register_info.index + ADD_8_BITS_BASE_OPCODE;
        let mut register = Registers::new();
        let mut memory = FakeMemory::new();
        let argument = Argument::new_empty();
        register.a = 0x45u8;
        // In the case of register A it just double the value.
        if register_info != &REGISTER_A {
            (register_info.setter)(&mut register, 0xA2u8);
        }
        let mut expected = register.clone();

        let (instruction, argument_type) = get_instruction(opcode);
        let nb_cycle = instruction(&mut register, &mut memory, &argument);

        assert_eq!(nb_cycle, 4);

        if register_info == &REGISTER_A {
            expected.a = 0x8Au8;
        } else {
            expected.a = 0x45u8 + 0xA2u8;
        }
        expected.pc = 1;
        assert_eq!(register, expected);
        assert_eq!(argument_type, ImmediateArgumentType::None);
    }
}

#[test]
fn test_add_8bits_half_carry() {
    for register_info in REGISTERS_8_BITS {
        let opcode = register_info.index + ADD_8_BITS_BASE_OPCODE;
        let mut registers = Registers::new();
        let mut memory = FakeMemory::new();
        let argument = Argument::new_empty();
        registers.a = 0x08u8;
        // In the case of register A it just double the value.
        if register_info != &REGISTER_A {
            (register_info.setter)(&mut registers, 0x09u8);
        }
        let mut expected = registers.clone();

        let (instruction, argument_type) = get_instruction(opcode);
        let nb_cycle = instruction(&mut registers, &mut memory, &argument);

        assert_eq!(nb_cycle, 4);

        expected.set_half_carry_flag(true);
        if register_info == &REGISTER_A {
            expected.a = 0x10u8;
        } else {
            expected.a = 0x08u8 + 0x09u8;
        }
        expected.pc = 1;
        assert_eq!(registers, expected);
        assert_eq!(argument_type, ImmediateArgumentType::None);
    }
}

#[test]
fn test_add_8bits_carry() {
    for register_info in REGISTERS_8_BITS {
        let opcode = register_info.index + ADD_8_BITS_BASE_OPCODE;
        let mut registers = Registers::new();
        let mut memory = FakeMemory::new();
        let argument = Argument::new_empty();
        registers.a = 0x81u8;
        // In the case of register A it just double the value.
        if register_info != &REGISTER_A {
            (register_info.setter)(&mut registers, 0x90u8);
        }
        let mut expected = registers.clone();

        let (instruction, argument_type) = get_instruction(opcode);
        let nb_cycle = instruction(&mut registers, &mut memory, &argument);

        assert_eq!(nb_cycle, 4);

        expected.set_carry_flag(true);
        if register_info == &REGISTER_A {
            expected.a = 0x02u8;
        } else {
            expected.a = 0x11u8;
        }
        expected.pc = 1;
        assert_eq!(registers, expected);
        assert_eq!(argument_type, ImmediateArgumentType::None);
    }
}

#[test]
fn test_add_8bits_zero() {
    for register_info in REGISTERS_8_BITS {
        let opcode = register_info.index + ADD_8_BITS_BASE_OPCODE;
        let mut registers = Registers::new();
        let mut memory = FakeMemory::new();
        let argument = Argument::new_empty();
        // In the case of register A it just double the value.
        registers.a = 0x80u8;
        if register_info != &REGISTER_A {
            (register_info.setter)(&mut registers, 0x80u8);
        }
        let mut expected = registers.clone();

        let (instruction, argument_type) = get_instruction(opcode);
        let nb_cycle = instruction(&mut registers, &mut memory, &argument);

        assert_eq!(nb_cycle, 4);
        expected.set_zero_flag(true);
        expected.set_carry_flag(true);
        expected.a = 0;
        expected.pc = 1;
        assert_eq!(registers, expected);
        assert_eq!(argument_type, ImmediateArgumentType::None);
    }
}

#[test]
fn test_add_8bits_address() {
    let opcode = 0x86;
    let mut register = Registers::new();
    let mut memory = FakeMemory::new();
    let argument = Argument::new_empty();
    register.a = 0x45;
    memory.write(0xD1C7, 0xAA);
    register.set_hl(0xD1C7);
    let mut expected = register.clone();

    let (instruction, argument_type) = get_instruction(opcode);
    let nb_cycle = instruction(&mut register, &mut memory, &argument);

    assert_eq!(nb_cycle, 8);

    expected.a = 0x45 + 0xAA;
    expected.pc = 1;
    assert_eq!(register, expected);
    assert_eq!(argument_type, ImmediateArgumentType::None);
}

#[test]
fn test_add_8bits_immediate() {
    let opcode = 0xC6;
    let mut register = Registers::new();
    let mut memory = FakeMemory::new();
    let argument = Argument::new_u8(0xA0);
    register.a = 0x45;
    let mut expected = register.clone();

    let (instruction, argument_type) = get_instruction(opcode);
    let nb_cycle = instruction(&mut register, &mut memory, &argument);

    assert_eq!(nb_cycle, 8);

    expected.a = 0x45 + 0xA0;
    expected.pc = 2;
    assert_eq!(register, expected);
    assert_eq!(argument_type, ImmediateArgumentType::Unsigned8Bits);
}
