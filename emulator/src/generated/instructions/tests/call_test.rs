/// Tests for the CALL instruction
/// The CALL instruction is described in chapter 4: page 118 of https://ia803208.us.archive.org/9/items/GameBoyProgManVer1.1/GameBoyProgManVer1.1.pdf
use crate::generated::instructions::tests::fixture::{FakeMemory, FLAG_INFOS};
use crate::generated::instructions::{get_instruction, ImmediateArgumentType};
use crate::memory::argument::Argument;
use crate::memory::registers::Registers;

const CONDITIONAL_CALL_BASE: u16 = 0xC4;

#[test]
fn test_conditional_call_condition_true() {
    for flag_info in FLAG_INFOS {
        let opcode = (flag_info.index << 3) + CONDITIONAL_CALL_BASE;
        let mut registers = Registers::new();
        let mut memory = FakeMemory::new();
        let argument = Argument::new_u16(0x1234);
        registers.pc = 0x8000;
        registers.sp = 0xFFFE;
        (flag_info.setter)(&mut registers, true);
        let mut expected = registers.clone();

        let (instruction, argument_type) = get_instruction(opcode);
        let nb_cycle = instruction(&mut registers, &mut memory, &argument);

        assert_eq!(nb_cycle, 24);

        expected.pc = 0x1234;
        expected.sp = 0xFFFC;
        memory.assert_value(0xFFFD, 0x80);
        memory.assert_value(0xFFFC, 0x03);
        assert_eq!(registers, expected);
        assert_eq!(argument_type, ImmediateArgumentType::Unsigned16Bits);
    }
}

#[test]
fn test_conditional_call_condition_false() {
    for flag_info in FLAG_INFOS {
        let opcode = (flag_info.index << 3) + CONDITIONAL_CALL_BASE;
        let mut registers = Registers::new();
        let mut memory = FakeMemory::new();
        let argument = Argument::new_u16(0x1234);
        registers.pc = 0x8000;
        registers.sp = 0xFFFE;
        (flag_info.setter)(&mut registers, false);
        let mut expected = registers.clone();

        let (instruction, argument_type) = get_instruction(opcode);
        let nb_cycle = instruction(&mut registers, &mut memory, &argument);

        assert_eq!(nb_cycle, 12);

        expected.pc = 0x8003;
        memory.assert_never_updated();
        assert_eq!(registers, expected);
        assert_eq!(argument_type, ImmediateArgumentType::Unsigned16Bits);
    }
}

#[test]
fn test_unconditional_call() {
    let opcode = 0xCD;
    let mut registers = Registers::new();
    let mut memory = FakeMemory::new();
    let argument = Argument::new_u16(0x1234);
    registers.pc = 0x8000;
    registers.sp = 0xFFFE;
    let mut expected = registers.clone();

    let (instruction, argument_type) = get_instruction(opcode);
    let nb_cycle = instruction(&mut registers, &mut memory, &argument);

    assert_eq!(nb_cycle, 24);

    expected.pc = 0x1234;
    expected.sp = 0xFFFC;
    memory.assert_value(0xFFFD, 0x80);
    memory.assert_value(0xFFFC, 0x03);
    assert_eq!(registers, expected);
    assert_eq!(argument_type, ImmediateArgumentType::Unsigned16Bits);
}
