/// Tests for the LD instruction
/// The LD instruction is described in chapter 4: page 95-101 of https://ia803208.us.archive.org/9/items/GameBoyProgManVer1.1/GameBoyProgManVer1.1.pdf
use crate::generated::instructions::tests::fixture::{
    FakeMemory, REGISTERS_16_BITS, REGISTERS_8_BITS, REGISTER_H, REGISTER_L,
};
use crate::generated::instructions::{get_instruction, ImmediateArgumentType};
use crate::memory::argument::Argument;
use crate::memory::registers::Registers;
use crate::memory::Memory;

#[test]
fn test_ld_8bits_register_to_8bits_register() {
    for input_register in REGISTERS_8_BITS {
        for output_register in REGISTERS_8_BITS {
            let opcode = (output_register.index << 3) + input_register.index + 0b1000000;
            let mut register = Registers::new();
            let mut memory = FakeMemory::new();
            let argument = Argument::new_empty();
            (input_register.setter)(&mut register, 0x45);
            let mut expected = register.clone();

            let (instruction, argument_type) = get_instruction(opcode);
            let nb_cycle = instruction(&mut register, &mut memory, &argument);

            assert_eq!(nb_cycle, 4);

            (output_register.setter)(&mut expected, 0x45);
            expected.pc = 1;
            assert_eq!(register, expected);
            assert_eq!(argument_type, ImmediateArgumentType::None);
        }
    }
}

#[test]
fn test_ld_immediate_to_8bits_register() {
    for register_info in REGISTERS_8_BITS {
        let opcode = (register_info.index << 3) + 0b110;
        let mut registers = Registers::new();
        let mut memory = FakeMemory::new();
        let argument = Argument::new_u8(0x4A);
        let mut expected = registers.clone();

        let (instruction, argument_type) = get_instruction(opcode);
        let nb_cycle = instruction(&mut registers, &mut memory, &argument);

        assert_eq!(nb_cycle, 8);

        (register_info.setter)(&mut expected, 0x4A);
        expected.pc = 2;
        assert_eq!(registers, expected);
        assert_eq!(argument_type, ImmediateArgumentType::Unsigned8Bits);
    }
}

#[test]
fn test_ld_hl_address_to_8bits_register() {
    for register_info in REGISTERS_8_BITS {
        let opcode = (register_info.index << 3) + 0b1000110;
        let mut registers = Registers::new();
        let mut memory = FakeMemory::new();
        let argument = Argument::new_empty();
        registers.set_hl(0x12DF);
        memory.write(0x12DF, 0xE5);
        let mut expected = registers.clone();

        let (instruction, argument_type) = get_instruction(opcode);
        let nb_cycle = instruction(&mut registers, &mut memory, &argument);

        assert_eq!(nb_cycle, 8);

        (register_info.setter)(&mut expected, 0xE5);
        expected.pc = 1;
        assert_eq!(registers, expected);
        assert_eq!(argument_type, ImmediateArgumentType::None);
    }
}

#[test]
fn test_ld_8bits_register_to_hl_address() {
    for register_info in REGISTERS_8_BITS {
        let opcode = register_info.index + 0b1110000;
        let mut registers = Registers::new();
        let mut memory = FakeMemory::new();
        let argument = Argument::new_empty();
        (register_info.setter)(&mut registers, 0xE5);
        registers.set_hl(0x12DF);
        let mut expected = registers.clone();

        let (instruction, argument_type) = get_instruction(opcode);
        let nb_cycle = instruction(&mut registers, &mut memory, &argument);

        assert_eq!(nb_cycle, 8);
        if register_info == &REGISTER_H {
            memory.assert_value(0x12DF, 0x12);
        } else if register_info == &REGISTER_L {
            memory.assert_value(0x12DF, 0xDF);
        } else {
            memory.assert_value(0x12DF, 0xE5);
        }
        expected.pc = 1;
        assert_eq!(registers, expected);
        assert_eq!(argument_type, ImmediateArgumentType::None);
    }
}

