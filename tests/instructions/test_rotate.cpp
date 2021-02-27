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
}
