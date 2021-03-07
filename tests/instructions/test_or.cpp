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
    class Or8BitsTestFixture: public InstructionTestFixture, public ::testing::WithParamInterface<uint16_t> {};

    TEST_P(Or8BitsTestFixture, OR8Bits)
    {
        uint16_t register_index = GetParam();
        uint16_t instruction_index = register_index + 0b1011'0000;
        // Test from Chapter 4: page 102
        REGISTER_8_BITS_VALUE_SETTER_MAP.at(register_index)(registers, 0b10100101);
        registers.F = emulator::memory::make_flag(true, true, false, true);
        registers.A = 0b01011010;
        REGISTER_8_BITS_VALUE_SETTER_MAP.at(register_index)(expected_registers, 0b10100101);
        expected_registers.A = 0b11111111;
        expected_registers.F = emulator::memory::make_flag(false, false, false, false);

        set_expected_pc_increase(1);

        const auto cycle = gen::INSTRUCTION_FUNCTIONS[instruction_index](arguments, registers, controller);

        EXPECT_EQ(4, cycle);
    }

    TEST_P(Or8BitsTestFixture, OR8BitsZero)
    {
        uint16_t register_index = GetParam();
        uint16_t instruction_index = register_index + 0b1011'0000;
        // Test from Chapter 4: page 102
        REGISTER_8_BITS_VALUE_SETTER_MAP.at(register_index)(registers, 0);
        registers.F = emulator::memory::make_flag(true, true, false, true);
        registers.A = 0;
        REGISTER_8_BITS_VALUE_SETTER_MAP.at(register_index)(expected_registers, 0);
        expected_registers.A = 0;
        expected_registers.F = emulator::memory::make_flag(true, false, false, false);

        set_expected_pc_increase(1);

        const auto cycle = gen::INSTRUCTION_FUNCTIONS[instruction_index](arguments, registers, controller);

        EXPECT_EQ(4, cycle);
    }

    INSTANTIATE_TEST_SUITE_P(
        Or8BitsTest, Or8BitsTestFixture, REGISTER_8_BITS_VALUES_WITHOUT_A, NameMapPrinter(REGISTER_8_BITS_VALUE_NAME_MAP)
    );

    TEST_F(Or8BitsTestFixture, OR8BitsRegisterA)
    {
        // Test from Chapter 4: page 104
        uint16_t instruction_index = REGISTER_A_INDEX + 0b1011'0000;
        registers.F = emulator::memory::make_flag(true, true, false, true);
        registers.A = 0x7C;
        expected_registers.A = 0x7C;
        expected_registers.F = emulator::memory::make_flag(false, false, false, false);

        set_expected_pc_increase(1);

        const auto cycle = gen::INSTRUCTION_FUNCTIONS[instruction_index](arguments, registers, controller);

        EXPECT_EQ(4, cycle);
    }

    TEST_F(Or8BitsTestFixture, OR8BitsRegisterAZero)
    {
        // Test from Chapter 4: page 104
        uint16_t instruction_index = REGISTER_A_INDEX + 0b1011'0000;
        registers.F = emulator::memory::make_flag(true, true, false, true);
        registers.A = 0;
        expected_registers.A = 0;
        expected_registers.F = emulator::memory::make_flag(true, false, false, false);

        set_expected_pc_increase(1);

        const auto cycle = gen::INSTRUCTION_FUNCTIONS[instruction_index](arguments, registers, controller);

        EXPECT_EQ(4, cycle);
    }

    TEST_F(Or8BitsTestFixture, OR8BitsAdress)
    {
        // Test from Chapter 4: page 104
        uint16_t instruction_index = 0b1011'0110;
        registers.F = emulator::memory::make_flag(true, true, false, true);
        registers.H = 0xD1;
        registers.L = 0xC7;
        registers.A = 0x5A;
        expected_registers.F = emulator::memory::make_flag(false, false, false, false);
        expected_registers.H = 0xD1;
        expected_registers.L = 0xC7;
        expected_registers.A = 0x5F;
        EXPECT_CALL (controller, get(0xD1C7)).Times(1).WillOnce(::testing::Return(0x0F));

        set_expected_pc_increase(1);

        const auto cycle = gen::INSTRUCTION_FUNCTIONS[instruction_index](arguments, registers, controller);

        EXPECT_EQ(8, cycle);
    }

    TEST_F(Or8BitsTestFixture, OR8BitsAdressZero)
    {
        // Test from Chapter 4: page 104
        uint16_t instruction_index = 0b1011'0110;
        registers.F = emulator::memory::make_flag(true, true, false, true);
        registers.H = 0xD1;
        registers.L = 0xC7;
        registers.A = 0;
        expected_registers.F = emulator::memory::make_flag(true, false, false, false);
        expected_registers.H = 0xD1;
        expected_registers.L = 0xC7;
        expected_registers.A = 0;
        EXPECT_CALL (controller, get(0xD1C7)).Times(1).WillOnce(::testing::Return(0));
        set_expected_pc_increase(1);

        const auto cycle = gen::INSTRUCTION_FUNCTIONS[instruction_index](arguments, registers, controller);

        EXPECT_EQ(8, cycle);
    }

    TEST_F(Or8BitsTestFixture, OR8BitsImmediate)
    {
        // Test from Chapter 4: page 104
        uint16_t instruction_index = 0b1111'0110;
        registers.F = emulator::memory::make_flag(true, true, false, true);
        registers.A = 0x5A;
        arguments.uint8 = 0x03;
        expected_registers.A = 0x5B;
        expected_registers.F = emulator::memory::make_flag(false, false, false, false);

        set_expected_pc_increase(2);

        const auto cycle = gen::INSTRUCTION_FUNCTIONS[instruction_index](arguments, registers, controller);

        EXPECT_EQ(8, cycle);
    }

    TEST_F(Or8BitsTestFixture, OR8BitsImmediateZero)
    {
        // Test from Chapter 4: page 104
        uint16_t instruction_index = 0b1111'0110;
        registers.F = emulator::memory::make_flag(true, true, false, true);
        registers.A = 0;
        arguments.uint8 = 0;
        expected_registers.A = 0;
        expected_registers.F = emulator::memory::make_flag(true, false, false, false);
        set_expected_pc_increase(2);

        const auto cycle = gen::INSTRUCTION_FUNCTIONS[instruction_index](arguments, registers, controller);

        EXPECT_EQ(8, cycle);
    }
}