#[test]
fn test_ld_immediate_to_hl_address() {
    let opcode = 0b00110110;
    let mut registers = Registers::new();
    let mut memory = FakeMemory::new();
    let argument = Argument::new_u8(0x6F);
    registers.set_hl(0x12DF);
    let mut expected = registers.clone();

    let (instruction, argument_type) = get_instruction(opcode);
    let nb_cycle = instruction(&mut registers, &mut memory, &argument);

    assert_eq!(nb_cycle, 12);
    memory.assert_value(0x12DF, 0x6F);
    expected.pc = 2;
    assert_eq!(registers, expected);
    assert_eq!(argument_type, ImmediateArgumentType::Unsigned8Bits);
}

#[test]
fn test_ld_bc_address_to_register_a() {
    let opcode = 0b00001010;
    let mut registers = Registers::new();
    let mut memory = FakeMemory::new();
    let argument = Argument::new_empty();
    registers.set_bc(0x13DF);
    memory.write(0x13DF, 0xAB);
    let mut expected = registers.clone();

    let (instruction, argument_type) = get_instruction(opcode);
    let nb_cycle = instruction(&mut registers, &mut memory, &argument);

    assert_eq!(nb_cycle, 8);

    expected.a = 0xAB;
    expected.pc = 1;
    assert_eq!(registers, expected);
    assert_eq!(argument_type, ImmediateArgumentType::None);
}

#[test]
fn test_ld_register_a_to_bc_address() {
    let opcode = 0b00000010;
    let mut registers = Registers::new();
    let mut memory = FakeMemory::new();
    let argument = Argument::new_empty();
    registers.set_bc(0x13DF);
    registers.a = 0xAB;
    let mut expected = registers.clone();

    let (instruction, argument_type) = get_instruction(opcode);
    let nb_cycle = instruction(&mut registers, &mut memory, &argument);

    assert_eq!(nb_cycle, 8);

    memory.assert_value(0x13DF, 0xAB);
    expected.pc = 1;
    assert_eq!(registers, expected);
    assert_eq!(argument_type, ImmediateArgumentType::None);
}

#[test]
fn test_ld_de_address_to_register_a() {
    let opcode = 0b00011010;
    let mut registers = Registers::new();
    let mut memory = FakeMemory::new();
    let argument = Argument::new_empty();
    registers.set_de(0x1ADF);
    memory.write(0x1ADF, 0xA1);
    let mut expected = registers.clone();

    let (instruction, argument_type) = get_instruction(opcode);
    let nb_cycle = instruction(&mut registers, &mut memory, &argument);

    assert_eq!(nb_cycle, 8);

    expected.a = 0xA1;
    expected.pc = 1;
    assert_eq!(registers, expected);
    assert_eq!(argument_type, ImmediateArgumentType::None);
}

#[test]
fn test_ld_register_a_to_de_address() {
    let opcode = 0b00010010;
    let mut registers = Registers::new();
    let mut memory = FakeMemory::new();
    let argument = Argument::new_empty();
    registers.set_de(0x1ADF);
    registers.a = 0xA1;
    let mut expected = registers.clone();

    let (instruction, argument_type) = get_instruction(opcode);
    let nb_cycle = instruction(&mut registers, &mut memory, &argument);

    assert_eq!(nb_cycle, 8);

    memory.assert_value(0x1ADF, 0xA1);
    expected.pc = 1;
    assert_eq!(registers, expected);
    assert_eq!(argument_type, ImmediateArgumentType::None);
}

