/// Tests for the SWAP instructions
/// The SWAP instructions is described in chapter 4: page 113 of https://ia803208.us.archive.org/9/items/GameBoyProgManVer1.1/GameBoyProgManVer1.1.pdf
use crate::generated::instructions::tests::fixture::{FakeMemory, REGISTERS_8_BITS};
use crate::generated::instructions::{get_instruction, ImmediateArgumentType};
use crate::memory::argument::Argument;
use crate::memory::registers::Registers;
use crate::memory::Memory;

const SWAP_BASE_OPCODE: u16 = 0b100110000;

#[test]
fn test_swap() {
    for register_info in REGISTERS_8_BITS {
        let opcode = register_info.index + SWAP_BASE_OPCODE;
        let mut registers = Registers::new();
        let mut memory = FakeMemory::new();
        let argument = Argument::new_empty();
        (register_info.setter)(&mut registers, 0b01101100);
        let mut expected = registers.clone();

        let (instruction, argument_type) = get_instruction(opcode);
        let nb_cycle = instruction(&mut registers, &mut memory, &argument);

        assert_eq!(nb_cycle, 8);

        (register_info.setter)(&mut expected, 0b11000110);
        expected.pc = 2;
        assert_eq!(registers, expected);
        assert_eq!(argument_type, ImmediateArgumentType::None);
    }
}

#[test]
fn test_swap_zero() {
    for register_info in REGISTERS_8_BITS {
        let opcode = register_info.index + SWAP_BASE_OPCODE;
        let mut registers = Registers::new();
        let mut memory = FakeMemory::new();
        let argument = Argument::new_empty();
        (register_info.setter)(&mut registers, 0);
        let mut expected = registers.clone();

        let (instruction, argument_type) = get_instruction(opcode);
        let nb_cycle = instruction(&mut registers, &mut memory, &argument);

        assert_eq!(nb_cycle, 8);

        (register_info.setter)(&mut expected, 0);
        expected.set_zero_flag(true);
        expected.pc = 2;
        assert_eq!(registers, expected);
        assert_eq!(argument_type, ImmediateArgumentType::None);
    }
}

#[test]
fn test_swap_address() {
    let opcode = 0b110 + SWAP_BASE_OPCODE;
    let mut registers = Registers::new();
    let mut memory = FakeMemory::new();
    let argument = Argument::new_empty();
    registers.set_hl(0x6F54);
    memory.set(0x6F54, 0b01101100);
    let mut expected = registers.clone();

    let (instruction, argument_type) = get_instruction(opcode);
    let nb_cycle = instruction(&mut registers, &mut memory, &argument);

    assert_eq!(nb_cycle, 16);

    memory.assert_value(0x6F54, 0b11000110);
    expected.pc = 2;
    assert_eq!(registers, expected);
    assert_eq!(argument_type, ImmediateArgumentType::None);
}
