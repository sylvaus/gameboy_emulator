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
using emulator::instructions::REGISTER_HL_INDEX;
using emulator::instructions::REGISTER_16_BITS_VALUES_WITHOUT_HL;
using emulator::instructions::REGISTER_16_BITS_VALUE_SETTER_MAP;
using emulator::instructions::REGISTER_16_BITS_VALUE_NAME_MAP;
using emulator::instructions::NameMapPrinter;
using emulator::instructions::fixtures::InstructionTestFixture;

namespace gen = emulator::generated;

namespace
{
    class Add16BitsTestFixture: public InstructionTestFixture, public ::testing::WithParamInterface<uint16_t> {};

    TEST_P(Add16BitsTestFixture, ADD16Bits)
    {
        uint16_t register_index = GetParam();
        uint16_t instruction_index = (register_index << 4) + 0b1001;
        uint16_t hl_value = 0x456;
        uint16_t other_register_value = 0x123;
        // Test from Chapter 4: page 107
        registers.F = emulator::memory::make_flag(true, true, false, false);
        REGISTER_16_BITS_VALUE_SETTER_MAP.at(register_index)(registers, other_register_value);
        registers.set_HL(hl_value);

        expected_registers.F = emulator::memory::make_flag(true, false, false, false);
        REGISTER_16_BITS_VALUE_SETTER_MAP.at(register_index)(expected_registers, other_register_value);
        expected_registers.set_HL(other_register_value + hl_value);
        set_expected_pc_increase(1);

        const auto cycle = gen::INSTRUCTION_FUNCTIONS[instruction_index](arguments, registers, controller);

        EXPECT_EQ(8, cycle);
    }

    TEST_P(Add16BitsTestFixture, ADD16BitsHalfCarry)
    {
        uint16_t register_index = GetParam();
        uint16_t instruction_index = (register_index << 4) + 0b1001;
        uint16_t hl_value = 0x8A23;
        uint16_t other_register_value = 0x605;
        // Test from Chapter 4: page 107
        registers.F = emulator::memory::make_flag(true, true, false, false);
        REGISTER_16_BITS_VALUE_SETTER_MAP.at(register_index)(registers, other_register_value);
        registers.set_HL(hl_value);

        REGISTER_16_BITS_VALUE_SETTER_MAP.at(register_index)(expected_registers, other_register_value);
        expected_registers.set_HL(other_register_value + hl_value);
        expected_registers.F = emulator::memory::make_flag(true, false, true, false);

        set_expected_pc_increase(1);

        const auto cycle = gen::INSTRUCTION_FUNCTIONS[instruction_index](arguments, registers, controller);

        EXPECT_EQ(8, cycle);
    }

    TEST_P(Add16BitsTestFixture, ADD16BitsCarry)
    {
        uint16_t register_index = GetParam();
        uint16_t instruction_index = (register_index << 4) + 0b1001;
        uint16_t hl_value = 0x8000;
        uint16_t other_register_value = 0x9000;
        // Test from Chapter 4: page 107
        registers.F = emulator::memory::make_flag(true, true, false, false);
        REGISTER_16_BITS_VALUE_SETTER_MAP.at(register_index)(registers, other_register_value);
        registers.set_HL(hl_value);

        expected_registers.F = emulator::memory::make_flag(true, false, false, true);
        REGISTER_16_BITS_VALUE_SETTER_MAP.at(register_index)(expected_registers, other_register_value);
        expected_registers.set_HL(0x1000);
        set_expected_pc_increase(1);

        const auto cycle = gen::INSTRUCTION_FUNCTIONS[instruction_index](arguments, registers, controller);

        EXPECT_EQ(8, cycle);
    }

    INSTANTIATE_TEST_SUITE_P(
        Add16BitsTest, Add16BitsTestFixture, REGISTER_16_BITS_VALUES_WITHOUT_HL, NameMapPrinter(REGISTER_16_BITS_VALUE_NAME_MAP)
    );

    TEST_F(Add16BitsTestFixture, ADD16BitsRegisterHL)
    {
        uint16_t instruction_index = (REGISTER_HL_INDEX << 4) + 0b1001;
        // Test from Chapter 4: page 107
        registers.F = emulator::memory::make_flag(true, true, false, false);
        registers.set_HL(0x456);
        expected_registers.F = emulator::memory::make_flag(true, false, false, false);
        expected_registers.set_HL(2 * 0x456);
        set_expected_pc_increase(1);

        const auto cycle = gen::INSTRUCTION_FUNCTIONS[instruction_index](arguments, registers, controller);

        EXPECT_EQ(8, cycle);
    }

    TEST_F(Add16BitsTestFixture, ADD16BitsHalfCarryRegisterHL)
    {
        uint16_t instruction_index = (REGISTER_HL_INDEX << 4) + 0b1001;
        // Test from Chapter 4: page 107
        registers.F = emulator::memory::make_flag(true, true, false, false);
        registers.set_HL(0x8A23);
        expected_registers.set_HL(0x1446);
        expected_registers.F = emulator::memory::make_flag(true, false, true, true);

        set_expected_pc_increase(1);

        const auto cycle = gen::INSTRUCTION_FUNCTIONS[instruction_index](arguments, registers, controller);

        EXPECT_EQ(8, cycle);
    }

