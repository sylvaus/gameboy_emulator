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
    TEST(LDTest, LD008)
    {
        // Test from Chapter 4: page 101
        MockMemoryController controller;
        Registers registers{};
        registers.set_HL(0x3A5B);
        registers.SP = 0xFFF8;
        Arguments arguments{};
        arguments.uint16 = 0xC100;

        EXPECT_CALL (controller, set(0xC100, 0xF8)).Times(1);
        EXPECT_CALL (controller, set(0xC101, 0xFF)).Times(1);

        const auto cycle = gen::ld_008(arguments, registers, controller);

        EXPECT_EQ (20, cycle);
    }

    TEST(LDTest, LD040)
    {
        // Test from Chapter 4: page 95
        MockMemoryController controller;
        Registers registers{};
        registers.A = 0x12;
        registers.F = 0b1100'0000;
        registers.B = 0x26;
        registers.C = 0x81;
        Arguments arguments{};

        const auto cycle = gen::ld_040(arguments, registers, controller);

        EXPECT_EQ (0x12, registers.A);
        EXPECT_EQ (0b1100'0000, registers.F);
        EXPECT_EQ (0x26, registers.B);
        EXPECT_EQ (0x81, registers.C);
        EXPECT_EQ (4, cycle);
    }

    TEST(LDTest, LD041)
    {
        // Test from Chapter 4: page 95
        MockMemoryController controller;
        Registers registers{};
        registers.A = 0x12;
        registers.F = 0b1100'0000;
        registers.B = 0x26;
        registers.C = 0x81;
        Arguments arguments{};

        const auto cycle = gen::ld_041(arguments, registers, controller);

        EXPECT_EQ (0x12, registers.A);
        EXPECT_EQ (0b1100'0000, registers.F);
        EXPECT_EQ (0x81, registers.B);
        EXPECT_EQ (0x81, registers.C);
        EXPECT_EQ (4, cycle);
    }

    TEST(LDHLTest, LDHL0F8)
    {
        // Test from Chapter 4: page 101
        MockMemoryController controller;
        Registers registers{};
        registers.F = emulator::make_flag(true, true, true, true);
        registers.SP = 0xFFF8;
        Arguments arguments{};
        arguments.int8 = 2;

        const auto cycle = gen::ldhl_0f8(arguments, registers, controller);

        EXPECT_EQ (12, cycle);
        EXPECT_EQ (false, registers.get_carry_flag());
        EXPECT_EQ (false, registers.get_half_carry_flag());
        EXPECT_EQ (false, registers.get_add_sub_flag());
        EXPECT_EQ (false, registers.get_zero_flag());
        EXPECT_EQ (0xFFFA, registers.get_HL());
    }

    TEST(LDTest, LD0F9)
    {
        // Test from Chapter 4: page 100
        MockMemoryController controller;
        Registers registers{};
        registers.set_HL(0x3A5B);
        Arguments arguments{};

        const auto cycle = gen::ld_0f9(arguments, registers, controller);

        EXPECT_EQ (8, cycle);
        EXPECT_EQ (0x3A5B, registers.SP);
    }
}

