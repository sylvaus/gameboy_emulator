/// Tests for the General Purpose Arithmetic instructions
/// The General Purpose Arithmetic instruction are described in chapter 4: page 122/3 of https://ia803208.us.archive.org/9/items/GameBoyProgManVer1.1/GameBoyProgManVer1.1.pdf
use crate::generated::instructions::tests::fixture::FakeMemory;
use crate::generated::instructions::{get_instruction, ImmediateArgumentType};
use crate::memory::argument::Argument;
use crate::memory::registers::Registers;

#[test]
fn test_nop() {
    let opcode = 0;
    let mut register = Registers::new();
    let mut memory = FakeMemory::new();
    let argument = Argument::new_empty();
    let mut expected = register.clone();

    let (instruction, argument_type) = get_instruction(opcode);
    let nb_cycle = instruction(&mut register, &mut memory, &argument);

    assert_eq!(nb_cycle, 4);

    expected.pc = 1;
    assert_eq!(register, expected);
    assert_eq!(argument_type, ImmediateArgumentType::None);
}

#[test]
fn test_cpl() {
    for i in 0..=0xFFu8 {
        let opcode = 0x2F;
        let mut registers = Registers::new();
        let mut memory = FakeMemory::new();
        let argument = Argument::new_empty();
        registers.a = i;
        let mut expected = registers.clone();

        let (instruction, argument_type) = get_instruction(opcode);
        let nb_cycle = instruction(&mut registers, &mut memory, &argument);

        assert_eq!(nb_cycle, 4);

        expected.pc = 1;
        expected.a = !i;
        expected.set_half_carry_flag(true);
        expected.set_add_sub_flag(true);
        assert_eq!(registers, expected);
        assert_eq!(argument_type, ImmediateArgumentType::None);
    }
}

#[test]
fn test_scf() {
    let opcode = 0x37;
    let mut registers = Registers::new();
    let mut memory = FakeMemory::new();
    let argument = Argument::new_empty();
    registers.set_half_carry_flag(true);
    registers.set_add_sub_flag(true);
    registers.set_zero_flag(true);
    let mut expected = registers.clone();

    let (instruction, argument_type) = get_instruction(opcode);
    let nb_cycle = instruction(&mut registers, &mut memory, &argument);

    assert_eq!(nb_cycle, 4);

    expected.pc = 1;
    expected.set_carry_flag(true);
    expected.set_half_carry_flag(false);
    expected.set_add_sub_flag(false);
    assert_eq!(registers, expected);
    assert_eq!(argument_type, ImmediateArgumentType::None);
}

#[test]
fn test_ccf() {
    for carry_flag in [true, false] {
        let opcode = 0x3F;
        let mut registers = Registers::new();
        let mut memory = FakeMemory::new();
        let argument = Argument::new_empty();
        registers.set_carry_flag(carry_flag);
        registers.set_half_carry_flag(true);
        registers.set_add_sub_flag(true);
        registers.set_zero_flag(true);
        let mut expected = registers.clone();

        let (instruction, argument_type) = get_instruction(opcode);
        let nb_cycle = instruction(&mut registers, &mut memory, &argument);

        assert_eq!(nb_cycle, 4);

        expected.pc = 1;
        expected.set_carry_flag(!carry_flag);
        expected.set_half_carry_flag(false);
        expected.set_add_sub_flag(false);
        registers.set_zero_flag(true);
        assert_eq!(registers, expected);
        assert_eq!(argument_type, ImmediateArgumentType::None);
    }
}
