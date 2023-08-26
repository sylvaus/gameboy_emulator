/// Tests for the ADC instruction
/// The ADC instruction is described in chapter 4: page 102-103 of https://ia803208.us.archive.org/9/items/GameBoyProgManVer1.1/GameBoyProgManVer1.1.pdf
use crate::generated::instructions::tests::fixture::{
    FakeMemory, REGISTERS_8_BITS, REGISTERS_8_BITS_WITHOUT_A, REGISTER_A,
};
use crate::generated::instructions::{get_instruction, ImmediateArgumentType};
use crate::memory::argument::Argument;
use crate::memory::registers::Registers;
use crate::memory::Memory;

const ADC_8_BITS_BASE_OPCODE: u16 = 0b10001000;

#[test]
fn test_adc_8bits() {
    for register_info in REGISTERS_8_BITS {
        let opcode = register_info.index + ADC_8_BITS_BASE_OPCODE;
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
fn test_adc_8bits_half_carry() {
    for register_info in REGISTERS_8_BITS {
        let opcode = register_info.index + ADC_8_BITS_BASE_OPCODE;
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
fn test_adc_8bits_carry() {
    for register_info in REGISTERS_8_BITS {
        let opcode = register_info.index + ADC_8_BITS_BASE_OPCODE;
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
fn test_adc_8bits_zero() {
    for register_info in REGISTERS_8_BITS {
        let opcode = register_info.index + ADC_8_BITS_BASE_OPCODE;
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
fn test_adc_8bits_half_carry_with_carry() {
    for register_info in REGISTERS_8_BITS {
        let opcode = register_info.index + ADC_8_BITS_BASE_OPCODE;
        let mut registers = Registers::new();
        let mut memory = FakeMemory::new();
        let argument = Argument::new_empty();
        registers.set_carry_flag(true);
        registers.a = 0x08u8;
        if register_info != &REGISTER_A {
            (register_info.setter)(&mut registers, 0x07u8);
        }
        let mut expected = registers.clone();

        let (instruction, argument_type) = get_instruction(opcode);
        let nb_cycle = instruction(&mut registers, &mut memory, &argument);

        assert_eq!(nb_cycle, 4);

        expected.set_carry_flag(false);
        expected.set_half_carry_flag(true);
        if register_info == &REGISTER_A {
            expected.a = 0x11u8;
        } else {
            expected.a = 0x08u8 + 0x07u8 + 1;
        }
        expected.pc = 1;
        assert_eq!(registers, expected);
        assert_eq!(argument_type, ImmediateArgumentType::None);
    }
}

#[test]
fn test_adc_8bits_carry_with_carry() {
    for register_info in REGISTERS_8_BITS {
        let opcode = register_info.index + ADC_8_BITS_BASE_OPCODE;
        let mut registers = Registers::new();
        let mut memory = FakeMemory::new();
        let argument = Argument::new_empty();
        registers.set_carry_flag(true);
        registers.a = 0x81u8;
        if register_info != &REGISTER_A {
            (register_info.setter)(&mut registers, 0x7Eu8);
        }
        let mut expected = registers.clone();

        let (instruction, argument_type) = get_instruction(opcode);
        let nb_cycle = instruction(&mut registers, &mut memory, &argument);

        assert_eq!(nb_cycle, 4);

        expected.set_carry_flag(true);
        if register_info == &REGISTER_A {
            expected.a = 3;
        } else {
            expected.set_zero_flag(true);
            expected.set_half_carry_flag(true);
            expected.a = 0;
        }
        expected.pc = 1;
        assert_eq!(registers, expected);
        assert_eq!(argument_type, ImmediateArgumentType::None);
    }
}

#[test]
fn test_adc_8bits_zero_with_carry() {
    // Zero flag cannot be achieved for register a when carry flag is set since it doubles the value.
    for register_info in REGISTERS_8_BITS_WITHOUT_A {
        let opcode = register_info.index + ADC_8_BITS_BASE_OPCODE;
        let mut registers = Registers::new();
        let mut memory = FakeMemory::new();
        let argument = Argument::new_empty();
        registers.set_carry_flag(true);
        registers.a = 0x80u8;
        if register_info != &REGISTER_A {
            (register_info.setter)(&mut registers, 0x7Fu8);
        }
        let mut expected = registers.clone();

        let (instruction, argument_type) = get_instruction(opcode);
        let nb_cycle = instruction(&mut registers, &mut memory, &argument);

        assert_eq!(nb_cycle, 4);
        expected.set_zero_flag(true);
        expected.set_carry_flag(true);
        expected.set_half_carry_flag(true);
        expected.a = 0;
        expected.pc = 1;
        assert_eq!(registers, expected);
        assert_eq!(argument_type, ImmediateArgumentType::None);
    }
}

#[test]
fn test_adc_8bits_address() {
    let opcode = 0x8E;
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

    expected.a = 0x45 + 0xAA;
    expected.pc = 1;
    assert_eq!(register, expected);
    assert_eq!(argument_type, ImmediateArgumentType::None);
}

#[test]
fn test_adc_8bits_immediate() {
    let opcode = 0xCE;
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
