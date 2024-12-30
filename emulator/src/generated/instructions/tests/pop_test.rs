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
        // 0xf1 POP AF
        // Only the top 4 bits should be set for the flags register:
        // https://forums.nesdev.org/viewtopic.php?p=147669&sid=968b67f5e97f5c4e8419d9267a7ac9ed#p147669
        if opcode == 0xF1 {
            expected.flags &= 0xF0;
        }
        expected.sp = 0x1245 + 2;
        expected.pc = 1;
        assert_eq!(register, expected);
        assert_eq!(argument_type, ImmediateArgumentType::None);
    }
}
