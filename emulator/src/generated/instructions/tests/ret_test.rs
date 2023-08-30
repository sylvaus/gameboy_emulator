/// Tests for the RET/RETI instruction
/// The RET/RETI instruction is described in chapter 4: page 119/120 of https://ia803208.us.archive.org/9/items/GameBoyProgManVer1.1/GameBoyProgManVer1.1.pdf
use crate::generated::instructions::tests::fixture::{FakeMemory, FLAG_INFOS};
use crate::generated::instructions::{get_instruction, ImmediateArgumentType};
use crate::memory::argument::Argument;
use crate::memory::registers::Registers;
use crate::memory::Memory;

#[test]
fn test_ret_conditional_true() {
    for flag_info in FLAG_INFOS {
        let opcode = (flag_info.index << 3) + 0b11000000;
        let mut register = Registers::new();
        let mut memory = FakeMemory::new();
        let argument = Argument::new_empty();
        register.sp = 0x1234;
        register.pc = 0x2458;
        (flag_info.setter)(&mut register, true);
        memory.write(0x1234, 0xFE);
        memory.write(0x1235, 0xCA);
        let mut expected = register.clone();

        let (instruction, argument_type) = get_instruction(opcode);
        let nb_cycle = instruction(&mut register, &mut memory, &argument);

        assert_eq!(nb_cycle, 20);

        expected.sp = 0x1234 + 2;
        expected.pc = 0xCAFE;
        assert_eq!(register, expected);
        assert_eq!(argument_type, ImmediateArgumentType::None);
    }
}

#[test]
fn test_ret_conditional_false() {
    for flag_info in FLAG_INFOS {
        let opcode = (flag_info.index << 3) + 0b11000000;
        let mut register = Registers::new();
        let mut memory = FakeMemory::new();
        let argument = Argument::new_empty();
        register.sp = 0x1234;
        register.pc = 0x2458;
        (flag_info.setter)(&mut register, false);
        let mut expected = register.clone();

        let (instruction, argument_type) = get_instruction(opcode);
        let nb_cycle = instruction(&mut register, &mut memory, &argument);

        assert_eq!(nb_cycle, 8);

        expected.pc += 1;
        assert_eq!(register, expected);
        assert_eq!(argument_type, ImmediateArgumentType::None);
    }
}

#[test]
fn test_ret_unconditional() {
    let opcode = 0b11001001;
    let mut register = Registers::new();
    let mut memory = FakeMemory::new();
    let argument = Argument::new_empty();
    register.sp = 0x1234;
    register.pc = 0x2458;
    memory.write(0x1234, 0xFE);
    memory.write(0x1235, 0xCA);
    let mut expected = register.clone();

    let (instruction, argument_type) = get_instruction(opcode);
    let nb_cycle = instruction(&mut register, &mut memory, &argument);

    assert_eq!(nb_cycle, 16);

    expected.sp = 0x1234 + 2;
    expected.pc = 0xCAFE;
    assert_eq!(register, expected);
    assert_eq!(argument_type, ImmediateArgumentType::None);
}

#[test]
fn test_reti() {
    let opcode = 0b11011001;
    let mut register = Registers::new();
    let mut memory = FakeMemory::new();
    let argument = Argument::new_empty();
    register.sp = 0x1234;
    register.pc = 0x2458;
    memory.write(0x1234, 0xFE);
    memory.write(0x1235, 0xCA);
    let mut expected = register.clone();

    let (instruction, argument_type) = get_instruction(opcode);
    let nb_cycle = instruction(&mut register, &mut memory, &argument);

    assert_eq!(nb_cycle, 16);

    expected.ime_flag = true;
    expected.sp = 0x1234 + 2;
    expected.pc = 0xCAFE;
    assert_eq!(register, expected);
    assert_eq!(argument_type, ImmediateArgumentType::None);
}
