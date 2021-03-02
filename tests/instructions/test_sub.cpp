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
    constexpr uint16_t REGISTER_A_INDEX = 0b111;

    class Sub8BitsTestFixture: public InstructionTestFixture, public ::testing::WithParamInterface<uint16_t> {};

    TEST_P(Sub8BitsTestFixture, SUB8Bits)
    {
        uint16_t register_index = GetParam();
        uint16_t instruction_index = register_index + 0b1001'0000;
        uint16_t a_value = 0xAE;
        uint16_t other_register_value = 0x47;
        // Test from Chapter 4: page 102
        REGISTER_8_BITS_VALUE_SETTER_MAP.at(register_index)(registers, other_register_value);
        registers.F = 0;
        registers.A = a_value;
        REGISTER_8_BITS_VALUE_SETTER_MAP.at(register_index)(expected_registers, other_register_value);
        if (register_index == REGISTER_A_INDEX)
        {
            expected_registers.A = 0x00;
            expected_registers.F = emulator::memory::make_flag(true, true, false, false);
        }
        else
        {
            expected_registers.A = a_value - other_register_value;
            expected_registers.F = emulator::memory::make_flag(false, true, false, false);
        }
        set_expected_pc_increase(1);

        const auto cycle = gen::INSTRUCTION_FUNCTIONS[instruction_index](arguments, registers, controller);

        EXPECT_EQ(4, cycle);
    }

    TEST_P(Sub8BitsTestFixture, SUB8BitsHalfCarry)
    {
        uint16_t register_index = GetParam();
        uint16_t instruction_index = register_index + 0b1001'0000;
        uint16_t a_value = 0x3E;
        uint16_t other_register_value = 0x0F;
        // Test from Chapter 4: page 102
        REGISTER_8_BITS_VALUE_SETTER_MAP.at(register_index)(registers, other_register_value);
        registers.F = 0;
        registers.A = a_value;
        REGISTER_8_BITS_VALUE_SETTER_MAP.at(register_index)(expected_registers, other_register_value);
        if (register_index == REGISTER_A_INDEX)
        {
            expected_registers.A = 0x00;
            expected_registers.F = emulator::memory::make_flag(true, true, false, false);
        }
        else
        {
            expected_registers.A = 0x2F;
            expected_registers.F = emulator::memory::make_flag(false, true, true, false);
        }
        set_expected_pc_increase(1);

        const auto cycle = gen::INSTRUCTION_FUNCTIONS[instruction_index](arguments, registers, controller);

        EXPECT_EQ(4, cycle);
    }

    TEST_P(Sub8BitsTestFixture, SUB8BitsCarry)
    {
        uint16_t register_index = GetParam();
        uint16_t instruction_index = register_index + 0b1001'0000;
        uint16_t a_value = 0x3E;
        uint16_t other_register_value = 0x40;
        // Test from Chapter 4: page 102
        REGISTER_8_BITS_VALUE_SETTER_MAP.at(register_index)(registers, other_register_value);
        registers.F = 0;
        registers.A = a_value;
        REGISTER_8_BITS_VALUE_SETTER_MAP.at(register_index)(expected_registers, other_register_value);
        if (register_index == REGISTER_A_INDEX)
        {
            expected_registers.A = 0x00;
            expected_registers.F = emulator::memory::make_flag(true, true, false, false);
        }
        else
        {
            expected_registers.A = 0xFE;
            expected_registers.F = emulator::memory::make_flag(false, true, false, true);
        }
        set_expected_pc_increase(1);

        const auto cycle = gen::INSTRUCTION_FUNCTIONS[instruction_index](arguments, registers, controller);

        EXPECT_EQ(4, cycle);
    }

    TEST_P(Sub8BitsTestFixture, SUB8BitsZero)
    {
        uint16_t register_index = GetParam();
        uint16_t instruction_index = register_index + 0b1001'0000;
        uint16_t a_value = 0x3E;
        uint16_t other_register_value = 0x3E;
        // Test from Chapter 4: page 102
        REGISTER_8_BITS_VALUE_SETTER_MAP.at(register_index)(registers, other_register_value);
        registers.F = 0;
        registers.A = a_value;
        REGISTER_8_BITS_VALUE_SETTER_MAP.at(register_index)(expected_registers, other_register_value);
        expected_registers.F = emulator::memory::make_flag(true, true, false, false);
        expected_registers.A = 0x00;

        set_expected_pc_increase(1);

        const auto cycle = gen::INSTRUCTION_FUNCTIONS[instruction_index](arguments, registers, controller);

        EXPECT_EQ(4, cycle);
    }

    INSTANTIATE_TEST_SUITE_P(
        Sub8BitsTest, Sub8BitsTestFixture, REGISTER_8_BITS_VALUES, NameMapPrinter(REGISTER_8_BITS_VALUE_NAME_MAP)
    );

    TEST_F(Sub8BitsTestFixture, SUB8BitsAddress)
    {
        // Test from Chapter 4: page 102
        uint16_t instruction_index = 0b1001'0110;
        registers.F = emulator::memory::make_flag(false, false, false, false);
        registers.H = 0xD1;
        registers.L = 0xC7;
        registers.A = 0x3C;
        expected_registers.F = emulator::memory::make_flag(false, true, false, false);
        expected_registers.H = 0xD1;
        expected_registers.L = 0xC7;
        expected_registers.A = 0x2A;
        EXPECT_CALL (controller, get(0xD1C7)).Times(1).WillOnce(::testing::Return(0x12));
        set_expected_pc_increase(1);

        const auto cycle = gen::INSTRUCTION_FUNCTIONS[instruction_index](arguments, registers, controller);

        EXPECT_EQ(8, cycle);
    }

    TEST_F(Sub8BitsTestFixture, SUB8BitsAddressHalfCarry)
    {
        // Test from Chapter 4: page 102
        uint16_t instruction_index = 0b1001'0110;
        registers.F = emulator::memory::make_flag(false, false, false, false);
        registers.H = 0xD1;
        registers.L = 0xC7;
        registers.A = 0x68;
        expected_registers.F = emulator::memory::make_flag(false, true, true, false);
        expected_registers.H = 0xD1;
        expected_registers.L = 0xC7;
        expected_registers.A = 0x09;
        EXPECT_CALL (controller, get(0xD1C7)).Times(1).WillOnce(::testing::Return(0x5F));
        set_expected_pc_increase(1);

        const auto cycle = gen::INSTRUCTION_FUNCTIONS[instruction_index](arguments, registers, controller);

        EXPECT_EQ(8, cycle);
    }

    TEST_F(Sub8BitsTestFixture, SUB8BitsAddressCarry)
    {
        // Test from Chapter 4: page 102
        uint16_t instruction_index = 0b1001'0110;
        registers.F = emulator::memory::make_flag(false, false, false, false);
        registers.H = 0xD1;
        registers.L = 0xC7;
        registers.A = 0x7F;
        expected_registers.F = emulator::memory::make_flag(false, true, false, true);
        expected_registers.H = 0xD1;
        expected_registers.L = 0xC7;
        expected_registers.A = 0xFE;
        EXPECT_CALL (controller, get(0xD1C7)).Times(1).WillOnce(::testing::Return(0x81));
        set_expected_pc_increase(1);

        const auto cycle = gen::INSTRUCTION_FUNCTIONS[instruction_index](arguments, registers, controller);

        EXPECT_EQ(8, cycle);
    }

    TEST_F(Sub8BitsTestFixture, SUB8BitsAddressZero)
    {
        // Test from Chapter 4: page 102
        uint16_t instruction_index = 0b1001'0110;
        registers.F = emulator::memory::make_flag(false, false, false, false);
        registers.H = 0xD1;
        registers.L = 0xC7;
        registers.A = 0xA8;
        expected_registers.F = emulator::memory::make_flag(true, true, false, false);
        expected_registers.H = 0xD1;
        expected_registers.L = 0xC7;
        expected_registers.A = 0x00;
        EXPECT_CALL (controller, get(0xD1C7)).Times(1).WillOnce(::testing::Return(0xA8));
        set_expected_pc_increase(1);

        const auto cycle = gen::INSTRUCTION_FUNCTIONS[instruction_index](arguments, registers, controller);

        EXPECT_EQ(8, cycle);
    }

    TEST_F(Sub8BitsTestFixture, SUB8BitsImmediate)
    {
        // Test from Chapter 4: page 102
        uint16_t instruction_index = 0b1101'0110;
        registers.F = emulator::memory::make_flag(false, false, false, false);
        registers.H = 0xD1;
        registers.L = 0xC7;
        registers.A = 0x3C;
        arguments.uint8 = 0x2B;
        expected_registers.F = emulator::memory::make_flag(false, true, false, false);
        expected_registers.H = 0xD1;
        expected_registers.L = 0xC7;
        expected_registers.A = 0x11;
        set_expected_pc_increase(2);

        const auto cycle = gen::INSTRUCTION_FUNCTIONS[instruction_index](arguments, registers, controller);

        EXPECT_EQ(8, cycle);
    }

    TEST_F(Sub8BitsTestFixture, SUB8BitsImmediateHalfCarry)
    {
        // Test from Chapter 4: page 102
        uint16_t instruction_index = 0b1101'0110;
        registers.F = emulator::memory::make_flag(false, false, false, false);
        registers.H = 0xD1;
        registers.L = 0xC7;
        registers.A = 0x78;
        arguments.uint8 = 0x0F;
        expected_registers.F = emulator::memory::make_flag(false, true, true, false);
        expected_registers.H = 0xD1;
        expected_registers.L = 0xC7;
        expected_registers.A = 0x69;
        set_expected_pc_increase(2);

        const auto cycle = gen::INSTRUCTION_FUNCTIONS[instruction_index](arguments, registers, controller);

        EXPECT_EQ(8, cycle);
    }

    TEST_F(Sub8BitsTestFixture, SUB8BitsImmediateCarry)
    {
        // Test from Chapter 4: page 102
        uint16_t instruction_index = 0b1101'0110;
        registers.F = emulator::memory::make_flag(false, false, false, false);
        registers.H = 0xD1;
        registers.L = 0xC7;
        registers.A = 0x3C;
        arguments.uint8 = 0x50;
        expected_registers.F = emulator::memory::make_flag(false, true, false, true);
        expected_registers.H = 0xD1;
        expected_registers.L = 0xC7;
        expected_registers.A = 0xEC;
        set_expected_pc_increase(2);

        const auto cycle = gen::INSTRUCTION_FUNCTIONS[instruction_index](arguments, registers, controller);

        EXPECT_EQ(8, cycle);
    }

    TEST_F(Sub8BitsTestFixture, SUB8BitsImmediateZero)
    {
        // Test from Chapter 4: page 102
        uint16_t instruction_index = 0b1101'0110;
        registers.F = emulator::memory::make_flag(false, false, false, false);
        registers.H = 0xD1;
        registers.L = 0xC7;
        registers.A = 0xAE;
        arguments.uint8 = 0xAE;
        expected_registers.F = emulator::memory::make_flag(true, true, false, false);
        expected_registers.H = 0xD1;
        expected_registers.L = 0xC7;
        expected_registers.A = 0x00;
        set_expected_pc_increase(2);

        const auto cycle = gen::INSTRUCTION_FUNCTIONS[instruction_index](arguments, registers, controller);

        EXPECT_EQ(8, cycle);
    }
}
