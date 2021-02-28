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
using emulator::instructions::fixtures::InstructionTestFixture;

namespace gen = emulator::generated;

namespace
{
    class CpuControlTestFixture: public InstructionTestFixture{};

    TEST_F(CpuControlTestFixture, STOP)
    {
        // Test from Chapter 4: page 124
        registers.stopped = false;
        expected_registers.stopped = true;
        set_expected_pc_increase(2);

        const auto cycle = gen::INSTRUCTION_FUNCTIONS[0b10000](arguments, registers, controller);

        EXPECT_EQ(4, cycle);
    }

    TEST_F(CpuControlTestFixture, HALT)
    {
        // Test from Chapter 4: page 124
        registers.halted = false;
        expected_registers.halted = true;
        set_expected_pc_increase(1);

        const auto cycle = gen::INSTRUCTION_FUNCTIONS[0x76](arguments, registers, controller);

        EXPECT_EQ(4, cycle);
    }
}