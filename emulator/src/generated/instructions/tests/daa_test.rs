/// Tests for the DAA instruction
/// The DAA instruction is described in chapter 4: page 121 of https://ia803208.us.archive.org/9/items/GameBoyProgManVer1.1/GameBoyProgManVer1.1.pdf
use crate::generated::instructions::tests::fixture::FakeMemory;
use crate::generated::instructions::{get_instruction, ImmediateArgumentType};
use crate::memory::argument::Argument;
use crate::memory::registers::Registers;

struct DAATestCase {
    name: &'static str,
    add_sub_flag: bool,
    register_a_before: u8,
    carry_flag_before: bool,
    half_carry_flag_before: bool,
    register_a_after: u8,
    carry_flag_after: bool,
}

const DAA_TEST_CASES: &[DAATestCase] = &[
    DAATestCase {
        name: "NoChangeNeededBcdAdd",
        add_sub_flag: false,
        register_a_before: 0x85,
        carry_flag_before: false,
        half_carry_flag_before: false,
        register_a_after: 0x85,
        carry_flag_after: false,
    },
    DAATestCase {
        name: "ZeroBcdAdd",
        add_sub_flag: false,
        register_a_before: 0,
        carry_flag_before: false,
        half_carry_flag_before: false,
        register_a_after: 0,
        carry_flag_after: false,
    },
    DAATestCase {
        name: "LowerBitsOverflowBcdAdd",
        add_sub_flag: false,
        register_a_before: 0x5C,
        carry_flag_before: false,
        half_carry_flag_before: false,
        register_a_after: 0x5C + 0x06,
        carry_flag_after: false,
    },
    DAATestCase {
        name: "HalfCarryBcdAdd",
        add_sub_flag: false,
        register_a_before: 0x73,
        carry_flag_before: false,
        half_carry_flag_before: true,
        register_a_after: 0x73 + 0x06,
        carry_flag_after: false,
    },
    DAATestCase {
        name: "UpperBitsOverflowBcdAdd",
        add_sub_flag: false,
        register_a_before: 0xA7,
        carry_flag_before: false,
        half_carry_flag_before: false,
        register_a_after: 0xA7u8.wrapping_add(0x60),
        carry_flag_after: true,
    },
    DAATestCase {
        name: "UpperAndLowerBitsOverflowBcdAdd",
        add_sub_flag: false,
        register_a_before: 0x9C,
        carry_flag_before: false,
        half_carry_flag_before: false,
        register_a_after: 0x9Cu8.wrapping_add(0x66),
        carry_flag_after: true,
    },
    DAATestCase {
        name: "UpperBitsOverflowAndHalfCarryBcdAdd",
        add_sub_flag: false,
        register_a_before: 0xF0,
        carry_flag_before: false,
        half_carry_flag_before: true,
        register_a_after: 0xF0u8.wrapping_add(0x66),
        carry_flag_after: true,
    },
    DAATestCase {
        name: "CarryBcdAdd",
        add_sub_flag: false,
        register_a_before: 0x25,
        carry_flag_before: true,
        half_carry_flag_before: false,
        register_a_after: 0x25 + 0x60,
        carry_flag_after: true,
    },
    DAATestCase {
        name: "LowerBitsOverflowAndCarryBcdAdd",
        add_sub_flag: false,
        register_a_before: 0x1E,
        carry_flag_before: true,
        half_carry_flag_before: false,
        register_a_after: 0x1E + 0x66,
        carry_flag_after: true,
    },
    DAATestCase {
        name: "HalfCarryAndCarryBcdAdd",
        add_sub_flag: false,
        register_a_before: 0x30,
        carry_flag_before: true,
        half_carry_flag_before: true,
        register_a_after: 0x30 + 0x66,
        carry_flag_after: true,
    },
    DAATestCase {
        name: "NoChangeNeededBcdSub",
        add_sub_flag: true,
        register_a_before: 0x49,
        carry_flag_before: false,
        half_carry_flag_before: false,
        register_a_after: 0x49,
        carry_flag_after: false,
    },
    DAATestCase {
        name: "HalfCarryBcdSub",
        add_sub_flag: true,
        register_a_before: 0x86,
        carry_flag_before: false,
        half_carry_flag_before: true,
        register_a_after: 0x86u8.wrapping_add(0xFA),
        carry_flag_after: false,
    },
    DAATestCase {
        name: "CarryBcdSub",
        add_sub_flag: true,
        register_a_before: 0xD5,
        carry_flag_before: true,
        half_carry_flag_before: false,
        register_a_after: 0xD5u8.wrapping_add(0xA0),
        carry_flag_after: true,
    },
    DAATestCase {
        name: "HalfCarryAndCarryBcdSub",
        add_sub_flag: true,
        register_a_before: 0x94,
        carry_flag_before: true,
        half_carry_flag_before: true,
        register_a_after: 0x94u8.wrapping_add(0x9A),
        carry_flag_after: true,
    },
    DAATestCase {
        name: "ZeroBcdSub",
        add_sub_flag: true,
        register_a_before: 0,
        carry_flag_before: false,
        half_carry_flag_before: false,
        register_a_after: 0,
        carry_flag_after: false,
    },
];

#[test]
fn test_daa() {
    for test_case in DAA_TEST_CASES {
        let opcode = 0x27;
        let mut registers = Registers::new();
        let mut memory = FakeMemory::new();
        let argument = Argument::new_empty();
        registers.a = test_case.register_a_before;
        registers.set_add_sub_flag(test_case.add_sub_flag);
        registers.set_carry_flag(test_case.carry_flag_before);
        registers.set_half_carry_flag(test_case.half_carry_flag_before);
        registers.set_zero_flag(test_case.register_a_before == 0);
        let mut expected = registers.clone();

        let (instruction, argument_type) = get_instruction(opcode);
        let nb_cycle = instruction(&mut registers, &mut memory, &argument);

        assert_eq!(nb_cycle, 4);

        expected.pc = 1;
        expected.a = test_case.register_a_after;
        expected.set_carry_flag(test_case.carry_flag_after);
        expected.set_half_carry_flag(false);
        expected.set_zero_flag(test_case.register_a_after == 0);
        assert_eq!(registers, expected);
        assert_eq!(argument_type, ImmediateArgumentType::None);
    }
}
