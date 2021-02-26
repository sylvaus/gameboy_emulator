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
    class Dec16BitsTestFixture: public InstructionTestFixture, public ::testing::WithParamInterface<uint16_t> {};

    TEST_P(Dec16BitsTestFixture, DEC16Bits)
    {
        uint16_t register_index = GetParam();
        uint16_t instruction_index = (register_index << 4) + 0b1011;
        // Test from Chapter 4: page 108
        REGISTER_16_BITS_VALUE_SETTER_MAP.at(register_index)(registers, 0xA8F1);
        REGISTER_16_BITS_VALUE_SETTER_MAP.at(register_index)(expected_registers, 0xA8F0);
        set_expected_pc_increase(1);

        const auto cycle = gen::INSTRUCTION_FUNCTIONS[instruction_index](arguments, registers, controller);

        EXPECT_EQ(8, cycle);
    }

    INSTANTIATE_TEST_SUITE_P(
        Dec16BitsTest, Dec16BitsTestFixture, REGISTER_16_BITS_VALUES, NameMapPrinter(REGISTER_16_BITS_VALUE_NAME_MAP)
    );

    class Dec8BitsTestFixture: public InstructionTestFixture, public ::testing::WithParamInterface<uint16_t> {};

    TEST_P(Dec8BitsTestFixture, DEC8Bits)
    {
        uint16_t register_index = GetParam();
        uint16_t instruction_index = (register_index << 3) + 0b101;
        // Test from Chapter 4: page 107
        REGISTER_8_BITS_VALUE_SETTER_MAP.at(register_index)(registers, 0xA8);
        registers.F = 0;
        REGISTER_8_BITS_VALUE_SETTER_MAP.at(register_index)(expected_registers, 0xA7);
        expected_registers.F = emulator::memory::make_flag(false, true, false, false);
        set_expected_pc_increase(1);

        const auto cycle = gen::INSTRUCTION_FUNCTIONS[instruction_index](arguments, registers, controller);

        EXPECT_EQ(4, cycle);
    }

    TEST_P(Dec8BitsTestFixture, DEC8BitsCarry)
    {
        uint16_t register_index = GetParam();
        uint16_t instruction_index = (register_index << 3) + 0b101;
        // Test from Chapter 4: page 106
        REGISTER_8_BITS_VALUE_SETTER_MAP.at(register_index)(registers, 0x00);
        registers.F = 0;
        REGISTER_8_BITS_VALUE_SETTER_MAP.at(register_index)(expected_registers, 0xFF);
        expected_registers.F = emulator::memory::make_flag(false, true, true, false);
        set_expected_pc_increase(1);

        const auto cycle = gen::INSTRUCTION_FUNCTIONS[instruction_index](arguments, registers, controller);

        EXPECT_EQ(4, cycle);
    }

    TEST_P(Dec8BitsTestFixture, DEC8BitsZero)
    {
        uint16_t register_index = GetParam();
        uint16_t instruction_index = (register_index << 3) + 0b101;
        // Test from Chapter 4: page 106
        REGISTER_8_BITS_VALUE_SETTER_MAP.at(register_index)(registers, 0x01);
        registers.F = 0;
        REGISTER_8_BITS_VALUE_SETTER_MAP.at(register_index)(expected_registers, 0x00);
        expected_registers.F = emulator::memory::make_flag(true, true, false, false);
        set_expected_pc_increase(1);

        const auto cycle = gen::INSTRUCTION_FUNCTIONS[instruction_index](arguments, registers, controller);

        EXPECT_EQ(4, cycle);
    }

    INSTANTIATE_TEST_SUITE_P(
        Dec8BitsTest, Dec8BitsTestFixture, REGISTER_8_BITS_VALUES, NameMapPrinter(REGISTER_8_BITS_VALUE_NAME_MAP)
    );

    TEST_F(Dec8BitsTestFixture, DEC8BitsAddress)
    {
        // Test from Chapter 4: page 106
        uint16_t instruction_index = 0b110101;
        registers.F = 0;
        registers.H = 0xD1;
        registers.L = 0xC7;
        expected_registers.F = emulator::memory::make_flag(false, true, false, false);
        expected_registers.H = 0xD1;
        expected_registers.L = 0xC7;
        EXPECT_CALL (controller, get(0xD1C7)).Times(1).WillOnce(::testing::Return(0x5A));
        EXPECT_CALL (controller, set(0xD1C7, 0x59)).Times(1);

        set_expected_pc_increase(1);

        const auto cycle = gen::INSTRUCTION_FUNCTIONS[instruction_index](arguments, registers, controller);

        EXPECT_EQ(12, cycle);
    }

    TEST_F(Dec8BitsTestFixture, DEC8BitsAddressCarry)
    {
        // Test from Chapter 4: page 106
        uint16_t instruction_index = 0b110101;
        registers.F = 0;
        registers.H = 0xD1;
        registers.L = 0xC7;
        expected_registers.F = emulator::memory::make_flag(false, true, true, false);
        expected_registers.H = 0xD1;
        expected_registers.L = 0xC7;
        EXPECT_CALL (controller, get(0xD1C7)).Times(1).WillOnce(::testing::Return(0x00));
        EXPECT_CALL (controller, set(0xD1C7, 0xFF)).Times(1);

        set_expected_pc_increase(1);

        const auto cycle = gen::INSTRUCTION_FUNCTIONS[instruction_index](arguments, registers, controller);

        EXPECT_EQ(12, cycle);
    }

    TEST_F(Dec8BitsTestFixture, DEC8BitsAddressZero)
    {
        // Test from Chapter 4: page 106
        uint16_t instruction_index = 0b110101;
        registers.F = 0;
        registers.H = 0xD1;
        registers.L = 0xC7;
        expected_registers.F = emulator::memory::make_flag(true, true, false, false);
        expected_registers.H = 0xD1;
        expected_registers.L = 0xC7;
        EXPECT_CALL (controller, get(0xD1C7)).Times(1).WillOnce(::testing::Return(0x01));
        EXPECT_CALL (controller, set(0xD1C7, 0x00)).Times(1);

        set_expected_pc_increase(1);

        const auto cycle = gen::INSTRUCTION_FUNCTIONS[instruction_index](arguments, registers, controller);

        EXPECT_EQ(12, cycle);
    }
}
