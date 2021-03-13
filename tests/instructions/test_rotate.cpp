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
    class RotateTestFixture: public InstructionTestFixture {};

    TEST_F(RotateTestFixture, RLCA)
    {
        // Test from Chapter 4: page 109
        registers.A = 0b01001101;
        registers.F = emulator::memory::make_flag(true, true, true, true);
        expected_registers.A = 0b10011010;
        expected_registers.F = emulator::memory::make_flag(false, false, false, false);

        set_expected_pc_increase(1);

        const auto cycle = gen::INSTRUCTION_FUNCTIONS[0x07](arguments, registers, controller);

        EXPECT_EQ(4, cycle);
    }

    TEST_F(RotateTestFixture, RLCACarry)
    {
        // Test from Chapter 4: page 109
        registers.A = 0x85;
        registers.F = emulator::memory::make_flag(true, true, true, false);
        expected_registers.A = 0x0B;
        expected_registers.F = emulator::memory::make_flag(false, false, false, true);

        set_expected_pc_increase(1);

        const auto cycle = gen::INSTRUCTION_FUNCTIONS[0x07](arguments, registers, controller);

        EXPECT_EQ(4, cycle);
    }

    TEST_F(RotateTestFixture, RLA)
    {
        // Test from Chapter 4: page 109
        registers.A = 0b01001101;
        registers.F = emulator::memory::make_flag(true, true, true, false);
        expected_registers.A = 0b10011010;
        expected_registers.F = emulator::memory::make_flag(false, false, false, false);

        set_expected_pc_increase(1);

        const auto cycle = gen::INSTRUCTION_FUNCTIONS[0b010111](arguments, registers, controller);

        EXPECT_EQ(4, cycle);
    }

    TEST_F(RotateTestFixture, RLACarry)
    {
        // Test from Chapter 4: page 109
        registers.A = 0x85;
        registers.F = emulator::memory::make_flag(true, true, true, false);
        expected_registers.A = 0x0A;
        expected_registers.F = emulator::memory::make_flag(false, false, false, true);

        set_expected_pc_increase(1);

        const auto cycle = gen::INSTRUCTION_FUNCTIONS[0b010111](arguments, registers, controller);

        EXPECT_EQ(4, cycle);
    }

    TEST_F(RotateTestFixture, RLACarryAndCarryAlreadyPresent)
    {
        // Test from Chapter 4: page 109
        registers.A = 0x95;
        registers.F = emulator::memory::make_flag(true, true, true, true);
        expected_registers.A = 0x2B;
        expected_registers.F = emulator::memory::make_flag(false, false, false, true);

        set_expected_pc_increase(1);

        const auto cycle = gen::INSTRUCTION_FUNCTIONS[0b010111](arguments, registers, controller);

        EXPECT_EQ(4, cycle);
    }

    TEST_F(RotateTestFixture, RRCA)
    {
        // Test from Chapter 4: page 109
        registers.A = 0b1001'1010;
        registers.F = emulator::memory::make_flag(true, true, true, true);
        expected_registers.A = 0b0100'1101;
        expected_registers.F = emulator::memory::make_flag(false, false, false, false);

        set_expected_pc_increase(1);

        const auto cycle = gen::INSTRUCTION_FUNCTIONS[0b1111](arguments, registers, controller);

        EXPECT_EQ(4, cycle);
    }

    TEST_F(RotateTestFixture, RRCACarry)
    {
        // Test from Chapter 4: page 109
        registers.A = 0x3B;
        registers.F = emulator::memory::make_flag(true, true, true, false);
        expected_registers.A = 0x9D;
        expected_registers.F = emulator::memory::make_flag(false, false, false, true);

        set_expected_pc_increase(1);

        const auto cycle = gen::INSTRUCTION_FUNCTIONS[0b1111](arguments, registers, controller);

        EXPECT_EQ(4, cycle);
    }

    TEST_F(RotateTestFixture, RRA)
    {
        // Test from Chapter 4: page 109
        registers.A = 0b1001'1010;
        registers.F = emulator::memory::make_flag(true, true, true, false);
        expected_registers.A = 0b0100'1101;
        expected_registers.F = emulator::memory::make_flag(false, false, false, false);

        set_expected_pc_increase(1);

        const auto cycle = gen::INSTRUCTION_FUNCTIONS[0b11111](arguments, registers, controller);

        EXPECT_EQ(4, cycle);
    }

    TEST_F(RotateTestFixture, RRACarry)
    {
        // Test from Chapter 4: page 109
        registers.A = 0x81;
        registers.F = emulator::memory::make_flag(true, true, true, false);
        expected_registers.A = 0x40;
        expected_registers.F = emulator::memory::make_flag(false, false, false, true);

        set_expected_pc_increase(1);

        const auto cycle = gen::INSTRUCTION_FUNCTIONS[0b11111](arguments, registers, controller);

        EXPECT_EQ(4, cycle);
    }

    TEST_F(RotateTestFixture, RRACarryAlreadyPresent)
    {
        // Test from Chapter 4: page 109
        registers.A = 0b1001'1100;
        registers.F = emulator::memory::make_flag(true, true, true, true);
        expected_registers.A = 0b1100'1110;
        expected_registers.F = emulator::memory::make_flag(false, false, false, false);

        set_expected_pc_increase(1);

        const auto cycle = gen::INSTRUCTION_FUNCTIONS[0b11111](arguments, registers, controller);

        EXPECT_EQ(4, cycle);
    }

    class RotateGenericTestFixture: public InstructionTestFixture, public ::testing::WithParamInterface<uint16_t> {};

    TEST_P(RotateGenericTestFixture, RLC)
    {
        // Test from Chapter 4: page 110
        uint16_t register_index = GetParam();
        uint16_t instruction_index = register_index + 0b1'0000'0000;
        REGISTER_8_BITS_VALUE_SETTER_MAP.at(register_index)(registers, 0b0110'1100);
        registers.F = emulator::memory::make_flag(true, true, true, true);
        REGISTER_8_BITS_VALUE_SETTER_MAP.at(register_index)(expected_registers, 0b1101'1000);
        expected_registers.F = emulator::memory::make_flag(false, false, false, false);

        set_expected_pc_increase(2);

        const auto cycle = gen::INSTRUCTION_FUNCTIONS[instruction_index](arguments, registers, controller);

        EXPECT_EQ(8, cycle);
    }

    TEST_P(RotateGenericTestFixture, RLCCarry)
    {
        // Test from Chapter 4: page 110
        uint16_t register_index = GetParam();
        uint16_t instruction_index = register_index + 0b1'0000'0000;
        REGISTER_8_BITS_VALUE_SETTER_MAP.at(register_index)(registers, 0x85);
        registers.F = emulator::memory::make_flag(true, true, true, true);
        REGISTER_8_BITS_VALUE_SETTER_MAP.at(register_index)(expected_registers, 0x0B);
        expected_registers.F = emulator::memory::make_flag(false, false, false, true);

        set_expected_pc_increase(2);

        const auto cycle = gen::INSTRUCTION_FUNCTIONS[instruction_index](arguments, registers, controller);

        EXPECT_EQ(8, cycle);
    }

    TEST_P(RotateGenericTestFixture, RLCZero)
    {
        // Test from Chapter 4: page 110
        uint16_t register_index = GetParam();
        uint16_t instruction_index = register_index + 0b1'0000'0000;
        REGISTER_8_BITS_VALUE_SETTER_MAP.at(register_index)(registers, 0x0);
        registers.F = emulator::memory::make_flag(false, true, true, true);
        REGISTER_8_BITS_VALUE_SETTER_MAP.at(register_index)(expected_registers, 0x0);
        expected_registers.F = emulator::memory::make_flag(true, false, false, false);

        set_expected_pc_increase(2);

        const auto cycle = gen::INSTRUCTION_FUNCTIONS[instruction_index](arguments, registers, controller);

        EXPECT_EQ(8, cycle);
    }

    TEST_P(RotateGenericTestFixture, RRC)
    {
        // Test from Chapter 4: page 111
        uint16_t register_index = GetParam();
        uint16_t instruction_index = register_index + 0b1'0000'1000;
        REGISTER_8_BITS_VALUE_SETTER_MAP.at(register_index)(registers, 0b0110'1100);
        registers.F = emulator::memory::make_flag(true, true, true, true);
        REGISTER_8_BITS_VALUE_SETTER_MAP.at(register_index)(expected_registers, 0b0011'0110);
        expected_registers.F = emulator::memory::make_flag(false, false, false, false);

        set_expected_pc_increase(2);

        const auto cycle = gen::INSTRUCTION_FUNCTIONS[instruction_index](arguments, registers, controller);

        EXPECT_EQ(8, cycle);
    }

    TEST_P(RotateGenericTestFixture, RRCCarry)
    {
        // Test from Chapter 4: page 111
        uint16_t register_index = GetParam();
        uint16_t instruction_index = register_index + 0b1'0000'1000;
        REGISTER_8_BITS_VALUE_SETTER_MAP.at(register_index)(registers, 0x01);
        registers.F = emulator::memory::make_flag(true, true, true, true);
        REGISTER_8_BITS_VALUE_SETTER_MAP.at(register_index)(expected_registers, 0x80);
        expected_registers.F = emulator::memory::make_flag(false, false, false, true);

        set_expected_pc_increase(2);

        const auto cycle = gen::INSTRUCTION_FUNCTIONS[instruction_index](arguments, registers, controller);

        EXPECT_EQ(8, cycle);
    }

    TEST_P(RotateGenericTestFixture, RRCZero)
    {
        // Test from Chapter 4: page 111
        uint16_t register_index = GetParam();
        uint16_t instruction_index = register_index + 0b1'0000'1000;
        REGISTER_8_BITS_VALUE_SETTER_MAP.at(register_index)(registers, 0x0);
        registers.F = emulator::memory::make_flag(false, true, true, true);
        REGISTER_8_BITS_VALUE_SETTER_MAP.at(register_index)(expected_registers, 0x0);
        expected_registers.F = emulator::memory::make_flag(true, false, false, false);

        set_expected_pc_increase(2);

        const auto cycle = gen::INSTRUCTION_FUNCTIONS[instruction_index](arguments, registers, controller);

        EXPECT_EQ(8, cycle);
    }

    INSTANTIATE_TEST_SUITE_P(
        RotateGenericTest, RotateGenericTestFixture, REGISTER_8_BITS_VALUES, NameMapPrinter(REGISTER_8_BITS_VALUE_NAME_MAP)
    );


    TEST_F(RotateGenericTestFixture, RLCAddress)
    {
        // Test from Chapter 4: page 110
        registers.set_HL(0xD1C7);
        EXPECT_CALL (controller, get(0xD1C7)).Times(1).WillOnce(::testing::Return(0b0110'1100));
        registers.F = emulator::memory::make_flag(true, true, true, true);
        expected_registers.set_HL(0xD1C7);
        EXPECT_CALL (controller, set(0xD1C7, 0b1101'1000)).Times(1);
        expected_registers.F = emulator::memory::make_flag(false, false, false, false);

        set_expected_pc_increase(2);

        const auto cycle = gen::INSTRUCTION_FUNCTIONS[0b1'0000'0110](arguments, registers, controller);

        EXPECT_EQ(16, cycle);
    }

    TEST_F(RotateGenericTestFixture, RLCAddressCarry)
    {
        // Test from Chapter 4: page 110
        registers.set_HL(0xD1C7);
        EXPECT_CALL (controller, get(0xD1C7)).Times(1).WillOnce(::testing::Return(0x85));
        registers.F = emulator::memory::make_flag(true, true, true, true);
        expected_registers.set_HL(0xD1C7);
        EXPECT_CALL (controller, set(0xD1C7, 0x0B)).Times(1);
        expected_registers.F = emulator::memory::make_flag(false, false, false, true);

        set_expected_pc_increase(2);

        const auto cycle = gen::INSTRUCTION_FUNCTIONS[0b1'0000'0110](arguments, registers, controller);

        EXPECT_EQ(16, cycle);
    }

    TEST_F(RotateGenericTestFixture, RLCAddressZero)
    {
        // Test from Chapter 4: page 110
        registers.set_HL(0xD1C7);
        EXPECT_CALL (controller, get(0xD1C7)).Times(1).WillOnce(::testing::Return(0x0));
        registers.F = emulator::memory::make_flag(false, true, true, true);
        expected_registers.set_HL(0xD1C7);
        EXPECT_CALL (controller, set(0xD1C7, 0x0)).Times(1);
        expected_registers.F = emulator::memory::make_flag(true, false, false, false);

        set_expected_pc_increase(2);

        const auto cycle = gen::INSTRUCTION_FUNCTIONS[0b1'0000'0110](arguments, registers, controller);

        EXPECT_EQ(16, cycle);
    }

    TEST_F(RotateGenericTestFixture, RRCAddress)
    {
        // Test from Chapter 4: page 111
        registers.set_HL(0xD1C7);
        EXPECT_CALL (controller, get(0xD1C7)).Times(1).WillOnce(::testing::Return(0b0110'1100));
        registers.F = emulator::memory::make_flag(true, true, true, true);
        expected_registers.set_HL(0xD1C7);
        EXPECT_CALL (controller, set(0xD1C7, 0b0011'0110)).Times(1);
        expected_registers.F = emulator::memory::make_flag(false, false, false, false);

        set_expected_pc_increase(2);

        const auto cycle = gen::INSTRUCTION_FUNCTIONS[0b1'0000'1110](arguments, registers, controller);

        EXPECT_EQ(16, cycle);
    }

    TEST_F(RotateGenericTestFixture, RRCAddressCarry)
    {
        // Test from Chapter 4: page 111
        registers.set_HL(0xD1C7);
        EXPECT_CALL (controller, get(0xD1C7)).Times(1).WillOnce(::testing::Return(0x01));
        registers.F = emulator::memory::make_flag(true, true, true, true);
        expected_registers.set_HL(0xD1C7);
        EXPECT_CALL (controller, set(0xD1C7, 0x80)).Times(1);
        expected_registers.F = emulator::memory::make_flag(false, false, false, true);

        set_expected_pc_increase(2);

        const auto cycle = gen::INSTRUCTION_FUNCTIONS[0b1'0000'1110](arguments, registers, controller);

        EXPECT_EQ(16, cycle);
    }

    TEST_F(RotateGenericTestFixture, RRCAddressZero)
    {
        // Test from Chapter 4: page 111
        registers.set_HL(0xD1C7);
        EXPECT_CALL (controller, get(0xD1C7)).Times(1).WillOnce(::testing::Return(0x0));
        registers.F = emulator::memory::make_flag(false, true, true, true);
        expected_registers.set_HL(0xD1C7);
        EXPECT_CALL (controller, set(0xD1C7, 0x0)).Times(1);
        expected_registers.F = emulator::memory::make_flag(true, false, false, false);

        set_expected_pc_increase(2);

        const auto cycle = gen::INSTRUCTION_FUNCTIONS[0b1'0000'1110](arguments, registers, controller);

        EXPECT_EQ(16, cycle);
    }
}
