/// Tests for the BIT/RES/SET instruction
/// The BIT/RES/SET instruction is described in chapter 4: page 114-115 of https://ia803208.us.archive.org/9/items/GameBoyProgManVer1.1/GameBoyProgManVer1.1.pdf
use crate::generated::instructions::tests::fixture::{FakeMemory, REGISTERS_8_BITS, REGISTER_A};
use crate::generated::instructions::{get_instruction, ImmediateArgumentType};
use crate::memory::argument::Argument;
use crate::memory::registers::Registers;
use crate::memory::Memory;

const BIT_8_BITS_BASE_OPCODE: u16 = 0b101000000;
const RES_8_BITS_BASE_OPCODE: u16 = 0b110000000;
const SET_8_BITS_BASE_OPCODE: u16 = 0b111000000;

const BIT_INDEXES: &[u16] = &[0, 1, 2, 3, 4, 5, 6, 7];

/// BIT sets the zero flag to the inverse of the bit value of the register.
#[test]
fn test_bit_8bits_bit_is_set_to_zero() {
    for register_info in REGISTERS_8_BITS {
        for bit_index in BIT_INDEXES {
            let opcode = register_info.index + (bit_index << 3) + BIT_8_BITS_BASE_OPCODE;
            let mut register = Registers::new();
            let mut memory = FakeMemory::new();
            let argument = Argument::new_empty();
            (register_info.setter)(&mut register, 0xFF - (1 << bit_index));
            let mut expected = register.clone();

            let (instruction, argument_type) = get_instruction(opcode);
            let nb_cycle = instruction(&mut register, &mut memory, &argument);

            assert_eq!(nb_cycle, 8);

            expected.set_half_carry_flag(true);
            expected.set_zero_flag(true);
            expected.pc = 2;
            assert_eq!(register, expected);
            assert_eq!(argument_type, ImmediateArgumentType::None);
        }
    }
}
#[test]
fn test_bit_8bits_bit_is_set_to_one() {
    for register_info in REGISTERS_8_BITS {
        for bit_index in BIT_INDEXES {
            let opcode = register_info.index + (bit_index << 3) + BIT_8_BITS_BASE_OPCODE;
            let mut register = Registers::new();
            let mut memory = FakeMemory::new();
            let argument = Argument::new_empty();
            (register_info.setter)(&mut register, 1 << bit_index);
            let mut expected = register.clone();

            let (instruction, argument_type) = get_instruction(opcode);
            let nb_cycle = instruction(&mut register, &mut memory, &argument);

            assert_eq!(nb_cycle, 8);

            expected.set_half_carry_flag(true);
            expected.set_zero_flag(false);
            expected.pc = 2;
            assert_eq!(register, expected);
            assert_eq!(argument_type, ImmediateArgumentType::None);
        }
    }
}

/// RES resets the zero flag to the inverse of the bit value of the register.
#[test]
fn test_res_8bits() {
    for register_info in REGISTERS_8_BITS {
        for bit_index in BIT_INDEXES {
            let opcode = register_info.index + (bit_index << 3) + RES_8_BITS_BASE_OPCODE;
            let mut register = Registers::new();
            let mut memory = FakeMemory::new();
            let argument = Argument::new_empty();
            (register_info.setter)(&mut register, 0xFF);
            let mut expected = register.clone();

            let (instruction, argument_type) = get_instruction(opcode);
            let nb_cycle = instruction(&mut register, &mut memory, &argument);

            assert_eq!(nb_cycle, 8);

            (register_info.setter)(&mut expected, 0xFF - (1 << bit_index));
            expected.pc = 2;
            assert_eq!(register, expected);
            assert_eq!(argument_type, ImmediateArgumentType::None);
        }
    }
}

