/*
Testing all the inc functions

The pages referenced in this file are pointing to [GameBoyProgManVer1.1.pdf](https://ia803208.us.archive.org/9/items/GameBoyProgManVer1.1/GameBoyProgManVer1.1.pdf)
 */

#include <string>

#include "gtest/gtest.h"

#include "emulator/memory/registers.h"
#include "emulator/generated/instructions.h"

#include "fixtures/instruction.h"
#include "constants.h"

using emulator::generated::Arguments;
using emulator::memory::Registers;
using emulator::instructions::fixtures::InstructionTestFixture;

namespace gen = emulator::generated;

namespace
{
    struct DaaParameters
    {
        uint8_t a_before_execution;
        uint8_t a_after_execution;
        bool carry_before_execution;
        bool carry_after_execution;
        bool half_carry_flag;
        bool add_sub_flag;
        std::string name;
    };

    std::string print_name(const testing::TestParamInfo<DaaParameters>& info)
    {
        return info.param.name;
    }

    class DaaTestFixture: public InstructionTestFixture, public ::testing::WithParamInterface<DaaParameters> {};

    TEST_P(DaaTestFixture, DAA)
    {
        // Test from Chapter 4: page 122
        const auto params = GetParam();
        registers.A = params.a_before_execution;
        registers.F = emulator::memory::make_flag(
            params.a_before_execution == 0,
            params.add_sub_flag,
            params.half_carry_flag,
            params.carry_before_execution
        );
        expected_registers.A = params.a_after_execution;
        expected_registers.F = emulator::memory::make_flag(
            params.a_after_execution == 0,
            params.add_sub_flag,
            false,
            params.carry_after_execution
        );

        set_expected_pc_increase(1);

        const auto cycle = gen::INSTRUCTION_FUNCTIONS[0x27](arguments, registers, controller);

        EXPECT_EQ(4, cycle);
    }

    const auto values = ::testing::Values(
        DaaParameters{0x85, 0x85, false, false, false, false, "NoChangeNeededBcdAdd"},
        DaaParameters{0x5C, 0x5C + 0x06, false, false, false, false, "LowerBitsOverflowBcdAdd"},
        DaaParameters{0x73, 0x73 + 0x06, false, false, true, false, "HalfCarryBcdAdd"},
        DaaParameters{0xA7, static_cast<uint8_t>(0xA7 + 0x60), false, true, false, false, "UpperBitsOverflowBcdAdd"},
        DaaParameters{0x9C, static_cast<uint8_t>(0x9C + 0x66), false, true, false, false, "UpperAndLowerBitsOverflowBcdAdd"},
        DaaParameters{0xF0, static_cast<uint8_t>(0xF0 + 0x66), false, true, true, false, "UpperBitsOverflowAndHalfCarryBcdAdd"},
        DaaParameters{0x25, 0x25 + 0x60, true, true, false, false, "CarryBcdAdd"},
        DaaParameters{0x1E, 0x1E + 0x66, true, true, false, false, "LowerBitsOverflowAndCarryBcdAdd"},
        DaaParameters{0x30, 0x30 + 0x66, true, true, true, false, "HalfCarryAndCarryBcdAdd"},
        DaaParameters{0x49, 0x49, false, false, false, true, "NoChangeNeededBcdSub"},
        DaaParameters{0x86, static_cast<uint8_t>(0x86 + 0xFA), false, false, true, true, "HalfCarryBcdSub"},
        DaaParameters{0xD5, static_cast<uint8_t>(0xD5 + 0xA0), true, true, false, true, "CarryBcdSub"},
        DaaParameters{0x94, static_cast<uint8_t>(0x94 + 0x9A), true, true, true, true, "HalfCarryAndCarryBcdSub"}
    );

    INSTANTIATE_TEST_SUITE_P(DaaTest, DaaTestFixture, values, &print_name);
}
