/// Tests for the PUSH instruction
/// The PUSH instruction is described in chapter 4: page 101 of https://ia803208.us.archive.org/9/items/GameBoyProgManVer1.1/GameBoyProgManVer1.1.pdf
use crate::generated::instructions::tests::fixture::{
    FakeMemory, PUSH_POP_REGISTERS_16_BITS
};
use crate::generated::instructions::{get_instruction, ImmediateArgumentType};
use crate::memory::argument::Argument;
use crate::memory::registers::Registers;

#[test]
fn test_push_16bits() {
    for register_info in PUSH_POP_REGISTERS_16_BITS {
        let opcode = (register_info.index << 4) + 0b11000101;
        let mut register = Registers::new();
        let mut memory = FakeMemory::new();
        let argument = Argument::new_empty();
        register.sp = 0x1245;
        (register_info.setter)(&mut register, 0x5CD1);
        let mut expected = register.clone();

        let (instruction, argument_type) = get_instruction(opcode);
        let nb_cycle = instruction(&mut register, &mut memory, &argument);

        assert_eq!(nb_cycle, 16);

        memory.assert_value(0x1244, 0x5C);
        memory.assert_value(0x1243, 0xD1);
        expected.sp = 0x1245 - 2;
        expected.pc = 1;
        assert_eq!(register, expected);
        assert_eq!(argument_type, ImmediateArgumentType::None);
    }
}