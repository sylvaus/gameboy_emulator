/// Tests for the RST instruction
/// The RST instruction is described in chapter 4: page 121 of https://ia803208.us.archive.org/9/items/GameBoyProgManVer1.1/GameBoyProgManVer1.1.pdf
use crate::generated::instructions::tests::fixture::FakeMemory;
use crate::generated::instructions::{get_instruction, ImmediateArgumentType};
use crate::memory::argument::Argument;
use crate::memory::registers::Registers;

const RST_BASE_OPCODE: u16 = 0b11000111;

const OPERAND_PC_VALUES: &[(u16, u16)] = &[
    (0, 0x00),
    (1, 0x08),
    (2, 0x10),
    (3, 0x18),
    (4, 0x20),
    (5, 0x28),
    (6, 0x30),
    (7, 0x38),
];

#[test]
fn test_rst() {
    for (operand, pc_value) in OPERAND_PC_VALUES {
        let opcode = (operand << 3) + RST_BASE_OPCODE;
        let mut register = Registers::new();
        let mut memory = FakeMemory::new();
        let argument = Argument::new_i8(0x5C);
        register.sp = 0xFFFE;
        register.pc = 0x8000;
        let mut expected = register.clone();

        let (instruction, argument_type) = get_instruction(opcode);
        let nb_cycle = instruction(&mut register, &mut memory, &argument);

        assert_eq!(nb_cycle, 16);

        memory.assert_value(0xFFFD, 0x80);
        memory.assert_value(0xFFFC, 0x01);
        expected.sp = 0xFFFC;
        expected.pc = *pc_value;
        assert_eq!(register, expected);
        assert_eq!(argument_type, ImmediateArgumentType::None);
    }
}
