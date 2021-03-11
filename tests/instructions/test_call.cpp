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
    class CallTestFixture : public InstructionTestFixture, public ::testing::WithParamInterface<uint16_t>
    {
    };

    TEST_P(CallTestFixture, CallConditionalTrue)
    {
        // Test from Chapter 4: page 118
        uint16_t flag_index = GetParam();
        registers.PC = 0x8000;
        registers.SP = 0xFFFE;
        arguments.uint16 = 0x1234;
        expected_registers.SP = 0xFFFC;
        EXPECT_CALL (controller, set(0xFFFD, 0x80)).Times(1);
        EXPECT_CALL (controller, set(0xFFFC, 0x03)).Times(1);
        REGISTER_FLAG_VALUE_SETTER_MAP.at(flag_index)(registers, true);
        REGISTER_FLAG_VALUE_SETTER_MAP.at(flag_index)(expected_registers, true);
        set_expected_pc(0x1234);

        uint16_t instruction_index = (flag_index << 3) + 0b1100'0100;
        const auto cycle = gen::INSTRUCTION_FUNCTIONS[instruction_index](arguments, registers, controller);

        EXPECT_EQ(24, cycle);
    }

    TEST_P(CallTestFixture, CallConditionalFalse)
    {
        // Test from Chapter 4: page 118
        uint16_t flag_index = GetParam();
        registers.PC = 0x8000;
        registers.SP = 0xFFFE;
        arguments.uint16 = 0x1234;
        expected_registers.SP = 0xFFFE;
        EXPECT_CALL (controller, set(0xFFFD, 0x80)).Times(0);
        EXPECT_CALL (controller, set(0xFFFC, 0x03)).Times(0);
        REGISTER_FLAG_VALUE_SETTER_MAP.at(flag_index)(registers, false);
        REGISTER_FLAG_VALUE_SETTER_MAP.at(flag_index)(expected_registers, false);
        set_expected_pc_increase(3);

        uint16_t instruction_index = (flag_index << 3) + 0b1100'0100;
        const auto cycle = gen::INSTRUCTION_FUNCTIONS[instruction_index](arguments, registers, controller);

        EXPECT_EQ(12, cycle);
    }

    INSTANTIATE_TEST_SUITE_P(
        CallConditionalTest, CallTestFixture, REGISTER_FLAG_VALUES, NameMapPrinter(REGISTER_FLAG_VALUE_NAME_MAP)
    );

    TEST_F(CallTestFixture, CallUnconditionalImmediate)
    {
        // Test from Chapter 4: page 118
        registers.PC = 0x8000;
        registers.SP = 0xFFFE;
        arguments.uint16 = 0x1234;
        expected_registers.SP = 0xFFFC;
        EXPECT_CALL (controller, set(0xFFFD, 0x80)).Times(1);
        EXPECT_CALL (controller, set(0xFFFC, 0x03)).Times(1);
        set_expected_pc(0x1234);

        const auto cycle = gen::INSTRUCTION_FUNCTIONS[0b1100'1101](arguments, registers, controller);

        EXPECT_EQ(24, cycle);
    }
}
