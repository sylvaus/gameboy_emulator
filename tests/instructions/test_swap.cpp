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
using emulator::instructions::REGISTER_8_BITS_VALUE_SETTER_MAP;
using emulator::instructions::REGISTER_8_BITS_VALUE_NAME_MAP;
using emulator::instructions::NameMapPrinter;
using emulator::instructions::fixtures::InstructionTestFixture;

namespace gen = emulator::generated;

namespace
{
    class SwapTestFixture: public InstructionTestFixture, public ::testing::WithParamInterface<uint16_t> {};

    TEST_P(SwapTestFixture, SWAP)
    {
        // Test from Chapter 4: page 110
        uint16_t register_index = GetParam();
        uint16_t instruction_index = register_index + 0b1'0011'0000;
        REGISTER_8_BITS_VALUE_SETTER_MAP.at(register_index)(registers, 0b0110'1100);
        registers.F = emulator::memory::make_flag(true, true, true, true);
        REGISTER_8_BITS_VALUE_SETTER_MAP.at(register_index)(expected_registers, 0b1100'0110);
        expected_registers.F = emulator::memory::make_flag(false, false, false, false);

        set_expected_pc_increase(2);

        const auto cycle = gen::INSTRUCTION_FUNCTIONS[instruction_index](arguments, registers, controller);

        EXPECT_EQ(8, cycle);
    }

    TEST_P(SwapTestFixture, SWAPZero)
    {
        // Test from Chapter 4: page 110
        uint16_t register_index = GetParam();
        uint16_t instruction_index = register_index + 0b1'0011'0000;
        REGISTER_8_BITS_VALUE_SETTER_MAP.at(register_index)(registers, 0x0);
        registers.F = emulator::memory::make_flag(false, true, true, true);
        REGISTER_8_BITS_VALUE_SETTER_MAP.at(register_index)(expected_registers, 0x0);
        expected_registers.F = emulator::memory::make_flag(true, false, false, false);

        set_expected_pc_increase(2);

        const auto cycle = gen::INSTRUCTION_FUNCTIONS[instruction_index](arguments, registers, controller);

        EXPECT_EQ(8, cycle);
    }

    INSTANTIATE_TEST_SUITE_P(
        SwapTest, SwapTestFixture, REGISTER_8_BITS_VALUES, NameMapPrinter(REGISTER_8_BITS_VALUE_NAME_MAP)
    );


    TEST_F(SwapTestFixture, SWAPAddress)
    {
        // Test from Chapter 4: page 110
        registers.set_HL(0xD1C7);
        EXPECT_CALL (controller, get(0xD1C7)).Times(1).WillOnce(::testing::Return(0b1110'0110));
        registers.F = emulator::memory::make_flag(true, true, true, true);
        expected_registers.set_HL(0xD1C7);
        EXPECT_CALL (controller, set(0xD1C7, 0b0110'1110)).Times(1);
        expected_registers.F = emulator::memory::make_flag(false, false, false, false);

        set_expected_pc_increase(2);

        const auto cycle = gen::INSTRUCTION_FUNCTIONS[0b1'0011'0110](arguments, registers, controller);

        EXPECT_EQ(16, cycle);
    }

    TEST_F(SwapTestFixture, SWAPAddressZero)
    {
        // Test from Chapter 4: page 110
        registers.set_HL(0xD1C7);
        EXPECT_CALL (controller, get(0xD1C7)).Times(1).WillOnce(::testing::Return(0x0));
        registers.F = emulator::memory::make_flag(false, true, true, true);
        expected_registers.set_HL(0xD1C7);
        EXPECT_CALL (controller, set(0xD1C7, 0x0)).Times(1);
        expected_registers.F = emulator::memory::make_flag(true, false, false, false);

        set_expected_pc_increase(2);

        const auto cycle = gen::INSTRUCTION_FUNCTIONS[0b1'0011'0110](arguments, registers, controller);

        EXPECT_EQ(16, cycle);
    }
}
