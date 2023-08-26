/// Tests for the CP instruction
/// The CP instruction is described in chapter 4: page 105/6 of https://ia803208.us.archive.org/9/items/GameBoyProgManVer1.1/GameBoyProgManVer1.1.pdf
use crate::generated::instructions::tests::fixture::{
    FakeMemory, REGISTERS_8_BITS_WITHOUT_A, REGISTER_A,
};
use crate::generated::instructions::{get_instruction, ImmediateArgumentType};
use crate::memory::argument::Argument;
use crate::memory::registers::Registers;
use crate::memory::Memory;

const COMPARE_8_BITS_BASE_OPCODE: u16 = 0xB8;

#[test]
fn test_cp_8bits() {
    for register_info in REGISTERS_8_BITS_WITHOUT_A {
        let opcode = register_info.index + COMPARE_8_BITS_BASE_OPCODE;
        let mut registers = Registers::new();
        let mut memory = FakeMemory::new();
        let argument = Argument::new_empty();
        registers.a = 0xAE;
        (register_info.setter)(&mut registers, 0x09u8);
        let mut expected = registers.clone();

        let (instruction, argument_type) = get_instruction(opcode);
        let nb_cycle = instruction(&mut registers, &mut memory, &argument);

        assert_eq!(nb_cycle, 4);

        expected.pc = 1;
        expected.set_add_sub_flag(true);
        assert_eq!(registers, expected);
        assert_eq!(argument_type, ImmediateArgumentType::None);
    }
}

#[test]
fn test_cp_8bits_half_carry() {
    for register_info in REGISTERS_8_BITS_WITHOUT_A {
        let opcode = register_info.index + COMPARE_8_BITS_BASE_OPCODE;
        let mut registers = Registers::new();
        let mut memory = FakeMemory::new();
        let argument = Argument::new_empty();
        registers.a = 0x3C;
        (register_info.setter)(&mut registers, 0x2F);
        let mut expected = registers.clone();

        let (instruction, argument_type) = get_instruction(opcode);
        let nb_cycle = instruction(&mut registers, &mut memory, &argument);

        assert_eq!(nb_cycle, 4);

        expected.pc = 1;
        expected.set_add_sub_flag(true);
        expected.set_half_carry_flag(true);
        assert_eq!(registers, expected);
        assert_eq!(argument_type, ImmediateArgumentType::None);
    }
}

#[test]
fn test_cp_8bits_carry() {
    for register_info in REGISTERS_8_BITS_WITHOUT_A {
        let opcode = register_info.index + COMPARE_8_BITS_BASE_OPCODE;
        let mut registers = Registers::new();
        let mut memory = FakeMemory::new();
        let argument = Argument::new_empty();
        registers.a = 0x3E;
        (register_info.setter)(&mut registers, 0x40);
        let mut expected = registers.clone();

        let (instruction, argument_type) = get_instruction(opcode);
        let nb_cycle = instruction(&mut registers, &mut memory, &argument);

        assert_eq!(nb_cycle, 4);

        expected.pc = 1;
        expected.set_add_sub_flag(true);
        expected.set_carry_flag(true);
        assert_eq!(registers, expected);
        assert_eq!(argument_type, ImmediateArgumentType::None);
    }
}

#[test]
fn test_cp_8bits_zero() {
    for register_info in REGISTERS_8_BITS_WITHOUT_A {
        let opcode = register_info.index + COMPARE_8_BITS_BASE_OPCODE;
        let mut registers = Registers::new();
        let mut memory = FakeMemory::new();
        let argument = Argument::new_empty();
        registers.a = 0x3E;
        (register_info.setter)(&mut registers, 0x3E);
        let mut expected = registers.clone();

        let (instruction, argument_type) = get_instruction(opcode);
        let nb_cycle = instruction(&mut registers, &mut memory, &argument);

        assert_eq!(nb_cycle, 4);

        expected.pc = 1;
        expected.set_add_sub_flag(true);
        expected.set_zero_flag(true);
        assert_eq!(registers, expected);
        assert_eq!(argument_type, ImmediateArgumentType::None);
    }
}

#[test]
fn test_cp_8bits_register_a() {
    let opcode = REGISTER_A.index + COMPARE_8_BITS_BASE_OPCODE;
    let mut registers = Registers::new();
    let mut memory = FakeMemory::new();
    let argument = Argument::new_empty();
    registers.a = 0x3E;
    let mut expected = registers.clone();

    let (instruction, argument_type) = get_instruction(opcode);
    let nb_cycle = instruction(&mut registers, &mut memory, &argument);

    assert_eq!(nb_cycle, 4);

    expected.pc = 1;
    expected.set_add_sub_flag(true);
    expected.set_zero_flag(true);
    assert_eq!(registers, expected);
    assert_eq!(argument_type, ImmediateArgumentType::None);
}

#[test]
fn test_cp_8bits_address() {
    let opcode = 0xBE;
    let mut register = Registers::new();
    let mut memory = FakeMemory::new();
    let argument = Argument::new_empty();
    register.a = 0x45;
    memory.set(0xD1C7, 0xAA);
    register.set_hl(0xD1C7);
    let mut expected = register.clone();

    let (instruction, argument_type) = get_instruction(opcode);
    let nb_cycle = instruction(&mut register, &mut memory, &argument);

    assert_eq!(nb_cycle, 8);

    expected.pc = 1;
    expected.set_add_sub_flag(true);
    expected.set_carry_flag(true);
    expected.set_half_carry_flag(true);
    assert_eq!(register, expected);
    assert_eq!(argument_type, ImmediateArgumentType::None);
}

#[test]
fn test_cp_8bits_immediate() {
    let opcode = 0xFE;
    let mut register = Registers::new();
    let mut memory = FakeMemory::new();
    let argument = Argument::new_u8(0xA0);
    register.a = 0x45;
    let mut expected = register.clone();

    let (instruction, argument_type) = get_instruction(opcode);
    let nb_cycle = instruction(&mut register, &mut memory, &argument);

    assert_eq!(nb_cycle, 8);

    expected.pc = 2;
    expected.set_add_sub_flag(true);
    expected.set_carry_flag(true);
    assert_eq!(register, expected);
    assert_eq!(argument_type, ImmediateArgumentType::Unsigned8Bits);
}