#[test]
fn test_ld_c_address_to_register_a() {
    let opcode = 0b11110010;
    let mut registers = Registers::new();
    let mut memory = FakeMemory::new();
    let argument = Argument::new_empty();
    registers.c = 0xDF;
    memory.write(0xFF00 + 0xDF, 0xA1);
    let mut expected = registers.clone();

    let (instruction, argument_type) = get_instruction(opcode);
    let nb_cycle = instruction(&mut registers, &mut memory, &argument);

    assert_eq!(nb_cycle, 8);

    expected.a = 0xA1;
    expected.pc = 1;
    assert_eq!(registers, expected);
    assert_eq!(argument_type, ImmediateArgumentType::None);
}

#[test]
fn test_ld_register_a_to_c_address() {
    let opcode = 0b11100010;
    let mut registers = Registers::new();
    let mut memory = FakeMemory::new();
    let argument = Argument::new_empty();
    registers.c = 0xDF;
    registers.a = 0xA1;
    let mut expected = registers.clone();

    let (instruction, argument_type) = get_instruction(opcode);
    let nb_cycle = instruction(&mut registers, &mut memory, &argument);

    assert_eq!(nb_cycle, 8);

    memory.assert_value(0xFF00 + 0xDF, 0xA1);
    expected.pc = 1;
    assert_eq!(registers, expected);
    assert_eq!(argument_type, ImmediateArgumentType::None);
}

#[test]
fn test_ld_immediate_8bits_address_to_register_a() {
    let opcode = 0b11110000;
    let mut registers = Registers::new();
    let mut memory = FakeMemory::new();
    let argument = Argument::new_u8(0xDF);
    memory.write(0xFF00 + 0xDF, 0xA1);
    let mut expected = registers.clone();

    let (instruction, argument_type) = get_instruction(opcode);
    let nb_cycle = instruction(&mut registers, &mut memory, &argument);

    assert_eq!(nb_cycle, 12);

    expected.a = 0xA1;
    expected.pc = 2;
    assert_eq!(registers, expected);
    assert_eq!(argument_type, ImmediateArgumentType::Unsigned8Bits);
}

#[test]
fn test_ld_register_a_to_immediate_8bits_address() {
    let opcode = 0b11100000;
    let mut registers = Registers::new();
    let mut memory = FakeMemory::new();
    let argument = Argument::new_u8(0xDF);
    registers.a = 0xA1;
    let mut expected = registers.clone();

    let (instruction, argument_type) = get_instruction(opcode);
    let nb_cycle = instruction(&mut registers, &mut memory, &argument);

    assert_eq!(nb_cycle, 12);

    memory.assert_value(0xFF00 + 0xDF, 0xA1);
    expected.pc = 2;
    assert_eq!(registers, expected);
    assert_eq!(argument_type, ImmediateArgumentType::Unsigned8Bits);
}

#[test]
fn test_ld_immediate_16bits_address_to_register_a() {
    let opcode = 0b11111010;
    let mut registers = Registers::new();
    let mut memory = FakeMemory::new();
    let argument = Argument::new_u16(0x6598);
    memory.write(0x6598, 0xA1);
    let mut expected = registers.clone();

    let (instruction, argument_type) = get_instruction(opcode);
    let nb_cycle = instruction(&mut registers, &mut memory, &argument);

    assert_eq!(nb_cycle, 16);

    expected.a = 0xA1;
    expected.pc = 3;
    assert_eq!(registers, expected);
    assert_eq!(argument_type, ImmediateArgumentType::Unsigned16Bits);
}

#[test]
fn test_ld_register_a_to_immediate_16bits_address() {
    let opcode = 0b11101010;
    let mut registers = Registers::new();
    let mut memory = FakeMemory::new();
    let argument = Argument::new_u16(0x68A2);
    registers.a = 0xA1;
    let mut expected = registers.clone();

    let (instruction, argument_type) = get_instruction(opcode);
    let nb_cycle = instruction(&mut registers, &mut memory, &argument);

    assert_eq!(nb_cycle, 16);

    memory.assert_value(0x68A2, 0xA1);
    expected.pc = 3;
    assert_eq!(registers, expected);
    assert_eq!(argument_type, ImmediateArgumentType::Unsigned16Bits);
}

