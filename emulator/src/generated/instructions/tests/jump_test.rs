/// Tests for the JUMP instruction
/// The JUMP instruction is described in chapter 4: page 116 and 117 of https://ia803208.us.archive.org/9/items/GameBoyProgManVer1.1/GameBoyProgManVer1.1.pdf
use crate::generated::instructions::tests::fixture::{FakeMemory, FLAG_INFOS};
use crate::generated::instructions::{get_instruction, ImmediateArgumentType};
use crate::memory::argument::Argument;
use crate::memory::registers::Registers;

const JP_BASE_OPCODE: u16 = 0xC2;

#[test]
fn test_jump_relative_unconditional_positive() {
    let opcode = 0x18;
    let mut register = Registers::new();
    let mut memory = FakeMemory::new();
    let argument = Argument::new_i8(0x5C);
    register.pc = 1234;
    let mut expected = register.clone();

    let (instruction, argument_type) = get_instruction(opcode);
    let nb_cycle = instruction(&mut register, &mut memory, &argument);

    assert_eq!(nb_cycle, 12);

    expected.pc += 2 + 0x5C;
    assert_eq!(register, expected);
    assert_eq!(argument_type, ImmediateArgumentType::Signed8Bits);
}

#[test]
fn test_jump_relative_unconditional_negative() {
    let opcode = 0x18;
    let mut register = Registers::new();
    let mut memory = FakeMemory::new();
    let argument = Argument::new_i8(-0x5C);
    register.pc = 1234;
    let mut expected = register.clone();

    let (instruction, argument_type) = get_instruction(opcode);
    let nb_cycle = instruction(&mut register, &mut memory, &argument);

    assert_eq!(nb_cycle, 12);

    expected.pc -= 0x5C - 2;
    assert_eq!(register, expected);
    assert_eq!(argument_type, ImmediateArgumentType::Signed8Bits);
}

#[test]
fn test_jump_conditional_true() {
    for flag_info in FLAG_INFOS {
        let opcode = (flag_info.index << 3) + JP_BASE_OPCODE;
        let mut register = Registers::new();
        let mut memory = FakeMemory::new();
        let argument = Argument::new_u16(0x8000);
        register.pc = 0x1234;
        (flag_info.setter)(&mut register, true);
        let mut expected = register.clone();

        let (instruction, argument_type) = get_instruction(opcode);
        let nb_cycle = instruction(&mut register, &mut memory, &argument);

        assert_eq!(nb_cycle, 16);

        expected.pc = 0x8000;
        assert_eq!(register, expected);
        assert_eq!(argument_type, ImmediateArgumentType::Unsigned16Bits);
    }
}

#[test]
fn test_jump_conditional_false() {
    for flag_info in FLAG_INFOS {
        let opcode = (flag_info.index << 3) + JP_BASE_OPCODE;
        let mut register = Registers::new();
        let mut memory = FakeMemory::new();
        let argument = Argument::new_u16(0x8000);
        register.pc = 0x1234;
        (flag_info.setter)(&mut register, false);
        let mut expected = register.clone();

        let (instruction, argument_type) = get_instruction(opcode);
        let nb_cycle = instruction(&mut register, &mut memory, &argument);

        assert_eq!(nb_cycle, 12);

        expected.pc += 3;
        assert_eq!(register, expected);
        assert_eq!(argument_type, ImmediateArgumentType::Unsigned16Bits);
    }
}

#[test]
fn test_jump_immediate() {
    let opcode = 0xC3;
    let mut register = Registers::new();
    let mut memory = FakeMemory::new();
    let argument = Argument::new_u16(0x89AB);
    register.pc = 1234;
    let mut expected = register.clone();

    let (instruction, argument_type) = get_instruction(opcode);
    let nb_cycle = instruction(&mut register, &mut memory, &argument);

    assert_eq!(nb_cycle, 16);

    expected.pc = 0x89AB;
    assert_eq!(register, expected);
    assert_eq!(argument_type, ImmediateArgumentType::Unsigned16Bits);
}

#[test]
fn test_jump_hl() {
    let opcode = 0xE9;
    let mut register = Registers::new();
    let mut memory = FakeMemory::new();
    let argument = Argument::new_empty();
    register.pc = 1234;
    register.set_hl(0xFEDC);
    let mut expected = register.clone();

    let (instruction, argument_type) = get_instruction(opcode);
    let nb_cycle = instruction(&mut register, &mut memory, &argument);

    assert_eq!(nb_cycle, 4);

    expected.pc = 0xFEDC;
    assert_eq!(register, expected);
    assert_eq!(argument_type, ImmediateArgumentType::None);
}
