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
    class Cp8BitsTestFixture: public InstructionTestFixture, public ::testing::WithParamInterface<uint16_t> {};

    TEST_P(Cp8BitsTestFixture, CP8Bits)
    {
        uint16_t register_index = GetParam();
        uint16_t instruction_index = register_index + 0b1011'1000;
        // Test from Chapter 4: page 105/6
        REGISTER_8_BITS_VALUE_SETTER_MAP.at(register_index)(registers, 0x47);
        registers.F = 0;
        registers.A = 0xAE;
        REGISTER_8_BITS_VALUE_SETTER_MAP.at(register_index)(expected_registers, 0x47);
        expected_registers.A = 0xAE;
        expected_registers.F = emulator::memory::make_flag(false, true, false, false);

        set_expected_pc_increase(1);

        const auto cycle = gen::INSTRUCTION_FUNCTIONS[instruction_index](arguments, registers, controller);

        EXPECT_EQ(4, cycle);
    }

    TEST_P(Cp8BitsTestFixture, CP8BitsHalfCarry)
    {
        uint16_t register_index = GetParam();
        uint16_t instruction_index = register_index + 0b1011'1000;
        // Test from Chapter 4: page 105/6
        REGISTER_8_BITS_VALUE_SETTER_MAP.at(register_index)(registers, 0x2F);
        registers.F = 0;
        registers.A = 0x3C;
        REGISTER_8_BITS_VALUE_SETTER_MAP.at(register_index)(expected_registers, 0x2F);
        expected_registers.A = 0x3C;
        expected_registers.F = emulator::memory::make_flag(false, true, true, false);

        set_expected_pc_increase(1);

        const auto cycle = gen::INSTRUCTION_FUNCTIONS[instruction_index](arguments, registers, controller);

        EXPECT_EQ(4, cycle);
    }

    TEST_P(Cp8BitsTestFixture, CP8BitsCarry)
    {
        uint16_t register_index = GetParam();
        uint16_t instruction_index = register_index + 0b1011'1000;
        // Test from Chapter 4: page 105/6
        REGISTER_8_BITS_VALUE_SETTER_MAP.at(register_index)(registers, 0x40);
        registers.F = 0;
        registers.A = 0x3E;
        REGISTER_8_BITS_VALUE_SETTER_MAP.at(register_index)(expected_registers, 0x40);
        expected_registers.A = 0x3E;
        expected_registers.F = emulator::memory::make_flag(false, true, false, true);

        set_expected_pc_increase(1);

        const auto cycle = gen::INSTRUCTION_FUNCTIONS[instruction_index](arguments, registers, controller);

        EXPECT_EQ(4, cycle);
    }

    TEST_P(Cp8BitsTestFixture, CP8BitsZero)
    {
        uint16_t register_index = GetParam();
        uint16_t instruction_index = register_index + 0b1011'1000;
        // Test from Chapter 4: page 105/6
        REGISTER_8_BITS_VALUE_SETTER_MAP.at(register_index)(registers, 0x3E);
        registers.F = 0;
        registers.A = 0x3E;
        REGISTER_8_BITS_VALUE_SETTER_MAP.at(register_index)(expected_registers, 0x3E);
        expected_registers.F = emulator::memory::make_flag(true, true, false, false);
        expected_registers.A = 0x3E;

        set_expected_pc_increase(1);

        const auto cycle = gen::INSTRUCTION_FUNCTIONS[instruction_index](arguments, registers, controller);

        EXPECT_EQ(4, cycle);
    }

    INSTANTIATE_TEST_SUITE_P(
        Cp8BitsTest, Cp8BitsTestFixture, REGISTER_8_BITS_VALUES_WITHOUT_A, NameMapPrinter(REGISTER_8_BITS_VALUE_NAME_MAP)
    );

    TEST_F(Cp8BitsTestFixture, CPBitsRegisterA)
    {
        uint16_t instruction_index = REGISTER_A_INDEX + 0b1011'1000;
        // Test from Chapter 4: page 105/6
        registers.F = 0;
        registers.A = 0xD6;
        expected_registers.A = 0xD6;
        expected_registers.F = emulator::memory::make_flag(true, true, false, false);

        set_expected_pc_increase(1);

        const auto cycle = gen::INSTRUCTION_FUNCTIONS[instruction_index](arguments, registers, controller);

        EXPECT_EQ(4, cycle);
    }

    TEST_F(Cp8BitsTestFixture, CP8BitsAddress)
    {
        // Test from Chapter 4: page 105/6
        uint16_t instruction_index = 0b1011'1110;
        registers.F = emulator::memory::make_flag(false, false, false, false);
        registers.H = 0xD1;
        registers.L = 0xC7;
        registers.A = 0x3C;
        expected_registers.F = emulator::memory::make_flag(false, true, false, false);
        expected_registers.H = 0xD1;
        expected_registers.L = 0xC7;
        expected_registers.A = 0x3C;
        EXPECT_CALL (controller, get(0xD1C7)).Times(1).WillOnce(::testing::Return(0x12));
        set_expected_pc_increase(1);

        const auto cycle = gen::INSTRUCTION_FUNCTIONS[instruction_index](arguments, registers, controller);

        EXPECT_EQ(8, cycle);
    }

    TEST_F(Cp8BitsTestFixture, CP8BitsAddressHalfCarry)
    {
        // Test from Chapter 4: page 105/6
        uint16_t instruction_index = 0b1011'1110;
        registers.F = emulator::memory::make_flag(false, false, false, false);
        registers.H = 0xD1;
        registers.L = 0xC7;
        registers.A = 0x68;
        expected_registers.F = emulator::memory::make_flag(false, true, true, false);
        expected_registers.H = 0xD1;
        expected_registers.L = 0xC7;
        expected_registers.A = 0x68;
        EXPECT_CALL (controller, get(0xD1C7)).Times(1).WillOnce(::testing::Return(0x5F));
        set_expected_pc_increase(1);

        const auto cycle = gen::INSTRUCTION_FUNCTIONS[instruction_index](arguments, registers, controller);

        EXPECT_EQ(8, cycle);
    }

    TEST_F(Cp8BitsTestFixture, CP8BitsAddressCarry)
    {
        // Test from Chapter 4: page 105/6
        uint16_t instruction_index = 0b1011'1110;
        registers.F = emulator::memory::make_flag(false, false, false, false);
        registers.H = 0xD1;
        registers.L = 0xC7;
        registers.A = 0x3C;
        expected_registers.F = emulator::memory::make_flag(false, true, false, true);
        expected_registers.H = 0xD1;
        expected_registers.L = 0xC7;
        expected_registers.A = 0x3C;
        EXPECT_CALL (controller, get(0xD1C7)).Times(1).WillOnce(::testing::Return(0x40));
        set_expected_pc_increase(1);

        const auto cycle = gen::INSTRUCTION_FUNCTIONS[instruction_index](arguments, registers, controller);

        EXPECT_EQ(8, cycle);
    }

    TEST_F(Cp8BitsTestFixture, CP8BitsAddressZero)
    {
        // Test from Chapter 4: page 105/6
        uint16_t instruction_index = 0b1011'1110;
        registers.F = emulator::memory::make_flag(false, false, false, false);
        registers.H = 0xD1;
        registers.L = 0xC7;
        registers.A = 0xA8;
        expected_registers.F = emulator::memory::make_flag(true, true, false, false);
        expected_registers.H = 0xD1;
        expected_registers.L = 0xC7;
        expected_registers.A = 0xA8;
        EXPECT_CALL (controller, get(0xD1C7)).Times(1).WillOnce(::testing::Return(0xA8));
        set_expected_pc_increase(1);

        const auto cycle = gen::INSTRUCTION_FUNCTIONS[instruction_index](arguments, registers, controller);

        EXPECT_EQ(8, cycle);
    }

    TEST_F(Cp8BitsTestFixture, CP8BitsImmediate)
    {
        // Test from Chapter 4: page 105/6
        uint16_t instruction_index = 0b1111'1110;
        registers.F = emulator::memory::make_flag(false, false, false, false);
        registers.H = 0xD1;
        registers.L = 0xC7;
        registers.A = 0x3C;
        arguments.uint8 = 0x2B;
        expected_registers.F = emulator::memory::make_flag(false, true, false, false);
        expected_registers.H = 0xD1;
        expected_registers.L = 0xC7;
        expected_registers.A = 0x3C;
        set_expected_pc_increase(2);

        const auto cycle = gen::INSTRUCTION_FUNCTIONS[instruction_index](arguments, registers, controller);

        EXPECT_EQ(8, cycle);
    }

    TEST_F(Cp8BitsTestFixture, CP8BitsImmediateHalfCarry)
    {
        // Test from Chapter 4: page 105/6
        uint16_t instruction_index = 0b1111'1110;
        registers.F = emulator::memory::make_flag(false, false, false, false);
        registers.H = 0xD1;
        registers.L = 0xC7;
        registers.A = 0x78;
        arguments.uint8 = 0x0F;
        expected_registers.F = emulator::memory::make_flag(false, true, true, false);
        expected_registers.H = 0xD1;
        expected_registers.L = 0xC7;
        expected_registers.A = 0x78;
        set_expected_pc_increase(2);

        const auto cycle = gen::INSTRUCTION_FUNCTIONS[instruction_index](arguments, registers, controller);

        EXPECT_EQ(8, cycle);
    }

    TEST_F(Cp8BitsTestFixture, CP8BitsImmediateCarry)
    {
        // Test from Chapter 4: page 105/6
        uint16_t instruction_index = 0b1111'1110;
        registers.F = emulator::memory::make_flag(false, false, false, false);
        registers.H = 0xD1;
        registers.L = 0xC7;
        registers.A = 0x3C;
        arguments.uint8 = 0x50;
        expected_registers.F = emulator::memory::make_flag(false, true, false, true);
        expected_registers.H = 0xD1;
        expected_registers.L = 0xC7;
        expected_registers.A = 0x3C;
        set_expected_pc_increase(2);

        const auto cycle = gen::INSTRUCTION_FUNCTIONS[instruction_index](arguments, registers, controller);

        EXPECT_EQ(8, cycle);
    }

    TEST_F(Cp8BitsTestFixture, CP8BitsImmediateZero)
    {
        // Test from Chapter 4: page 105/6
        uint16_t instruction_index = 0b1111'1110;
        registers.F = emulator::memory::make_flag(false, false, false, false);
        registers.H = 0xD1;
        registers.L = 0xC7;
        registers.A = 0x3C;
        arguments.uint8 = 0x3C;
        expected_registers.F = emulator::memory::make_flag(true, true, false, false);
        expected_registers.H = 0xD1;
        expected_registers.L = 0xC7;
        expected_registers.A = 0x3C;
        set_expected_pc_increase(2);

        const auto cycle = gen::INSTRUCTION_FUNCTIONS[instruction_index](arguments, registers, controller);

        EXPECT_EQ(8, cycle);
    }
}
