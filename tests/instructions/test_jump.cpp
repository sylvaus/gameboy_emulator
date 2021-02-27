/*
Testing all the inc functions

The pages referenced in this file are pointing to [GameBoyProgManVer1.1.pdf](https://ia803208.us.archive.org/9/items/GameBoyProgManVer1.1/GameBoyProgManVer1.1.pdf)
 */

#include "gtest/gtest.h"

#include "emulator/memory/registers.h"
#include "emulator/generated/instructions.h"

#include "fixtures/instruction.h"
#include "constants.h"

using emulator::generated::Arguments;
using emulator::memory::Registers;
using emulator::instructions::REGISTER_FLAG_VALUES;
using emulator::instructions::REGISTER_FLAG_VALUE_SETTER_MAP;
using emulator::instructions::REGISTER_FLAG_VALUE_NAME_MAP;
using emulator::instructions::NameMapPrinter;
using emulator::instructions::fixtures::InstructionTestFixture;

namespace gen = emulator::generated;

namespace
{
    class JumpRelativeTestFixture: public InstructionTestFixture {};

    TEST_F(JumpRelativeTestFixture, JRUnconditionalPositive)
    {
        // Test from Chapter 4: page 117
        uint16_t instruction_index = 0b11000;
        arguments.int8 = 0x5C;
        set_expected_pc_increase(2 + 0x5C);

        const auto cycle = gen::INSTRUCTION_FUNCTIONS[instruction_index](arguments, registers, controller);

        EXPECT_EQ(12, cycle);
    }

    TEST_F(JumpRelativeTestFixture, JRUnconditionalNegative)
    {
        // Test from Chapter 4: page 117
        uint16_t instruction_index = 0b11000;
        arguments.int8 = -0x6D;
        set_expected_pc_increase(2 - 0x6D);

        const auto cycle = gen::INSTRUCTION_FUNCTIONS[instruction_index](arguments, registers, controller);

        EXPECT_EQ(12, cycle);
    }

    class JumpRelativeConditionalTestFixture: public InstructionTestFixture, public ::testing::WithParamInterface<uint16_t> {};

    TEST_P(JumpRelativeConditionalTestFixture, JRConditionalTrue)
    {
        // Test from Chapter 4: page 117
        uint16_t flag_index = GetParam();
        uint16_t instruction_index = (flag_index << 3) + 0b10'0000;
        REGISTER_FLAG_VALUE_SETTER_MAP.at(flag_index)(registers, true);
        REGISTER_FLAG_VALUE_SETTER_MAP.at(flag_index)(expected_registers, true);
        arguments.int8 = -0x05;
        set_expected_pc_increase(2 - 0x05);

        const auto cycle = gen::INSTRUCTION_FUNCTIONS[instruction_index](arguments, registers, controller);

        EXPECT_EQ(12, cycle);
    }

    TEST_P(JumpRelativeConditionalTestFixture, JumpRelativeConditionalFalse)
    {
        // Test from Chapter 4: page 117
        uint16_t flag_index = GetParam();
        uint16_t instruction_index = (flag_index << 3) + 0b10'0000;
        REGISTER_FLAG_VALUE_SETTER_MAP.at(flag_index)(registers, false);
        REGISTER_FLAG_VALUE_SETTER_MAP.at(flag_index)(expected_registers, false);
        arguments.int8 = -0x05;
        set_expected_pc_increase(2);

        const auto cycle = gen::INSTRUCTION_FUNCTIONS[instruction_index](arguments, registers, controller);

        EXPECT_EQ(8, cycle);
    }

    INSTANTIATE_TEST_SUITE_P(
        JumpRelativeTest, JumpRelativeConditionalTestFixture, REGISTER_FLAG_VALUES, NameMapPrinter(REGISTER_FLAG_VALUE_NAME_MAP)
    );
}
