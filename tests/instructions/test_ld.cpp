/*
Testing all the ld functions

The pages referenced in this file are pointing to [GameBoyProgManVer1.1.pdf](https://ia803208.us.archive.org/9/items/GameBoyProgManVer1.1/GameBoyProgManVer1.1.pdf)
 */

#include "gtest/gtest.h"

#include "emulator/registers.h"
#include "emulator/generated/instructions.h"

#include "../mocks/romram_controller.h"

using emulator::generated::Arguments;
using emulator::Registers;
using emulator::mocks::MockMemoryController;

namespace gen = emulator::generated;

namespace
{
    TEST(LDHLTest, LDHL0F8)
    {
        // Test from Chapter 4: page 101
        MockMemoryController controller;
        Registers registers{};
        registers.F = emulator::make_flag(true, true, true, true);
        registers.SP = 0xFFF8;
        Arguments arguments{};
        arguments.int8 = 2;

        const auto value = gen::ldhl_0f8(arguments, registers, controller);

        EXPECT_EQ (12, value);
        EXPECT_EQ (false, registers.get_carry_flag());
        EXPECT_EQ (false, registers.get_half_carry_flag());
        EXPECT_EQ (false, registers.get_add_sub_flag());
        EXPECT_EQ (false, registers.get_zero_flag());
        EXPECT_EQ (0xFFFA, registers.get_HL());

    }
}