#[test]
fn test_ld_hl_address_to_register_a_increment() {
    let opcode = 0b00101010;
    let mut registers = Registers::new();
    let mut memory = FakeMemory::new();
    let argument = Argument::new_empty();
    registers.set_hl(0x12DF);
    memory.write(0x12DF, 0xE5);
    let mut expected = registers.clone();

    let (instruction, argument_type) = get_instruction(opcode);
    let nb_cycle = instruction(&mut registers, &mut memory, &argument);

    assert_eq!(nb_cycle, 8);

    expected.set_hl(expected.get_hl() + 1);
    expected.a = 0xE5;
    expected.pc = 1;
    assert_eq!(registers, expected);
    assert_eq!(argument_type, ImmediateArgumentType::None);
}

#[test]
fn test_ld_register_a_to_hl_address_increment() {
    let opcode = 0b00100010;
    let mut registers = Registers::new();
    let mut memory = FakeMemory::new();
    let argument = Argument::new_empty();
    registers.a = 0xE5;
    registers.set_hl(0x12DF);
    let mut expected = registers.clone();

    let (instruction, argument_type) = get_instruction(opcode);
    let nb_cycle = instruction(&mut registers, &mut memory, &argument);

    assert_eq!(nb_cycle, 8);

    memory.assert_value(0x12DF, 0xE5);
    expected.set_hl(expected.get_hl() + 1);
    expected.pc = 1;
    assert_eq!(registers, expected);
    assert_eq!(argument_type, ImmediateArgumentType::None);
}

#[test]
fn test_ld_hl_address_to_register_a_decrement() {
    let opcode = 0b00111010;
    let mut registers = Registers::new();
    let mut memory = FakeMemory::new();
    let argument = Argument::new_empty();
    registers.set_hl(0x12DF);
    memory.write(0x12DF, 0xE5);
    let mut expected = registers.clone();

    let (instruction, argument_type) = get_instruction(opcode);
    let nb_cycle = instruction(&mut registers, &mut memory, &argument);

    assert_eq!(nb_cycle, 8);

    expected.set_hl(expected.get_hl() - 1);
    expected.a = 0xE5;
    expected.pc = 1;
    assert_eq!(registers, expected);
    assert_eq!(argument_type, ImmediateArgumentType::None);
}

#[test]
fn test_ld_register_a_to_hl_address_decrement() {
    let opcode = 0b00110010;
    let mut registers = Registers::new();
    let mut memory = FakeMemory::new();
    let argument = Argument::new_empty();
    registers.a = 0xE5;
    registers.set_hl(0x12DF);
    let mut expected = registers.clone();

    let (instruction, argument_type) = get_instruction(opcode);
    let nb_cycle = instruction(&mut registers, &mut memory, &argument);

    assert_eq!(nb_cycle, 8);

    memory.assert_value(0x12DF, 0xE5);
    expected.set_hl(expected.get_hl() - 1);
    expected.pc = 1;
    assert_eq!(registers, expected);
    assert_eq!(argument_type, ImmediateArgumentType::None);
}

#[test]
fn test_ld_immediate_to_16bits_register() {
    for register_info in REGISTERS_16_BITS {
        let opcode = (register_info.index << 4) + 0b0001;
        let mut registers = Registers::new();
        let mut memory = FakeMemory::new();
        let argument = Argument::new_u16(0x3578);
        let mut expected = registers.clone();

        let (instruction, argument_type) = get_instruction(opcode);
        let nb_cycle = instruction(&mut registers, &mut memory, &argument);

        assert_eq!(nb_cycle, 12);

        (register_info.setter)(&mut expected, 0x3578);
        expected.pc = 3;
        assert_eq!(registers, expected);
        assert_eq!(argument_type, ImmediateArgumentType::Unsigned16Bits);
    }
}

