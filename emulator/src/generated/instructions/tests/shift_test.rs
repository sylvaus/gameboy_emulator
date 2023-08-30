/// Tests for the rotate instructions
/// The rotate instructions is described in chapter 4: page 109-111 of https://ia803208.us.archive.org/9/items/GameBoyProgManVer1.1/GameBoyProgManVer1.1.pdf
use crate::generated::instructions::tests::fixture::{FakeMemory, REGISTERS_8_BITS};
use crate::generated::instructions::{get_instruction, ImmediateArgumentType};
use crate::memory::argument::Argument;
use crate::memory::registers::Registers;
use crate::memory::Memory;

const SLA_BASE_OPCODE: u16 = 0b100100000;
const SRA_BASE_OPCODE: u16 = 0b100101000;
const SRL_BASE_OPCODE: u16 = 0b100111000;

#[test]
fn test_sla() {
    for register_info in REGISTERS_8_BITS {
        let opcode = register_info.index + SLA_BASE_OPCODE;
        let mut registers = Registers::new();
        let mut memory = FakeMemory::new();
        let argument = Argument::new_empty();
        (register_info.setter)(&mut registers, 0b01101100);
        let mut expected = registers.clone();

        let (instruction, argument_type) = get_instruction(opcode);
        let nb_cycle = instruction(&mut registers, &mut memory, &argument);

        assert_eq!(nb_cycle, 8);

        (register_info.setter)(&mut expected, 0b01101100 << 1);
        expected.pc = 2;
        assert_eq!(registers, expected);
        assert_eq!(argument_type, ImmediateArgumentType::None);
    }
}

#[test]
fn test_sla_carry() {
    for register_info in REGISTERS_8_BITS {
        let opcode = register_info.index + SLA_BASE_OPCODE;
        let mut registers = Registers::new();
        let mut memory = FakeMemory::new();
        let argument = Argument::new_empty();
        (register_info.setter)(&mut registers, 0x85);
        let mut expected = registers.clone();

        let (instruction, argument_type) = get_instruction(opcode);
        let nb_cycle = instruction(&mut registers, &mut memory, &argument);

        assert_eq!(nb_cycle, 8);

        (register_info.setter)(&mut expected, 0x0A);
        expected.set_carry_flag(true);
        expected.pc = 2;
        assert_eq!(registers, expected);
        assert_eq!(argument_type, ImmediateArgumentType::None);
    }
}

