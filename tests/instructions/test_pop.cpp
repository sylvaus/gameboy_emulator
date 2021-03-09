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
using emulator::instructions::REGISTER_16_BITS_VARIANT_VALUE_SETTER_MAP;
using emulator::instructions::REGISTER_16_BITS_VARIANT_VALUE_NAME_MAP;
using emulator::instructions::NameMapPrinter;
using emulator::instructions::fixtures::InstructionTestFixture;

namespace gen = emulator::generated;

namespace
{
    class PopTestFixture : public InstructionTestFixture, public ::testing::WithParamInterface<uint16_t>
    {
    };

    TEST_P(PopTestFixture, POP)
    {
        // Test from Chapter 4: page 101
        uint16_t register_index = GetParam();
        registers.F = emulator::memory::make_flag(true, true, false, false);
        registers.SP = 0xFFFC;
        REGISTER_16_BITS_VARIANT_VALUE_SETTER_MAP.at(register_index)(registers, 0);
        expected_registers.F = emulator::memory::make_flag(true, true, false, false);
        expected_registers.SP = 0xFFFE;
        REGISTER_16_BITS_VARIANT_VALUE_SETTER_MAP.at(register_index)(expected_registers, 0x3C5F);
        EXPECT_CALL (controller, get(0xFFFC)).Times(1).WillOnce(::testing::Return(0x5F));
        EXPECT_CALL (controller, get(0xFFFD)).Times(1).WillOnce(::testing::Return(0x3C));
        set_expected_pc_increase(1);

        uint16_t instruction_index = (register_index << 4) + 0b1100'0001;
        const auto cycle = gen::INSTRUCTION_FUNCTIONS[instruction_index](arguments, registers, controller);

        EXPECT_EQ(12, cycle);
    }

    INSTANTIATE_TEST_SUITE_P(
        RetConditionalTest, PopTestFixture, REGISTER_FLAG_VALUES, NameMapPrinter(REGISTER_16_BITS_VARIANT_VALUE_NAME_MAP)
    );
}

