/// Tests for the DEC instruction
/// The DEC instruction is described in chapter 4: page 106 and 108 of https://ia803208.us.archive.org/9/items/GameBoyProgManVer1.1/GameBoyProgManVer1.1.pdf
use crate::generated::instructions::tests::fixture::{
    FakeMemory, REGISTERS_16_BITS, REGISTERS_8_BITS,
};
use crate::generated::instructions::{get_instruction, ImmediateArgumentType};
use crate::memory::argument::Argument;
use crate::memory::registers::Registers;
use crate::memory::Memory;

const INC_8_BITS_BASE_OPCODE: u16 = 0b100;
const INC_16_BITS_BASE_OPCODE: u16 = 0b11;

#[test]
fn test_inc_16bits() {
    for register_info in REGISTERS_16_BITS {
        let opcode = (register_info.index << 4) + INC_16_BITS_BASE_OPCODE;
        let mut registers = Registers::new();
        let mut memory = FakeMemory::new();
        let argument = Argument::new_empty();
        (register_info.setter)(&mut registers, 0xA8F1);
        let mut expected = registers.clone();

        let (instruction, argument_type) = get_instruction(opcode);
        let nb_cycle = instruction(&mut registers, &mut memory, &argument);

        assert_eq!(nb_cycle, 8);

        (register_info.setter)(&mut expected, 0xA8F2);
        expected.pc = 1;
        assert_eq!(registers, expected);
        assert_eq!(argument_type, ImmediateArgumentType::None);
    }
}

#[test]
fn test_inc_16bits_wrapping() {
    for register_info in REGISTERS_16_BITS {
        let opcode = (register_info.index << 4) + INC_16_BITS_BASE_OPCODE;
        let mut registers = Registers::new();
        let mut memory = FakeMemory::new();
        let argument = Argument::new_empty();
        (register_info.setter)(&mut registers, 0xFFFF);
        let mut expected = registers.clone();

        let (instruction, argument_type) = get_instruction(opcode);
        let nb_cycle = instruction(&mut registers, &mut memory, &argument);

        assert_eq!(nb_cycle, 8);

        (register_info.setter)(&mut expected, 0x0);
        expected.pc = 1;
        assert_eq!(registers, expected);
        assert_eq!(argument_type, ImmediateArgumentType::None);
    }
}

#[test]
fn test_inc_8bits() {
    for register_info in REGISTERS_8_BITS {
        let opcode = (register_info.index << 3) + INC_8_BITS_BASE_OPCODE;
        let mut registers = Registers::new();
        let mut memory = FakeMemory::new();
        let argument = Argument::new_empty();
        (register_info.setter)(&mut registers, 0xA8);
        let mut expected = registers.clone();

        let (instruction, argument_type) = get_instruction(opcode);
        let nb_cycle = instruction(&mut registers, &mut memory, &argument);

        assert_eq!(nb_cycle, 4);

        (register_info.setter)(&mut expected, 0xA9);
        expected.pc = 1;
        assert_eq!(registers, expected);
        assert_eq!(argument_type, ImmediateArgumentType::None);
    }
}

#[test]
fn test_inc_8bits_half_carry_zero() {
    for register_info in REGISTERS_8_BITS {
        let opcode = (register_info.index << 3) + INC_8_BITS_BASE_OPCODE;
        let mut registers = Registers::new();
        let mut memory = FakeMemory::new();
        let argument = Argument::new_empty();
        (register_info.setter)(&mut registers, 0xFF);
        let mut expected = registers.clone();

        let (instruction, argument_type) = get_instruction(opcode);
        let nb_cycle = instruction(&mut registers, &mut memory, &argument);

        assert_eq!(nb_cycle, 4);

        (register_info.setter)(&mut expected, 0x00);
        expected.set_zero_flag(true);
        expected.set_half_carry_flag(true);
        expected.pc = 1;
        assert_eq!(registers, expected);
        assert_eq!(argument_type, ImmediateArgumentType::None);
    }
}

#[test]
fn test_inc_8bits_address() {
    let opcode = 0x34;
    let mut register = Registers::new();
    let mut memory = FakeMemory::new();
    let argument = Argument::new_empty();
    memory.write(0xD1C7, 0xAA);
    register.set_hl(0xD1C7);
    let mut expected = register.clone();

    let (instruction, argument_type) = get_instruction(opcode);
    let nb_cycle = instruction(&mut register, &mut memory, &argument);

    assert_eq!(nb_cycle, 12);

    memory.assert_value(0xD1C7, 0xAB);
    expected.pc = 1;
    assert_eq!(register, expected);
    assert_eq!(argument_type, ImmediateArgumentType::None);
}
