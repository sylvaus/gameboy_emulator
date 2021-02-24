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
using emulator::instructions::REGISTER_8_BITS_VALUES;
using emulator::instructions::REGISTER_16_BITS_VALUES;
using emulator::instructions::REGISTER_8_BITS_VALUE_SETTER_MAP;
using emulator::instructions::REGISTER_8_BITS_VALUE_NAME_MAP;
using emulator::instructions::REGISTER_16_BITS_VALUE_SETTER_MAP;
using emulator::instructions::REGISTER_16_BITS_VALUE_NAME_MAP;
using emulator::instructions::NameMapPrinter;
using emulator::instructions::fixtures::InstructionTestFixture;

namespace gen = emulator::generated;

namespace
{
    class Inc16BitsTestFixture: public InstructionTestFixture, public ::testing::WithParamInterface<uint16_t> {};

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

    INSTANTIATE_TEST_SUITE_P(
        Inc16BitsTest, Inc16BitsTestFixture, REGISTER_16_BITS_VALUES, NameMapPrinter(REGISTER_16_BITS_VALUE_NAME_MAP)
    );

    class Inc8BitsTestFixture: public InstructionTestFixture, public ::testing::WithParamInterface<uint16_t> {};

    TEST_P(Inc8BitsTestFixture, INC8Bits)
    {
        uint16_t register_index = GetParam();
        uint16_t instruction_index = (register_index << 3) + 0b100;
        // Test from Chapter 4: page 107
        REGISTER_8_BITS_VALUE_SETTER_MAP.at(register_index)(registers, 0xA8);
        registers.F = 0;
        REGISTER_8_BITS_VALUE_SETTER_MAP.at(register_index)(expected_registers, 0xA9);
        expected_registers.F = 0;
        set_expected_pc_increase(1);

        const auto cycle = gen::INSTRUCTION_FUNCTIONS[instruction_index](arguments, registers, controller);

        EXPECT_EQ(4, cycle);
    }

    INSTANTIATE_TEST_SUITE_P(
        Inc8BitsTest, Inc8BitsTestFixture, REGISTER_8_BITS_VALUES, NameMapPrinter(REGISTER_8_BITS_VALUE_NAME_MAP)
    );
}
