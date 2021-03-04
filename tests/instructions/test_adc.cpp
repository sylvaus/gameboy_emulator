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
    class Adc8BitsTestFixture: public InstructionTestFixture, public ::testing::WithParamInterface<uint16_t> {};

    TEST_P(Adc8BitsTestFixture, ADC8Bits)
    {
        uint16_t register_index = GetParam();
        uint16_t instruction_index = register_index + 0b1000'1000;
        uint16_t a_value = 0x45;
        uint16_t other_register_value = 0xA2;
        // Test from Chapter 4: page 102
        REGISTER_8_BITS_VALUE_SETTER_MAP.at(register_index)(registers, other_register_value);
        registers.F = 0;
        registers.A = a_value;
        REGISTER_8_BITS_VALUE_SETTER_MAP.at(register_index)(expected_registers, other_register_value);
        expected_registers.F = 0;
        expected_registers.A = a_value + other_register_value;

        set_expected_pc_increase(1);

        const auto cycle = gen::INSTRUCTION_FUNCTIONS[instruction_index](arguments, registers, controller);

        EXPECT_EQ(4, cycle);
    }

    TEST_P(Adc8BitsTestFixture, ADC8BitsHalfCarry)
    {
        uint16_t register_index = GetParam();
        uint16_t instruction_index = register_index + 0b1000'1000;
        uint16_t a_value = 0xA;
        uint16_t other_register_value = 0xB;
        // Test from Chapter 4: page 102
        REGISTER_8_BITS_VALUE_SETTER_MAP.at(register_index)(registers, other_register_value);
        registers.F = 0;
        registers.A = a_value;
        REGISTER_8_BITS_VALUE_SETTER_MAP.at(register_index)(expected_registers, other_register_value);
        expected_registers.F = emulator::memory::make_flag(false, false, true, false);
        expected_registers.A = a_value + other_register_value;

        set_expected_pc_increase(1);

        const auto cycle = gen::INSTRUCTION_FUNCTIONS[instruction_index](arguments, registers, controller);

        EXPECT_EQ(4, cycle);
    }

    TEST_P(Adc8BitsTestFixture, ADC8BitsCarry)
    {
        uint16_t register_index = GetParam();
        uint16_t instruction_index = register_index + 0b1000'1000;
        uint16_t a_value = 0xC0;
        uint16_t other_register_value = 0x90;
        // Test from Chapter 4: page 102
        REGISTER_8_BITS_VALUE_SETTER_MAP.at(register_index)(registers, other_register_value);
        registers.F = 0;
        registers.A = a_value;
        REGISTER_8_BITS_VALUE_SETTER_MAP.at(register_index)(expected_registers, other_register_value);
        expected_registers.F = emulator::memory::make_flag(false, false, false, true);
        expected_registers.A = a_value + other_register_value;

        set_expected_pc_increase(1);

        const auto cycle = gen::INSTRUCTION_FUNCTIONS[instruction_index](arguments, registers, controller);

        EXPECT_EQ(4, cycle);
    }

    TEST_P(Adc8BitsTestFixture, ADC8BitsZero)
    {
        uint16_t register_index = GetParam();
        uint16_t instruction_index = register_index + 0b1000'1000;
        uint16_t a_value = 0x00;
        uint16_t other_register_value = 0x00;
        // Test from Chapter 4: page 102
        REGISTER_8_BITS_VALUE_SETTER_MAP.at(register_index)(registers, other_register_value);
        registers.F = 0;
        registers.A = a_value;
        REGISTER_8_BITS_VALUE_SETTER_MAP.at(register_index)(expected_registers, other_register_value);
        expected_registers.F = emulator::memory::make_flag(true, false, false, false);
        expected_registers.A = 0x00;

        set_expected_pc_increase(1);

        const auto cycle = gen::INSTRUCTION_FUNCTIONS[instruction_index](arguments, registers, controller);

        EXPECT_EQ(4, cycle);
    }

    TEST_P(Adc8BitsTestFixture, ADC8BitsCarryAlreadyPresent)
    {
        uint16_t register_index = GetParam();
        uint16_t instruction_index = register_index + 0b1000'1000;
        uint16_t a_value = 0x45;
        uint16_t other_register_value = 0xA2;
        // Test from Chapter 4: page 102
        REGISTER_8_BITS_VALUE_SETTER_MAP.at(register_index)(registers, other_register_value);
        registers.F = emulator::memory::make_flag(false, false, false, true);
        registers.A = a_value;
        REGISTER_8_BITS_VALUE_SETTER_MAP.at(register_index)(expected_registers, other_register_value);
        expected_registers.F = emulator::memory::make_flag(false, false, false, false);
        expected_registers.A = a_value + other_register_value + 1;

        set_expected_pc_increase(1);

        const auto cycle = gen::INSTRUCTION_FUNCTIONS[instruction_index](arguments, registers, controller);

        EXPECT_EQ(4, cycle);
    }

    TEST_P(Adc8BitsTestFixture, ADC8BitsHalfCarryAndCarryAlreadyPresent)
    {
        uint16_t register_index = GetParam();
        uint16_t instruction_index = register_index + 0b1000'1000;
        uint16_t a_value = 0xA;
        uint16_t other_register_value = 0xB;
        // Test from Chapter 4: page 102
        REGISTER_8_BITS_VALUE_SETTER_MAP.at(register_index)(registers, other_register_value);
        registers.F =  emulator::memory::make_flag(false, false, false, true);
        registers.A = a_value;
        REGISTER_8_BITS_VALUE_SETTER_MAP.at(register_index)(expected_registers, other_register_value);
        expected_registers.F = emulator::memory::make_flag(false, false, true, false);
        expected_registers.A = a_value + other_register_value + 1;

        set_expected_pc_increase(1);

        const auto cycle = gen::INSTRUCTION_FUNCTIONS[instruction_index](arguments, registers, controller);

        EXPECT_EQ(4, cycle);
    }

    TEST_P(Adc8BitsTestFixture, ADC8BitsCarryAndCarryAlreadyPresent)
    {
        uint16_t register_index = GetParam();
        uint16_t instruction_index = register_index + 0b1000'1000;
        uint16_t a_value = 0xC0;
        uint16_t other_register_value = 0x90;
        // Test from Chapter 4: page 102
        REGISTER_8_BITS_VALUE_SETTER_MAP.at(register_index)(registers, other_register_value);
        registers.F = emulator::memory::make_flag(false, false, false, true);
        registers.A = a_value;
        REGISTER_8_BITS_VALUE_SETTER_MAP.at(register_index)(expected_registers, other_register_value);
        expected_registers.F = emulator::memory::make_flag(false, false, false, true);
        expected_registers.A = a_value + other_register_value + 1;

        set_expected_pc_increase(1);

        const auto cycle = gen::INSTRUCTION_FUNCTIONS[instruction_index](arguments, registers, controller);

        EXPECT_EQ(4, cycle);
    }

    TEST_P(Adc8BitsTestFixture, ADC8BitsZeroAndCarryAlreadyPresent)
    {
        uint16_t register_index = GetParam();
        uint16_t instruction_index = register_index + 0b1000'1000;
        uint16_t a_value = 0x88;
        uint16_t other_register_value = 0x77;
        // Test from Chapter 4: page 102
        REGISTER_8_BITS_VALUE_SETTER_MAP.at(register_index)(registers, other_register_value);
        registers.F =  emulator::memory::make_flag(false, false, false, true);
        registers.A = a_value;
        REGISTER_8_BITS_VALUE_SETTER_MAP.at(register_index)(expected_registers, other_register_value);
        expected_registers.F = emulator::memory::make_flag(true, false, true, true);
        expected_registers.A = 0x00;

        set_expected_pc_increase(1);

        const auto cycle = gen::INSTRUCTION_FUNCTIONS[instruction_index](arguments, registers, controller);

        EXPECT_EQ(4, cycle);
    }

    INSTANTIATE_TEST_SUITE_P(
        Adc8BitsTest, Adc8BitsTestFixture, REGISTER_8_BITS_VALUES_WITHOUT_A, NameMapPrinter(REGISTER_8_BITS_VALUE_NAME_MAP)
    );

    TEST_F(Adc8BitsTestFixture, ADC8BitsRegisterA)
    {
        uint16_t instruction_index = REGISTER_A_INDEX + 0b1000'1000;
        // Test from Chapter 4: page 102
        registers.F = 0;
        registers.A = 0x45;
        expected_registers.F = 0;
        expected_registers.A = 0x8A;

        set_expected_pc_increase(1);

        const auto cycle = gen::INSTRUCTION_FUNCTIONS[instruction_index](arguments, registers, controller);

        EXPECT_EQ(4, cycle);
    }

    TEST_F(Adc8BitsTestFixture, ADC8BitsRegisterAHalfCarry)
    {
        uint16_t instruction_index = REGISTER_A_INDEX + 0b1000'1000;
        // Test from Chapter 4: page 102
        registers.F = 0;
        registers.A = 0xA;
        expected_registers.F = emulator::memory::make_flag(false, false, true, false);
        expected_registers.A = 0x14;

        set_expected_pc_increase(1);

        const auto cycle = gen::INSTRUCTION_FUNCTIONS[instruction_index](arguments, registers, controller);

        EXPECT_EQ(4, cycle);
    }

    TEST_F(Adc8BitsTestFixture, ADC8BitsRegisterACarry)
    {
        uint16_t instruction_index = REGISTER_A_INDEX + 0b1000'1000;
        // Test from Chapter 4: page 102
        registers.F = 0;
        registers.A = 0xC0;
        expected_registers.F = emulator::memory::make_flag(false, false, false, true);
        expected_registers.A = 0x80;

        set_expected_pc_increase(1);

        const auto cycle = gen::INSTRUCTION_FUNCTIONS[instruction_index](arguments, registers, controller);

        EXPECT_EQ(4, cycle);
    }

    TEST_F(Adc8BitsTestFixture, ADC8BitsRegisterAZero)
    {
        uint16_t instruction_index = REGISTER_A_INDEX + 0b1000'1000;
        // Test from Chapter 4: page 102
        registers.F = 0;
        registers.A = 0x00;
        expected_registers.F = emulator::memory::make_flag(true, false, false, false);
        expected_registers.A = 0x00;

        set_expected_pc_increase(1);

        const auto cycle = gen::INSTRUCTION_FUNCTIONS[instruction_index](arguments, registers, controller);

        EXPECT_EQ(4, cycle);
    }

    TEST_F(Adc8BitsTestFixture, ADC8BitsRegisterACarryAlreadyPresent)
    {
        uint16_t instruction_index = REGISTER_A_INDEX + 0b1000'1000;
        // Test from Chapter 4: page 102
        registers.F = emulator::memory::make_flag(false, false, false, true);
        registers.A = 0x45;
        expected_registers.F = emulator::memory::make_flag(false, false, false, false);
        expected_registers.A = 0x8B;

        set_expected_pc_increase(1);

        const auto cycle = gen::INSTRUCTION_FUNCTIONS[instruction_index](arguments, registers, controller);

        EXPECT_EQ(4, cycle);
    }

    TEST_F(Adc8BitsTestFixture, ADC8BitsRegisterAHalfCarryAndCarryAlreadyPresent)
    {
        uint16_t instruction_index = REGISTER_A_INDEX + 0b1000'1000;
        // Test from Chapter 4: page 102
        registers.F =  emulator::memory::make_flag(false, false, false, true);
        registers.A = 0xA;
        expected_registers.F = emulator::memory::make_flag(false, false, true, false);
        expected_registers.A = 0x15;

        set_expected_pc_increase(1);

        const auto cycle = gen::INSTRUCTION_FUNCTIONS[instruction_index](arguments, registers, controller);

        EXPECT_EQ(4, cycle);
    }

    TEST_F(Adc8BitsTestFixture, ADC8BitsRegisterACarryAndCarryAlreadyPresent)
    {
        uint16_t instruction_index = REGISTER_A_INDEX + 0b1000'1000;
        // Test from Chapter 4: page 102
        registers.F = emulator::memory::make_flag(false, false, false, true);
        registers.A = 0xC0;
        expected_registers.F = emulator::memory::make_flag(false, false, false, true);
        expected_registers.A = 0x81;

        set_expected_pc_increase(1);

        const auto cycle = gen::INSTRUCTION_FUNCTIONS[instruction_index](arguments, registers, controller);

        EXPECT_EQ(4, cycle);
    }

    TEST_F(Adc8BitsTestFixture, ADC8BitsRegisterAZeroAndCarryAlreadyPresent)
    {
        uint16_t instruction_index = REGISTER_A_INDEX + 0b1000'1000;
        // Test from Chapter 4: page 102
        registers.F =  emulator::memory::make_flag(false, false, false, true);
        registers.A = 0x88;
        expected_registers.F = emulator::memory::make_flag(false, false, true, true);
        expected_registers.A = 0x11;

        set_expected_pc_increase(1);

        const auto cycle = gen::INSTRUCTION_FUNCTIONS[instruction_index](arguments, registers, controller);

        EXPECT_EQ(4, cycle);
    }

    TEST_F(Adc8BitsTestFixture, ADC8BitsAddress)
    {
        // Test from Chapter 4: page 102
        uint16_t instruction_index = 0x8E;
        registers.F = 0x00;
        registers.H = 0xD1;
        registers.L = 0xC7;
        registers.A = 0x3C;
        expected_registers.F = 0;
        expected_registers.H = 0xD1;
        expected_registers.L = 0xC7;
        expected_registers.A = 0x4E;
        EXPECT_CALL (controller, get(0xD1C7)).Times(1).WillOnce(::testing::Return(0x12));
        set_expected_pc_increase(1);

        const auto cycle = gen::INSTRUCTION_FUNCTIONS[instruction_index](arguments, registers, controller);

        EXPECT_EQ(8, cycle);
    }

    TEST_F(Adc8BitsTestFixture, ADC8BitsAddressHalfCarry)
    {
        // Test from Chapter 4: page 102
        uint16_t instruction_index = 0x8E;
        registers.F = 0;
        registers.H = 0xD1;
        registers.L = 0xC7;
        registers.A = 0x18;
        expected_registers.F = emulator::memory::make_flag(false, false, true, false);
        expected_registers.H = 0xD1;
        expected_registers.L = 0xC7;
        expected_registers.A = 0x77;
        EXPECT_CALL (controller, get(0xD1C7)).Times(1).WillOnce(::testing::Return(0x5F));
        set_expected_pc_increase(1);

        const auto cycle = gen::INSTRUCTION_FUNCTIONS[instruction_index](arguments, registers, controller);

        EXPECT_EQ(8, cycle);
    }

    TEST_F(Adc8BitsTestFixture, ADC8BitsAddressCarry)
    {
        // Test from Chapter 4: page 102
        uint16_t instruction_index = 0x8E;
        registers.F = 0;
        registers.H = 0xD1;
        registers.L = 0xC7;
        registers.A = 0x81;
        expected_registers.F = emulator::memory::make_flag(false, false, false, true);
        expected_registers.H = 0xD1;
        expected_registers.L = 0xC7;
        expected_registers.A = 0x7F;
        EXPECT_CALL (controller, get(0xD1C7)).Times(1).WillOnce(::testing::Return(0xFE));
        set_expected_pc_increase(1);

        const auto cycle = gen::INSTRUCTION_FUNCTIONS[instruction_index](arguments, registers, controller);

        EXPECT_EQ(8, cycle);
    }

    TEST_F(Adc8BitsTestFixture, ADC8BitsAddressZero)
    {
        // Test from Chapter 4: page 102
        uint16_t instruction_index = 0x8E;
        registers.F = 0;
        registers.H = 0xD1;
        registers.L = 0xC7;
        registers.A = 0x81;
        expected_registers.F = emulator::memory::make_flag(true, false, true, true);
        expected_registers.H = 0xD1;
        expected_registers.L = 0xC7;
        expected_registers.A = 0x00;
        EXPECT_CALL (controller, get(0xD1C7)).Times(1).WillOnce(::testing::Return(0x7F));
        set_expected_pc_increase(1);

        const auto cycle = gen::INSTRUCTION_FUNCTIONS[instruction_index](arguments, registers, controller);

        EXPECT_EQ(8, cycle);
    }

    TEST_F(Adc8BitsTestFixture, ADC8BitsAddressCarryAlreadyPresent)
    {
        // Test from Chapter 4: page 102
        uint16_t instruction_index = 0x8E;
        registers.F = emulator::memory::make_flag(false, false, false, true);
        registers.H = 0xD1;
        registers.L = 0xC7;
        registers.A = 0x3C;
        expected_registers.F = 0;
        expected_registers.H = 0xD1;
        expected_registers.L = 0xC7;
        expected_registers.A = 0x4F;
        EXPECT_CALL (controller, get(0xD1C7)).Times(1).WillOnce(::testing::Return(0x12));
        set_expected_pc_increase(1);

        const auto cycle = gen::INSTRUCTION_FUNCTIONS[instruction_index](arguments, registers, controller);

        EXPECT_EQ(8, cycle);
    }

    TEST_F(Adc8BitsTestFixture, ADC8BitsAddressHalfCarryAndCarryAlreadyPresent)
    {
        // Test from Chapter 4: page 102
        uint16_t instruction_index = 0x8E;
        registers.F = emulator::memory::make_flag(false, false, false, true);
        registers.H = 0xD1;
        registers.L = 0xC7;
        registers.A = 0x18;
        expected_registers.F = emulator::memory::make_flag(false, false, true, false);
        expected_registers.H = 0xD1;
        expected_registers.L = 0xC7;
        expected_registers.A = 0x78;
        EXPECT_CALL (controller, get(0xD1C7)).Times(1).WillOnce(::testing::Return(0x5F));
        set_expected_pc_increase(1);

        const auto cycle = gen::INSTRUCTION_FUNCTIONS[instruction_index](arguments, registers, controller);

        EXPECT_EQ(8, cycle);
    }

    TEST_F(Adc8BitsTestFixture, ADC8BitsAddressCarryAndCarryAlreadyPresent)
    {
        // Test from Chapter 4: page 102
        uint16_t instruction_index = 0x8E;
        registers.F = emulator::memory::make_flag(false, false, false, true);
        registers.H = 0xD1;
        registers.L = 0xC7;
        registers.A = 0x81;
        expected_registers.F = emulator::memory::make_flag(false, false, false, true);
        expected_registers.H = 0xD1;
        expected_registers.L = 0xC7;
        expected_registers.A = 0x7F;
        EXPECT_CALL (controller, get(0xD1C7)).Times(1).WillOnce(::testing::Return(0xFD));
        set_expected_pc_increase(1);

        const auto cycle = gen::INSTRUCTION_FUNCTIONS[instruction_index](arguments, registers, controller);

        EXPECT_EQ(8, cycle);
    }

    TEST_F(Adc8BitsTestFixture, ADC8BitsAddressZeroAndCarryAlreadyPresent)
    {
        // Test from Chapter 4: page 102
        uint16_t instruction_index = 0x8E;
        registers.F = emulator::memory::make_flag(false, false, false, true);
        registers.H = 0xD1;
        registers.L = 0xC7;
        registers.A = 0x81;
        expected_registers.F = emulator::memory::make_flag(true, false, true, true);
        expected_registers.H = 0xD1;
        expected_registers.L = 0xC7;
        expected_registers.A = 0x00;
        EXPECT_CALL (controller, get(0xD1C7)).Times(1).WillOnce(::testing::Return(0x7E));
        set_expected_pc_increase(1);

        const auto cycle = gen::INSTRUCTION_FUNCTIONS[instruction_index](arguments, registers, controller);

        EXPECT_EQ(8, cycle);
    }

    TEST_F(Adc8BitsTestFixture, ADC8BitsImmediate)
    {
        // Test from Chapter 4: page 102
        uint16_t instruction_index = 0xCE;
        registers.F = 0x00;
        registers.H = 0xD1;
        registers.L = 0xC7;
        registers.A = 0x3C;
        arguments.uint8 = 0x12;
        expected_registers.F = 0;
        expected_registers.H = 0xD1;
        expected_registers.L = 0xC7;
        expected_registers.A = 0x4E;
        set_expected_pc_increase(2);

        const auto cycle = gen::INSTRUCTION_FUNCTIONS[instruction_index](arguments, registers, controller);

        EXPECT_EQ(8, cycle);
    }

    TEST_F(Adc8BitsTestFixture, ADC8BitsImmediateHalfCarry)
    {
        // Test from Chapter 4: page 102
        uint16_t instruction_index = 0xCE;
        registers.F = 0;
        registers.H = 0xD1;
        registers.L = 0xC7;
        registers.A = 0x18;
        arguments.uint8 = 0x5F;
        expected_registers.F = emulator::memory::make_flag(false, false, true, false);
        expected_registers.H = 0xD1;
        expected_registers.L = 0xC7;
        expected_registers.A = 0x77;
        set_expected_pc_increase(2);

        const auto cycle = gen::INSTRUCTION_FUNCTIONS[instruction_index](arguments, registers, controller);

        EXPECT_EQ(8, cycle);
    }

    TEST_F(Adc8BitsTestFixture, ADC8BitsImmediateCarry)
    {
        // Test from Chapter 4: page 102
        uint16_t instruction_index = 0xCE;
        registers.F = 0;
        registers.H = 0xD1;
        registers.L = 0xC7;
        registers.A = 0x3C;
        arguments.uint8 = 0xFF;
        expected_registers.F = emulator::memory::make_flag(false, false, true, true);
        expected_registers.H = 0xD1;
        expected_registers.L = 0xC7;
        expected_registers.A = 0x3B;
        set_expected_pc_increase(2);

        const auto cycle = gen::INSTRUCTION_FUNCTIONS[instruction_index](arguments, registers, controller);

        EXPECT_EQ(8, cycle);
    }

    TEST_F(Adc8BitsTestFixture, ADC8BitsImmediateZero)
    {
        // Test from Chapter 4: page 102
        uint16_t instruction_index = 0xCE;
        registers.F = 0;
        registers.H = 0xD1;
        registers.L = 0xC7;
        registers.A = 0x81;
        arguments.uint8 = 0x7F;
        expected_registers.F = emulator::memory::make_flag(true, false, true, true);
        expected_registers.H = 0xD1;
        expected_registers.L = 0xC7;
        expected_registers.A = 0x00;
        set_expected_pc_increase(2);

        const auto cycle = gen::INSTRUCTION_FUNCTIONS[instruction_index](arguments, registers, controller);

        EXPECT_EQ(8, cycle);
    }

    TEST_F(Adc8BitsTestFixture, ADC8BitsImmediateCarryAlreadyPresent)
    {
        // Test from Chapter 4: page 102
        uint16_t instruction_index = 0xCE;
        registers.F = emulator::memory::make_flag(false, false, false, true);
        registers.H = 0xD1;
        registers.L = 0xC7;
        registers.A = 0x3C;
        arguments.uint8 = 0x12;
        expected_registers.F = 0;
        expected_registers.H = 0xD1;
        expected_registers.L = 0xC7;
        expected_registers.A = 0x4F;
        set_expected_pc_increase(2);

        const auto cycle = gen::INSTRUCTION_FUNCTIONS[instruction_index](arguments, registers, controller);

        EXPECT_EQ(8, cycle);
    }

    TEST_F(Adc8BitsTestFixture, ADC8BitsImmediateHalfCarryAndCarryAlreadyPresent)
    {
        // Test from Chapter 4: page 102
        uint16_t instruction_index = 0xCE;
        registers.F = emulator::memory::make_flag(false, false, false, true);
        registers.H = 0xD1;
        registers.L = 0xC7;
        registers.A = 0x18;
        arguments.uint8 = 0x5F;
        expected_registers.F = emulator::memory::make_flag(false, false, true, false);
        expected_registers.H = 0xD1;
        expected_registers.L = 0xC7;
        expected_registers.A = 0x78;
        set_expected_pc_increase(2);

        const auto cycle = gen::INSTRUCTION_FUNCTIONS[instruction_index](arguments, registers, controller);

        EXPECT_EQ(8, cycle);
    }

    TEST_F(Adc8BitsTestFixture, ADC8BitsImmediateCarryAndCarryAlreadyPresent)
    {
        // Test from Chapter 4: page 102
        uint16_t instruction_index = 0xCE;
        registers.F = emulator::memory::make_flag(false, false, false, true);
        registers.H = 0xD1;
        registers.L = 0xC7;
        registers.A = 0x3C;
        arguments.uint8 = 0xFF;
        expected_registers.F = emulator::memory::make_flag(false, false, true, true);
        expected_registers.H = 0xD1;
        expected_registers.L = 0xC7;
        expected_registers.A = 0x3C;
        set_expected_pc_increase(2);

        const auto cycle = gen::INSTRUCTION_FUNCTIONS[instruction_index](arguments, registers, controller);

        EXPECT_EQ(8, cycle);
    }

    TEST_F(Adc8BitsTestFixture, ADC8BitsImmediateZeroAndCarryAlreadyPresent)
    {
        // Test from Chapter 4: page 102
        uint16_t instruction_index = 0xCE;
        registers.F = emulator::memory::make_flag(false, false, false, true);
        registers.H = 0xD1;
        registers.L = 0xC7;
        registers.A = 0x81;
        arguments.uint8 = 0x7E;
        expected_registers.F = emulator::memory::make_flag(true, false, true, true);
        expected_registers.H = 0xD1;
        expected_registers.L = 0xC7;
        expected_registers.A = 0x00;
        set_expected_pc_increase(2);

        const auto cycle = gen::INSTRUCTION_FUNCTIONS[instruction_index](arguments, registers, controller);

        EXPECT_EQ(8, cycle);
    }
}
