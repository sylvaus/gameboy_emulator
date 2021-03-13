/*
Testing all the inc functions

The pages referenced in this file are pointing to [GameBoyProgManVer1.1.pdf](https://ia803208.us.archive.org/9/items/GameBoyProgManVer1.1/GameBoyProgManVer1.1.pdf)
 */

#include <unordered_map>

#include "gtest/gtest.h"

#include "emulator/memory/registers.h"
#include "emulator/generated/instructions.h"

#include "fixtures/instruction.h"
#include "constants.h"

using emulator::generated::Arguments;
using emulator::memory::Registers;
using emulator::instructions::NameMapPrinter;
using emulator::instructions::fixtures::InstructionTestFixture;

namespace gen = emulator::generated;

namespace
{
    const auto VALUES = ::testing::Values(0, 1, 2, 3, 4, 5, 6, 7);
    const std::unordered_map<uint16_t, uint16_t> EXPECTED_PC_MAP = {
        {0, 0x00},
        {1, 0x08},
        {2, 0x10},
        {3, 0x18},
        {4, 0x20},
        {5, 0x28},
        {6, 0x30},
        {7, 0x38},
    };

    class RstTestFixture : public InstructionTestFixture, public ::testing::WithParamInterface<uint16_t>
    {
    };

    TEST_P(RstTestFixture, Rst)
    {
        // Test from Chapter 4: page 121
        uint16_t value_index = GetParam();
        registers.F = emulator::memory::make_flag(true, true, false, false);
        registers.SP = 0xFFFE;
        registers.PC = 0x8000;
        expected_registers.F = emulator::memory::make_flag(true, true, false, false);
        expected_registers.SP = 0xFFFC;
        expected_registers.PC = EXPECTED_PC_MAP.at(value_index);
        EXPECT_CALL (controller, set(0xFFFD, 0x80)).Times(1);
        EXPECT_CALL (controller, set(0xFFFC, 0x01)).Times(1);

        uint16_t instruction_index = (value_index << 3) + 0b1100'0111;
        const auto cycle = gen::INSTRUCTION_FUNCTIONS[instruction_index](arguments, registers, controller);

        EXPECT_EQ(16, cycle);
    }

    INSTANTIATE_TEST_SUITE_P(RstConditionalTest, RstTestFixture, VALUES);
}

