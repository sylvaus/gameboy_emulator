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
using emulator::instructions::REGISTER_A_INDEX;
using emulator::instructions::REGISTER_8_BITS_VALUES_WITHOUT_A;
using emulator::instructions::REGISTER_8_BITS_VALUE_SETTER_MAP;
using emulator::instructions::REGISTER_8_BITS_VALUE_NAME_MAP;
using emulator::instructions::NameMapPrinter;
using emulator::instructions::fixtures::InstructionTestFixture;

namespace gen = emulator::generated;

namespace
{
    class And8BitsTestFixture: public InstructionTestFixture, public ::testing::WithParamInterface<uint16_t> {};

    TEST_P(And8BitsTestFixture, AND8Bits)
    {
        uint16_t register_index = GetParam();
        uint16_t instruction_index = register_index + 0b1010'0000;
        // Test from Chapter 4: page 102
        REGISTER_8_BITS_VALUE_SETTER_MAP.at(register_index)(registers, 0x3F);
        registers.F = emulator::memory::make_flag(true, true, false, true);
        registers.A = 0x5A;
        REGISTER_8_BITS_VALUE_SETTER_MAP.at(register_index)(expected_registers, 0x3F);
        expected_registers.A = 0x1A;
        expected_registers.F = emulator::memory::make_flag(false, false, true, false);

        set_expected_pc_increase(1);

        const auto cycle = gen::INSTRUCTION_FUNCTIONS[instruction_index](arguments, registers, controller);

        EXPECT_EQ(4, cycle);
    }

    TEST_P(And8BitsTestFixture, AND8BitsZero)
    {
        uint16_t register_index = GetParam();
        uint16_t instruction_index = register_index + 0b1010'0000;
        // Test from Chapter 4: page 102
        REGISTER_8_BITS_VALUE_SETTER_MAP.at(register_index)(registers, 0b0101);
        registers.F = emulator::memory::make_flag(true, true, false, true);
        registers.A = 0b1010;
        REGISTER_8_BITS_VALUE_SETTER_MAP.at(register_index)(expected_registers, 0b0101);
        expected_registers.A = 0b0000;
        expected_registers.F = emulator::memory::make_flag(true, false, true, false);

        set_expected_pc_increase(1);

        const auto cycle = gen::INSTRUCTION_FUNCTIONS[instruction_index](arguments, registers, controller);

        EXPECT_EQ(4, cycle);
    }

    INSTANTIATE_TEST_SUITE_P(
        And8BitsTest, And8BitsTestFixture, REGISTER_8_BITS_VALUES_WITHOUT_A, NameMapPrinter(REGISTER_8_BITS_VALUE_NAME_MAP)
    );

    TEST_F(And8BitsTestFixture, ANDBitsRegisterA)
    {
        // Test from Chapter 4: page 104
        uint16_t instruction_index = REGISTER_A_INDEX + 0b1010'0000;
        registers.F = emulator::memory::make_flag(true, true, false, true);
        registers.A = 0b0111;
        expected_registers.A = 0b0111;
        expected_registers.F = emulator::memory::make_flag(false, false, true, false);

        set_expected_pc_increase(1);

        const auto cycle = gen::INSTRUCTION_FUNCTIONS[instruction_index](arguments, registers, controller);

        EXPECT_EQ(4, cycle);
    }

    TEST_F(And8BitsTestFixture, AND8BitsRegisterAZero)
    {
        // Test from Chapter 4: page 104
        uint16_t instruction_index = REGISTER_A_INDEX + 0b1010'0000;
        registers.F = emulator::memory::make_flag(true, true, false, true);
        registers.A = 0;
        expected_registers.A = 0;
        expected_registers.F = emulator::memory::make_flag(true, false, true, false);

        set_expected_pc_increase(1);

        const auto cycle = gen::INSTRUCTION_FUNCTIONS[instruction_index](arguments, registers, controller);

        EXPECT_EQ(4, cycle);
    }

    TEST_F(And8BitsTestFixture, AND8BitsAdress)
    {
        // Test from Chapter 4: page 104
        uint16_t instruction_index = 0b1010'0110;
        registers.F = emulator::memory::make_flag(true, true, false, true);
        registers.H = 0xD1;
        registers.L = 0xC7;
        registers.A = 0b1010;
        expected_registers.F = emulator::memory::make_flag(false, false, true, false);
        expected_registers.H = 0xD1;
        expected_registers.L = 0xC7;
        expected_registers.A = 0b0010;
        EXPECT_CALL (controller, get(0xD1C7)).Times(1).WillOnce(::testing::Return(0b0110));

        set_expected_pc_increase(1);

        const auto cycle = gen::INSTRUCTION_FUNCTIONS[instruction_index](arguments, registers, controller);

        EXPECT_EQ(8, cycle);
    }

    TEST_F(And8BitsTestFixture, AND8BitsAdressZero)
    {
        // Test from Chapter 4: page 104
        uint16_t instruction_index = 0b1010'0110;
        registers.F = emulator::memory::make_flag(true, true, false, true);
        registers.H = 0xD1;
        registers.L = 0xC7;
        registers.A = 0b1010;
        expected_registers.F = emulator::memory::make_flag(true, false, true, false);
        expected_registers.H = 0xD1;
        expected_registers.L = 0xC7;
        expected_registers.A = 0b0000;
        EXPECT_CALL (controller, get(0xD1C7)).Times(1).WillOnce(::testing::Return(0b0100));
        set_expected_pc_increase(1);

        const auto cycle = gen::INSTRUCTION_FUNCTIONS[instruction_index](arguments, registers, controller);

        EXPECT_EQ(8, cycle);
    }

    TEST_F(And8BitsTestFixture, AND8BitsImmediate)
    {
        // Test from Chapter 4: page 104
        uint16_t instruction_index = 0b1110'0110;
        registers.F = emulator::memory::make_flag(true, true, false, true);
        registers.A = 0b11110101;
        arguments.uint8 = 0b10101111;
        expected_registers.A = 0b10100101;
        expected_registers.F = emulator::memory::make_flag(false, false, true, false);

        set_expected_pc_increase(2);

        const auto cycle = gen::INSTRUCTION_FUNCTIONS[instruction_index](arguments, registers, controller);

        EXPECT_EQ(8, cycle);
    }

    TEST_F(And8BitsTestFixture, AND8BitsImmediateZero)
    {
        // Test from Chapter 4: page 104
        uint16_t instruction_index = 0b1110'0110;
        registers.F = emulator::memory::make_flag(true, true, false, true);
        registers.A = 0;
        arguments.uint8 = 0x2B;
        expected_registers.A = 0;
        expected_registers.F = emulator::memory::make_flag(true, false, true, false);
        set_expected_pc_increase(2);

        const auto cycle = gen::INSTRUCTION_FUNCTIONS[instruction_index](arguments, registers, controller);

        EXPECT_EQ(8, cycle);
    }
}
