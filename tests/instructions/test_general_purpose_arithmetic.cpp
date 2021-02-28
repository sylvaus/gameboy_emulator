/*
Testing all the inc functions

The pages referenced in this file are pointing to [GameBoyProgManVer1.1.pdf](https://ia803208.us.archive.org/9/items/GameBoyProgManVer1.1/GameBoyProgManVer1.1.pdf)
 */

#include "gtest/gtest.h"

#include "emulator/memory/registers.h"
#include "emulator/generated/instructions.h"

#include "fixtures/instruction.h"

using emulator::generated::Arguments;
using emulator::memory::Registers;
using emulator::instructions::fixtures::InstructionTestFixture;

namespace gen = emulator::generated;

namespace
{
    class GeneralPurposeArithmeticTestFixture: public InstructionTestFixture {};

    TEST_F(GeneralPurposeArithmeticTestFixture, NOP)
    {
        // Test from Chapter 4: page 122
        set_expected_pc_increase(1);

        const auto cycle = gen::INSTRUCTION_FUNCTIONS[0x00](arguments, registers, controller);

        EXPECT_EQ(4, cycle);
    }

    TEST_F(GeneralPurposeArithmeticTestFixture, CPL)
    {
        // Test from Chapter 4: page 122
        registers.A = 0x35;
        registers.F = emulator::memory::make_flag(false, false, false, false);
        expected_registers.A = 0xCA;
        expected_registers.F = emulator::memory::make_flag(false, true, true, false);

        set_expected_pc_increase(1);

        const auto cycle = gen::INSTRUCTION_FUNCTIONS[0x2F](arguments, registers, controller);

        EXPECT_EQ(4, cycle);
    }

    TEST_F(GeneralPurposeArithmeticTestFixture, SCF)
    {
        // Test from Chapter 4: page 123
        registers.F = emulator::memory::make_flag(false, true, true, false);
        expected_registers.F = emulator::memory::make_flag(false, false, false, true);

        set_expected_pc_increase(1);

        const auto cycle = gen::INSTRUCTION_FUNCTIONS[0x37](arguments, registers, controller);

        EXPECT_EQ(4, cycle);
    }
}

