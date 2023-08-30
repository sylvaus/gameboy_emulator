/// Tests for the POP instruction
/// The POP instruction is described in chapter 4: page 101 of https://ia803208.us.archive.org/9/items/GameBoyProgManVer1.1/GameBoyProgManVer1.1.pdf
use crate::generated::instructions::tests::fixture::{FakeMemory, PUSH_POP_REGISTERS_16_BITS};
use crate::generated::instructions::{get_instruction, ImmediateArgumentType};
use crate::memory::argument::Argument;
use crate::memory::registers::Registers;
use crate::memory::Memory;

#[test]
fn test_pop_16bits() {
    for register_info in PUSH_POP_REGISTERS_16_BITS {
        let opcode = (register_info.index << 4) + 0b11000001;
        let mut register = Registers::new();
        let mut memory = FakeMemory::new();
        let argument = Argument::new_empty();
        register.sp = 0x1245;
        memory.write(0x1245, 0xD1);
        memory.write(0x1246, 0x5C);
        let mut expected = register.clone();

        let (instruction, argument_type) = get_instruction(opcode);
        let nb_cycle = instruction(&mut register, &mut memory, &argument);

        assert_eq!(nb_cycle, 12);

        (register_info.setter)(&mut expected, 0x5CD1);
        expected.sp = 0x1245 + 2;
        expected.pc = 1;
        assert_eq!(register, expected);
        assert_eq!(argument_type, ImmediateArgumentType::None);
    }
}
