/// Tests for the XOR instruction
/// The XOR instruction is described in chapter 4: page 105 of https://ia803208.us.archive.org/9/items/GameBoyProgManVer1.1/GameBoyProgManVer1.1.pdf
use crate::generated::instructions::tests::fixture::{
    FakeMemory, REGISTERS_8_BITS, REGISTER_A
};
use crate::generated::instructions::{get_instruction, ImmediateArgumentType};
use crate::memory::argument::Argument;
use crate::memory::registers::Registers;
use crate::memory::Memory;

const XOR_8_BITS_BASE_OPCODE: u16 = 0b10101000;

#[test]
fn test_xor_8bits() {
    for register_info in REGISTERS_8_BITS {
        let opcode = register_info.index + XOR_8_BITS_BASE_OPCODE;
        let mut register = Registers::new();
        let mut memory = FakeMemory::new();
        let argument = Argument::new_empty();
        register.a = 0x5A;
        if register_info != &REGISTER_A {
            (register_info.setter)(&mut register, 0x3F);
        }
        let mut expected = register.clone();

        let (instruction, argument_type) = get_instruction(opcode);
        let nb_cycle = instruction(&mut register, &mut memory, &argument);

        assert_eq!(nb_cycle, 4);

        if register_info == &REGISTER_A {
            expected.set_zero_flag(true);
            expected.a = 0x0;
        } else {
            expected.a = 0x5A ^ 0x3F;
        }
        expected.pc = 1;
        assert_eq!(register, expected);
        assert_eq!(argument_type, ImmediateArgumentType::None);
    }
}

#[test]
fn test_xor_8bits_zero() {
    for register_info in REGISTERS_8_BITS {
        let opcode = register_info.index + XOR_8_BITS_BASE_OPCODE;
        let mut registers = Registers::new();
        let mut memory = FakeMemory::new();
        let argument = Argument::new_empty();
        registers.a = 0b1010110;
        (register_info.setter)(&mut registers, 0b1010110);
        let mut expected = registers.clone();

        let (instruction, argument_type) = get_instruction(opcode);
        let nb_cycle = instruction(&mut registers, &mut memory, &argument);

        assert_eq!(nb_cycle, 4);
        expected.set_zero_flag(true);
        expected.a = 0;
        expected.pc = 1;
        assert_eq!(registers, expected);
        assert_eq!(argument_type, ImmediateArgumentType::None);
    }
}

#[test]
fn test_xor_8bits_address() {
    let opcode = 0xAE;
    let mut register = Registers::new();
    let mut memory = FakeMemory::new();
    let argument = Argument::new_empty();
    register.a = 0b11110101;
    memory.set(0xD1C7, 0b10101111);
    register.set_hl(0xD1C7);
    let mut expected = register.clone();

    let (instruction, argument_type) = get_instruction(opcode);
    let nb_cycle = instruction(&mut register, &mut memory, &argument);

    assert_eq!(nb_cycle, 8);

    expected.a = 0b11110101 ^ 0b10101111;
    expected.pc = 1;
    assert_eq!(register, expected);
    assert_eq!(argument_type, ImmediateArgumentType::None);
}

#[test]
fn test_xor_8bits_immediate() {
    let opcode = 0xEE;
    let mut register = Registers::new();
    let mut memory = FakeMemory::new();
    let argument = Argument::new_u8(0b10101111);
    register.a = 0b11110101;
    let mut expected = register.clone();

    let (instruction, argument_type) = get_instruction(opcode);
    let nb_cycle = instruction(&mut register, &mut memory, &argument);

    assert_eq!(nb_cycle, 8);

    expected.a = 0b11110101 ^ 0b10101111;
    expected.pc = 2;
    assert_eq!(register, expected);
    assert_eq!(argument_type, ImmediateArgumentType::Unsigned8Bits);
}