#[test]
fn test_ld_register_hl_to_register_sp() {
    let opcode = 0b11111001;
    let mut registers = Registers::new();
    let mut memory = FakeMemory::new();
    let argument = Argument::new_empty();
    registers.a = 0xE5;
    registers.set_hl(0x12DF);
    let mut expected = registers.clone();

    let (instruction, argument_type) = get_instruction(opcode);
    let nb_cycle = instruction(&mut registers, &mut memory, &argument);

    assert_eq!(nb_cycle, 8);

    expected.sp = expected.get_hl();
    expected.pc = 1;
    assert_eq!(registers, expected);
    assert_eq!(argument_type, ImmediateArgumentType::None);
}

#[test]
fn test_ld_register_sp_to_register_hl() {
    let opcode = 0b11111000;
    let mut registers = Registers::new();
    let mut memory = FakeMemory::new();
    let argument = Argument::new_i8(0x2);
    registers.set_add_sub_flag(true);
    registers.set_zero_flag(true);
    registers.sp = 0x12DA;
    let mut expected = registers.clone();

    let (instruction, argument_type) = get_instruction(opcode);
    let nb_cycle = instruction(&mut registers, &mut memory, &argument);

    assert_eq!(nb_cycle, 12);

    expected.set_hl(0x12DA + 0x2);
    expected.set_add_sub_flag(false);
    expected.set_zero_flag(false);
    expected.pc = 2;
    assert_eq!(registers, expected);
    assert_eq!(argument_type, ImmediateArgumentType::Signed8Bits);
}

#[test]
fn test_ld_register_sp_to_register_hl_half_carry() {
    let opcode = 0b11111000;
    let mut registers = Registers::new();
    let mut memory = FakeMemory::new();
    let argument = Argument::new_i8(0x7F);
    registers.sp = 0x1FDA;
    let mut expected = registers.clone();

    let (instruction, argument_type) = get_instruction(opcode);
    let nb_cycle = instruction(&mut registers, &mut memory, &argument);

    assert_eq!(nb_cycle, 12);
    expected.set_half_carry_flag(true);
    expected.set_hl(0x1FDA + 0x7F);
    expected.pc = 2;
    assert_eq!(registers, expected);
    assert_eq!(argument_type, ImmediateArgumentType::Signed8Bits);
}

#[test]
fn test_ld_register_sp_to_register_hl_carry() {
    let opcode = 0b11111000;
    let mut registers = Registers::new();
    let mut memory = FakeMemory::new();
    let argument = Argument::new_i8(0x7F);
    registers.sp = 0xFFDA;
    let mut expected = registers.clone();

    let (instruction, argument_type) = get_instruction(opcode);
    let nb_cycle = instruction(&mut registers, &mut memory, &argument);

    assert_eq!(nb_cycle, 12);
    expected.set_half_carry_flag(true);
    expected.set_carry_flag(true);
    expected.set_hl(0xFFDAu16.wrapping_add(0x7F));
    expected.pc = 2;
    assert_eq!(registers, expected);
    assert_eq!(argument_type, ImmediateArgumentType::Signed8Bits);
}

#[test]
fn test_ld_register_sp_to_immediate_address() {
    let opcode = 0b00001000;
    let mut registers = Registers::new();
    let mut memory = FakeMemory::new();
    let argument = Argument::new_u16(0x1452);
    registers.sp = 0xF59A;
    let mut expected = registers.clone();

    let (instruction, argument_type) = get_instruction(opcode);
    let nb_cycle = instruction(&mut registers, &mut memory, &argument);

    assert_eq!(nb_cycle, 20);

    memory.assert_value(0x1452, 0x9A);
    memory.assert_value(0x1453, 0xF5);
    expected.pc = 3;
    assert_eq!(registers, expected);
    assert_eq!(argument_type, ImmediateArgumentType::Unsigned16Bits);
}
