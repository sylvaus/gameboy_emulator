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
using emulator::instructions::fixtures::InstructionTest;

namespace gen = emulator::generated;

namespace
{
    class IncTest: public InstructionTest {};

    TEST_F(IncTest, INC003NoCarry) // 0x3 INC BC
    {
        // Test from Chapter 4: page 107
        registers.set_BC(0xA8F1);
        expected_registers.set_BC(0xA8F2);
        set_expected_pc_increase(1);

        const auto cycle = gen::inc_003(arguments, registers, controller);

        EXPECT_EQ(8, cycle);
    }
}
