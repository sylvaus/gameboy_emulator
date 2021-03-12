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
    class PushTestFixture : public InstructionTestFixture, public ::testing::WithParamInterface<uint16_t>
    {
    };

    TEST_P(PushTestFixture, Push)
    {
        // Test from Chapter 4: page 100
        uint16_t register_index = GetParam();
        registers.F = emulator::memory::make_flag(true, true, false, false);
        registers.SP = 0xFFFE;
        REGISTER_16_BITS_VARIANT_VALUE_SETTER_MAP.at(register_index)(registers, 0x3C5F);
        expected_registers.F = emulator::memory::make_flag(true, true, false, false);
        expected_registers.SP = 0xFFFC;
        REGISTER_16_BITS_VARIANT_VALUE_SETTER_MAP.at(register_index)(expected_registers, 0x3C5F);
        EXPECT_CALL (controller, set(0xFFFD, 0x3C)).Times(1);
        EXPECT_CALL (controller, set(0xFFFC, 0x5F)).Times(1);
        set_expected_pc_increase(1);

        uint16_t instruction_index = (register_index << 4) + 0b1100'0101;
        const auto cycle = gen::INSTRUCTION_FUNCTIONS[instruction_index](arguments, registers, controller);

        EXPECT_EQ(16, cycle);
    }

    INSTANTIATE_TEST_SUITE_P(
        RetConditionalTest, PushTestFixture, REGISTER_FLAG_VALUES, NameMapPrinter(REGISTER_16_BITS_VARIANT_VALUE_NAME_MAP)
    );
}