#[test]
fn test_sla_zero() {
    for register_info in REGISTERS_8_BITS {
        let opcode = register_info.index + SLA_BASE_OPCODE;
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
fn test_sla_carry_and_zero() {
    for register_info in REGISTERS_8_BITS {
        let opcode = register_info.index + SLA_BASE_OPCODE;
        let mut registers = Registers::new();
        let mut memory = FakeMemory::new();
        let argument = Argument::new_empty();
        (register_info.setter)(&mut registers, 0x80);
        let mut expected = registers.clone();

        let (instruction, argument_type) = get_instruction(opcode);
        let nb_cycle = instruction(&mut registers, &mut memory, &argument);

        assert_eq!(nb_cycle, 8);

        (register_info.setter)(&mut expected, 0);
        expected.set_zero_flag(true);
        expected.set_carry_flag(true);
        expected.pc = 2;
        assert_eq!(registers, expected);
        assert_eq!(argument_type, ImmediateArgumentType::None);
    }
}

#[test]
fn test_sla_address() {
    let opcode = 0b110 + SLA_BASE_OPCODE;
    let mut registers = Registers::new();
    let mut memory = FakeMemory::new();
    let argument = Argument::new_empty();
    registers.set_hl(0xD1C7);
    memory.write(0xD1C7, 0x6A);
    let mut expected = registers.clone();

    let (instruction, argument_type) = get_instruction(opcode);
    let nb_cycle = instruction(&mut registers, &mut memory, &argument);

    assert_eq!(nb_cycle, 16);

    memory.assert_value(0xD1C7, 0x6A << 1);
    expected.pc = 2;
    assert_eq!(registers, expected);
    assert_eq!(argument_type, ImmediateArgumentType::None);
}

#[test]
fn test_sra() {
    for register_info in REGISTERS_8_BITS {
        let opcode = register_info.index + SRA_BASE_OPCODE;
        let mut registers = Registers::new();
        let mut memory = FakeMemory::new();
        let argument = Argument::new_empty();
        (register_info.setter)(&mut registers, 0x6A);
        let mut expected = registers.clone();

        let (instruction, argument_type) = get_instruction(opcode);
        let nb_cycle = instruction(&mut registers, &mut memory, &argument);

        assert_eq!(nb_cycle, 8);

        (register_info.setter)(&mut expected, 0x6A >> 1);
        expected.pc = 2;
        assert_eq!(registers, expected);
        assert_eq!(argument_type, ImmediateArgumentType::None);
    }
}

#[test]
fn test_sra_highest_bit_set() {
    for register_info in REGISTERS_8_BITS {
        let opcode = register_info.index + SRA_BASE_OPCODE;
        let mut registers = Registers::new();
        let mut memory = FakeMemory::new();
        let argument = Argument::new_empty();
        (register_info.setter)(&mut registers, 0x8A);
        let mut expected = registers.clone();

        let (instruction, argument_type) = get_instruction(opcode);
        let nb_cycle = instruction(&mut registers, &mut memory, &argument);

        assert_eq!(nb_cycle, 8);

        (register_info.setter)(&mut expected, 0xC5);
        expected.pc = 2;
        assert_eq!(registers, expected);
        assert_eq!(argument_type, ImmediateArgumentType::None);
    }
}

#[test]
fn test_sra_carry() {
    for register_info in REGISTERS_8_BITS {
        let opcode = register_info.index + SRA_BASE_OPCODE;
        let mut registers = Registers::new();
        let mut memory = FakeMemory::new();
        let argument = Argument::new_empty();
        (register_info.setter)(&mut registers, 0x11);
        let mut expected = registers.clone();

        let (instruction, argument_type) = get_instruction(opcode);
        let nb_cycle = instruction(&mut registers, &mut memory, &argument);

        assert_eq!(nb_cycle, 8);

        (register_info.setter)(&mut expected, 0x08);
        expected.set_carry_flag(true);
        expected.pc = 2;
        assert_eq!(registers, expected);
        assert_eq!(argument_type, ImmediateArgumentType::None);
    }
}

#[test]
fn test_sra_zero() {
    for register_info in REGISTERS_8_BITS {
        let opcode = register_info.index + SRA_BASE_OPCODE;
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
fn test_sra_carry_and_zero() {
    for register_info in REGISTERS_8_BITS {
        let opcode = register_info.index + SRA_BASE_OPCODE;
        let mut registers = Registers::new();
        let mut memory = FakeMemory::new();
        let argument = Argument::new_empty();
        (register_info.setter)(&mut registers, 0x1);
        let mut expected = registers.clone();

        let (instruction, argument_type) = get_instruction(opcode);
        let nb_cycle = instruction(&mut registers, &mut memory, &argument);

        assert_eq!(nb_cycle, 8);

        (register_info.setter)(&mut expected, 0);
        expected.set_zero_flag(true);
        expected.set_carry_flag(true);
        expected.pc = 2;
        assert_eq!(registers, expected);
        assert_eq!(argument_type, ImmediateArgumentType::None);
    }
}

#[test]
fn test_sra_address() {
    let opcode = 0b110 + SRA_BASE_OPCODE;
    let mut registers = Registers::new();
    let mut memory = FakeMemory::new();
    let argument = Argument::new_empty();
    registers.set_hl(0xD1C7);
    memory.write(0xD1C7, 0x6A);
    let mut expected = registers.clone();

    let (instruction, argument_type) = get_instruction(opcode);
    let nb_cycle = instruction(&mut registers, &mut memory, &argument);

    assert_eq!(nb_cycle, 16);

    memory.assert_value(0xD1C7, 0x6A >> 1);
    expected.pc = 2;
    assert_eq!(registers, expected);
    assert_eq!(argument_type, ImmediateArgumentType::None);
}

#[test]
fn test_srl() {
    for register_info in REGISTERS_8_BITS {
        let opcode = register_info.index + SRL_BASE_OPCODE;
        let mut registers = Registers::new();
        let mut memory = FakeMemory::new();
        let argument = Argument::new_empty();
        (register_info.setter)(&mut registers, 0x6A);
        let mut expected = registers.clone();

        let (instruction, argument_type) = get_instruction(opcode);
        let nb_cycle = instruction(&mut registers, &mut memory, &argument);

        assert_eq!(nb_cycle, 8);

        (register_info.setter)(&mut expected, 0x6A >> 1);
        expected.pc = 2;
        assert_eq!(registers, expected);
        assert_eq!(argument_type, ImmediateArgumentType::None);
    }
}

#[test]
fn test_srl_highest_bit_set() {
    for register_info in REGISTERS_8_BITS {
        let opcode = register_info.index + SRL_BASE_OPCODE;
        let mut registers = Registers::new();
        let mut memory = FakeMemory::new();
        let argument = Argument::new_empty();
        (register_info.setter)(&mut registers, 0x8A);
        let mut expected = registers.clone();

        let (instruction, argument_type) = get_instruction(opcode);
        let nb_cycle = instruction(&mut registers, &mut memory, &argument);

        assert_eq!(nb_cycle, 8);

        (register_info.setter)(&mut expected, 0x8A >> 1);
        expected.pc = 2;
        assert_eq!(registers, expected);
        assert_eq!(argument_type, ImmediateArgumentType::None);
    }
}

#[test]
fn test_srl_carry() {
    for register_info in REGISTERS_8_BITS {
        let opcode = register_info.index + SRL_BASE_OPCODE;
        let mut registers = Registers::new();
        let mut memory = FakeMemory::new();
        let argument = Argument::new_empty();
        (register_info.setter)(&mut registers, 0x11);
        let mut expected = registers.clone();

        let (instruction, argument_type) = get_instruction(opcode);
        let nb_cycle = instruction(&mut registers, &mut memory, &argument);

        assert_eq!(nb_cycle, 8);

        (register_info.setter)(&mut expected, 0x08);
        expected.set_carry_flag(true);
        expected.pc = 2;
        assert_eq!(registers, expected);
        assert_eq!(argument_type, ImmediateArgumentType::None);
    }
}

#[test]
fn test_srl_zero() {
    for register_info in REGISTERS_8_BITS {
        let opcode = register_info.index + SRL_BASE_OPCODE;
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
fn test_srl_carry_and_zero() {
    for register_info in REGISTERS_8_BITS {
        let opcode = register_info.index + SRL_BASE_OPCODE;
        let mut registers = Registers::new();
        let mut memory = FakeMemory::new();
        let argument = Argument::new_empty();
        (register_info.setter)(&mut registers, 0x1);
        let mut expected = registers.clone();

        let (instruction, argument_type) = get_instruction(opcode);
        let nb_cycle = instruction(&mut registers, &mut memory, &argument);

        assert_eq!(nb_cycle, 8);

        (register_info.setter)(&mut expected, 0);
        expected.set_zero_flag(true);
        expected.set_carry_flag(true);
        expected.pc = 2;
        assert_eq!(registers, expected);
        assert_eq!(argument_type, ImmediateArgumentType::None);
    }
}

#[test]
fn test_srl_address() {
    let opcode = 0b110 + SRL_BASE_OPCODE;
    let mut registers = Registers::new();
    let mut memory = FakeMemory::new();
    let argument = Argument::new_empty();
    registers.set_hl(0xD1C7);
    memory.write(0xD1C7, 0x6A);
    let mut expected = registers.clone();

    let (instruction, argument_type) = get_instruction(opcode);
    let nb_cycle = instruction(&mut registers, &mut memory, &argument);

    assert_eq!(nb_cycle, 16);

    memory.assert_value(0xD1C7, 0x6A >> 1);
    expected.pc = 2;
    assert_eq!(registers, expected);
    assert_eq!(argument_type, ImmediateArgumentType::None);
}