/// SET sets the zero flag to the inverse of the bit value of the register.
#[test]
fn test_set_8bits() {
    for register_info in REGISTERS_8_BITS {
        for bit_index in BIT_INDEXES {
            let opcode = register_info.index + (bit_index << 3) + SET_8_BITS_BASE_OPCODE;
            let mut register = Registers::new();
            let mut memory = FakeMemory::new();
            let argument = Argument::new_empty();
            (register_info.setter)(&mut register, 0);
            let mut expected = register.clone();

            let (instruction, argument_type) = get_instruction(opcode);
            let nb_cycle = instruction(&mut register, &mut memory, &argument);

            assert_eq!(nb_cycle, 8);

            (register_info.setter)(&mut expected, (1 << bit_index));
            expected.pc = 2;
            assert_eq!(register, expected);
            assert_eq!(argument_type, ImmediateArgumentType::None);
        }
    }
}

/// BIT sets the zero flag to the inverse of the bit value of the register.
#[test]
fn test_bit_8bits_bit_is_set_to_zero_address() {
    for bit_index in BIT_INDEXES {
        let opcode = (bit_index << 3) + 0b101000110;
        let mut register = Registers::new();
        let mut memory = FakeMemory::new();
        let argument = Argument::new_empty();
        memory.set(0xD1C7, 0xFF - (1 << bit_index));
        register.set_hl(0xD1C7);
        let mut expected = register.clone();

        let (instruction, argument_type) = get_instruction(opcode);
        let nb_cycle = instruction(&mut register, &mut memory, &argument);

        assert_eq!(nb_cycle, 12);

        expected.set_half_carry_flag(true);
        expected.set_zero_flag(true);
        expected.pc = 2;
        assert_eq!(register, expected);
        assert_eq!(argument_type, ImmediateArgumentType::None);
    }
}
#[test]
fn test_bit_8bits_bit_is_set_to_one_address() {
    for bit_index in BIT_INDEXES {
        let opcode = (bit_index << 3) + 0b101000110;
        let mut register = Registers::new();
        let mut memory = FakeMemory::new();
        let argument = Argument::new_empty();
        memory.set(0xD1C7, 1 << bit_index);
        register.set_hl(0xD1C7);
        let mut expected = register.clone();

        let (instruction, argument_type) = get_instruction(opcode);
        let nb_cycle = instruction(&mut register, &mut memory, &argument);

        assert_eq!(nb_cycle, 12);

        expected.set_half_carry_flag(true);
        expected.set_zero_flag(false);
        expected.pc = 2;
        assert_eq!(register, expected);
        assert_eq!(argument_type, ImmediateArgumentType::None);
    }
}

/// RES resets the zero flag to the inverse of the bit value of the register.
#[test]
fn test_res_8bits_address() {
    for bit_index in BIT_INDEXES {
        let opcode = (bit_index << 3) + 0b110000110;
        let mut register = Registers::new();
        let mut memory = FakeMemory::new();
        let argument = Argument::new_empty();
        memory.set(0xD1C7, 0xFF);
        register.set_hl(0xD1C7);
        let mut expected = register.clone();

        let (instruction, argument_type) = get_instruction(opcode);
        let nb_cycle = instruction(&mut register, &mut memory, &argument);

        assert_eq!(nb_cycle, 16);

        memory.assert_value(0xD1C7, 0xFF - (1 << bit_index));
        expected.pc = 2;
        assert_eq!(register, expected);
        assert_eq!(argument_type, ImmediateArgumentType::None);
    }
}

/// SET sets the zero flag to the inverse of the bit value of the register.
#[test]
fn test_set_8bits_address() {
    for bit_index in BIT_INDEXES {
        let opcode =  (bit_index << 3) + 0b111000110;
        let mut register = Registers::new();
        let mut memory = FakeMemory::new();
        let argument = Argument::new_empty();
        memory.set(0xD1C7, 0);
        register.set_hl(0xD1C7);
        let mut expected = register.clone();

        let (instruction, argument_type) = get_instruction(opcode);
        let nb_cycle = instruction(&mut register, &mut memory, &argument);

        assert_eq!(nb_cycle, 16);

        memory.assert_value(0xD1C7, 1 << bit_index);
        expected.pc = 2;
        assert_eq!(register, expected);
        assert_eq!(argument_type, ImmediateArgumentType::None);
    }
}