    TEST_F(Add16BitsTestFixture, ADD16BitsCarryRegisterHL)
    {
        uint16_t instruction_index = (REGISTER_HL_INDEX << 4) + 0b1001;
        // Test from Chapter 4: page 107
        registers.F = emulator::memory::make_flag(true, true, false, false);
        registers.set_HL(0x8000);
        expected_registers.F = emulator::memory::make_flag(true, false, false, true);
        expected_registers.set_HL(0x0);
        set_expected_pc_increase(1);

        const auto cycle = gen::INSTRUCTION_FUNCTIONS[instruction_index](arguments, registers, controller);

        EXPECT_EQ(8, cycle);
    }

    class Add8BitsTestFixture: public InstructionTestFixture, public ::testing::WithParamInterface<uint16_t> {};

    TEST_P(Add8BitsTestFixture, ADD8Bits)
    {
        uint16_t register_index = GetParam();
        uint16_t instruction_index = register_index + 0b10'000'000;
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

    TEST_P(Add8BitsTestFixture, ADD8BitsHalfCarry)
    {
        uint16_t register_index = GetParam();
        uint16_t instruction_index = register_index + 0b10'000'000;
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

    TEST_P(Add8BitsTestFixture, ADD8BitsCarry)
    {
        uint16_t register_index = GetParam();
        uint16_t instruction_index = register_index + 0b10'000'000;
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

    TEST_P(Add8BitsTestFixture, ADD8BitsZero)
    {
        uint16_t register_index = GetParam();
        uint16_t instruction_index = register_index + 0b10'000'000;
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

    INSTANTIATE_TEST_SUITE_P(
        Add8BitsTest, Add8BitsTestFixture, REGISTER_8_BITS_VALUES_WITHOUT_A, NameMapPrinter(REGISTER_8_BITS_VALUE_NAME_MAP)
    );

    TEST_F(Add8BitsTestFixture, ADD8RegisterABits)
    {
        uint16_t instruction_index = REGISTER_A_INDEX + 0b10'000'000;
        // Test from Chapter 4: page 102
        registers.F = 0;
        registers.A = 0x45;
        expected_registers.F = 0;
        expected_registers.A = 0x8A;

        set_expected_pc_increase(1);

        const auto cycle = gen::INSTRUCTION_FUNCTIONS[instruction_index](arguments, registers, controller);

        EXPECT_EQ(4, cycle);
    }

    TEST_F(Add8BitsTestFixture, ADD8BitsRegisterAHalfCarry)
    {
        uint16_t instruction_index = REGISTER_A_INDEX + 0b10'000'000;
        // Test from Chapter 4: page 102
        registers.F = 0;
        registers.A = 0xA;
        expected_registers.F = emulator::memory::make_flag(false, false, true, false);
        expected_registers.A = 0x14;

        set_expected_pc_increase(1);

        const auto cycle = gen::INSTRUCTION_FUNCTIONS[instruction_index](arguments, registers, controller);

        EXPECT_EQ(4, cycle);
    }

    TEST_F(Add8BitsTestFixture, ADD8BitsRegisterACarry)
    {
        uint16_t instruction_index = REGISTER_A_INDEX + 0b10'000'000;
        // Test from Chapter 4: page 102
        registers.F = 0;
        registers.A = 0xC0;
        expected_registers.F = emulator::memory::make_flag(false, false, false, true);
        expected_registers.A = 0x80;

        set_expected_pc_increase(1);

        const auto cycle = gen::INSTRUCTION_FUNCTIONS[instruction_index](arguments, registers, controller);

        EXPECT_EQ(4, cycle);
    }

    TEST_F(Add8BitsTestFixture, ADD8BitsRegisterAZero)
    {
        uint16_t instruction_index = REGISTER_A_INDEX + 0b10'000'000;
        // Test from Chapter 4: page 102
        registers.F = 0;
        registers.A = 0x00;
        expected_registers.F = emulator::memory::make_flag(true, false, false, false);
        expected_registers.A = 0x00;

        set_expected_pc_increase(1);

        const auto cycle = gen::INSTRUCTION_FUNCTIONS[instruction_index](arguments, registers, controller);

        EXPECT_EQ(4, cycle);
    }

    TEST_F(Add8BitsTestFixture, ADD8BitsAddress)
    {
        // Test from Chapter 4: page 102
        uint16_t instruction_index = 0b10000110;
        registers.F = 0xF0;
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

    TEST_F(Add8BitsTestFixture, ADD8BitsAddressHalfCarry)
    {
        // Test from Chapter 4: page 102
        uint16_t instruction_index = 0b10000110;
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

    TEST_F(Add8BitsTestFixture, ADD8BitsAddressCarry)
    {
        // Test from Chapter 4: page 102
        uint16_t instruction_index = 0b10000110;
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

    TEST_F(Add8BitsTestFixture, ADD8BitsAddressZero)
    {
        // Test from Chapter 4: page 102
        uint16_t instruction_index = 0b10000110;
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

    TEST_F(Add8BitsTestFixture, ADD8BitsImmediate)
    {
        // Test from Chapter 4: page 102
        uint16_t instruction_index = 0b1100'0110;
        registers.F = 0xF0;
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

    TEST_F(Add8BitsTestFixture, ADD8BitsImmediateHalfCarry)
    {
        // Test from Chapter 4: page 102
        uint16_t instruction_index = 0b1100'0110;
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

    TEST_F(Add8BitsTestFixture, ADD8BitsImmediateCarry)
    {
        // Test from Chapter 4: page 102
        uint16_t instruction_index = 0b1100'0110;
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

    TEST_F(Add8BitsTestFixture, ADD8BitsImmediateZero)
    {
        // Test from Chapter 4: page 102
        uint16_t instruction_index = 0b1100'0110;
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
}
