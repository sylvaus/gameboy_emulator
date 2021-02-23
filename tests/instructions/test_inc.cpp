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
using emulator::instructions::REGISTER_16_BITS_VALUE_SETTER_MAP;
using emulator::instructions::fixtures::InstructionTestFixture;

namespace gen = emulator::generated;

namespace
{
    class Inc16BitsTestFixture: public InstructionTestFixture, public ::testing::WithParamInterface<int> {};

    TEST_P(Inc16BitsTestFixture, INC16Bits)
    {
        uint16_t register_index = GetParam();
        uint16_t instruction_index = (register_index << 4) + 0b11;
        // Test from Chapter 4: page 107
        REGISTER_16_BITS_VALUE_SETTER_MAP.at(register_index)(registers, 0xA8F1);
        REGISTER_16_BITS_VALUE_SETTER_MAP.at(register_index)(expected_registers, 0xA8F2);
        set_expected_pc_increase(1);

        const auto cycle = gen::INSTRUCTION_FUNCTIONS[instruction_index](arguments, registers, controller);

        EXPECT_EQ(8, cycle);
    }

    INSTANTIATE_TEST_CASE_P(Inc16BitsTest, Inc16BitsTestFixture, ::testing::Values(0, 1, 2, 3));
}
