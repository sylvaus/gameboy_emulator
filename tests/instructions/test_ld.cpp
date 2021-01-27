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

using ::testing::Return;

namespace
{
    TEST(LDTest, LD008) // 0x8 LD (a16), SP
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

    TEST(LDTest, LD040) // 0x40 LD B, B
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

    TEST(LDTest, LD041) // 0x41 LD B, C
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

    TEST(LDTest, LD042) // 0x42 LD B, D
    {
        // Test from Chapter 4: page 95
        MockMemoryController controller;
        Registers registers{};
        registers.A = 0x12;
        registers.F = 0b1100'0000;
        registers.B = 0x26;
        registers.C = 0x81;
        registers.D = 0x19;
        registers.E = 0x97;
        registers.H = 0xF7;
        registers.L = 0x1A;
        Arguments arguments{};

        const auto cycle = gen::ld_042(arguments, registers, controller);

        EXPECT_EQ (0x12, registers.A);
        EXPECT_EQ (0b1100'0000, registers.F);
        EXPECT_EQ (0x19, registers.B);
        EXPECT_EQ (0x81, registers.C);
        EXPECT_EQ (0x19, registers.D);
        EXPECT_EQ (0x97, registers.E);
        EXPECT_EQ (0xF7, registers.H);
        EXPECT_EQ (0x1A, registers.L);
        EXPECT_EQ (4, cycle);
    }

    TEST(LDTest, LD043) // 0x43 LD B, E
    {
        // Test from Chapter 4: page 95
        MockMemoryController controller;
        Registers registers{};
        registers.A = 0x12;
        registers.F = 0b1100'0000;
        registers.B = 0x26;
        registers.C = 0x81;
        registers.D = 0x19;
        registers.E = 0x97;
        registers.H = 0xF7;
        registers.L = 0x1A;
        Arguments arguments{};

        const auto cycle = gen::ld_043(arguments, registers, controller);

        EXPECT_EQ (0x12, registers.A);
        EXPECT_EQ (0b1100'0000, registers.F);
        EXPECT_EQ (0x97, registers.B);
        EXPECT_EQ (0x81, registers.C);
        EXPECT_EQ (0x19, registers.D);
        EXPECT_EQ (0x97, registers.E);
        EXPECT_EQ (0xF7, registers.H);
        EXPECT_EQ (0x1A, registers.L);
        EXPECT_EQ (4, cycle);
    }

    TEST(LDTest, LD044)  // 0x44 LD B, H
    {
        // Test from Chapter 4: page 95
        MockMemoryController controller;
        Registers registers{};
        registers.A = 0x12;
        registers.F = 0b1100'0000;
        registers.B = 0x26;
        registers.C = 0x81;
        registers.D = 0x19;
        registers.E = 0x97;
        registers.H = 0xF7;
        registers.L = 0x1A;
        Arguments arguments{};

        const auto cycle = gen::ld_044(arguments, registers, controller);

        EXPECT_EQ (0x12, registers.A);
        EXPECT_EQ (0b1100'0000, registers.F);
        EXPECT_EQ (0xF7, registers.B);
        EXPECT_EQ (0x81, registers.C);
        EXPECT_EQ (0x19, registers.D);
        EXPECT_EQ (0x97, registers.E);
        EXPECT_EQ (0xF7, registers.H);
        EXPECT_EQ (0x1A, registers.L);
        EXPECT_EQ (4, cycle);
    }

    TEST(LDTest, LD045)  // 0x45 LD B, L
    {
        // Test from Chapter 4: page 95
        MockMemoryController controller;
        Registers registers{};
        registers.A = 0x12;
        registers.F = 0b1100'0000;
        registers.B = 0x26;
        registers.C = 0x81;
        registers.D = 0x19;
        registers.E = 0x97;
        registers.H = 0xF7;
        registers.L = 0x1A;
        Arguments arguments{};

        const auto cycle = gen::ld_045(arguments, registers, controller);

        EXPECT_EQ (0x12, registers.A);
        EXPECT_EQ (0b1100'0000, registers.F);
        EXPECT_EQ (0x1A, registers.B);
        EXPECT_EQ (0x81, registers.C);
        EXPECT_EQ (0x19, registers.D);
        EXPECT_EQ (0x97, registers.E);
        EXPECT_EQ (0xF7, registers.H);
        EXPECT_EQ (0x1A, registers.L);
        EXPECT_EQ (4, cycle);
    }

    TEST(LDTest, LD046)  // 0x46 LD B, (HL)
    {
        // Test from Chapter 4: page 95
        MockMemoryController controller;
        Registers registers{};
        registers.A = 0x12;
        registers.F = 0b1100'0000;
        registers.B = 0x26;
        registers.C = 0x81;
        registers.D = 0x19;
        registers.E = 0x97;
        registers.H = 0xF7;
        registers.L = 0x1A;
        Arguments arguments{};

        EXPECT_CALL (controller, get(0xF71A)).Times(1).WillOnce(Return(0xAC));

        const auto cycle = gen::ld_046(arguments, registers, controller);

        EXPECT_EQ (0x12, registers.A);
        EXPECT_EQ (0b1100'0000, registers.F);
        EXPECT_EQ (0xAC, registers.B);
        EXPECT_EQ (0x81, registers.C);
        EXPECT_EQ (0x19, registers.D);
        EXPECT_EQ (0x97, registers.E);
        EXPECT_EQ (0xF7, registers.H);
        EXPECT_EQ (0x1A, registers.L);
        EXPECT_EQ (8, cycle);
    }

    TEST(LDTest, LD047) // 0x47 LD B, A
    {
        // Test from Chapter 4: page 95
        MockMemoryController controller;
        Registers registers{};
        registers.A = 0x12;
        registers.F = 0b1100'0000;
        registers.B = 0x26;
        registers.C = 0x81;
        registers.D = 0x19;
        registers.E = 0x97;
        registers.H = 0xF7;
        registers.L = 0x1A;
        Arguments arguments{};

        const auto cycle = gen::ld_047(arguments, registers, controller);

        EXPECT_EQ (0x12, registers.A);
        EXPECT_EQ (0b1100'0000, registers.F);
        EXPECT_EQ (0x12, registers.B);
        EXPECT_EQ (0x81, registers.C);
        EXPECT_EQ (0x19, registers.D);
        EXPECT_EQ (0x97, registers.E);
        EXPECT_EQ (0xF7, registers.H);
        EXPECT_EQ (0x1A, registers.L);
        EXPECT_EQ (4, cycle);
    }

    TEST(LDTest, LD048) // 0x48 LD C, B
    {
        // Test from Chapter 4: page 95
        MockMemoryController controller;
        Registers registers{};
        registers.A = 0x12;
        registers.F = 0b1100'0000;
        registers.B = 0x26;
        registers.C = 0x81;
        registers.D = 0x19;
        registers.E = 0x97;
        registers.H = 0xF7;
        registers.L = 0x1A;
        Arguments arguments{};

        const auto cycle = gen::ld_048(arguments, registers, controller);

        EXPECT_EQ (0x12, registers.A);
        EXPECT_EQ (0b1100'0000, registers.F);
        EXPECT_EQ (0x26, registers.B);
        EXPECT_EQ (0x26, registers.C);
        EXPECT_EQ (0x19, registers.D);
        EXPECT_EQ (0x97, registers.E);
        EXPECT_EQ (0xF7, registers.H);
        EXPECT_EQ (0x1A, registers.L);
        EXPECT_EQ (4, cycle);
    }

    TEST(LDTest, LD049) // 0x49 LD C, C
    {
        // Test from Chapter 4: page 95
        MockMemoryController controller;
        Registers registers{};
        registers.A = 0x12;
        registers.F = 0b1100'0000;
        registers.B = 0x26;
        registers.C = 0x81;
        registers.D = 0x19;
        registers.E = 0x97;
        registers.H = 0xF7;
        registers.L = 0x1A;
        Arguments arguments{};

        const auto cycle = gen::ld_049(arguments, registers, controller);

        EXPECT_EQ (0x12, registers.A);
        EXPECT_EQ (0b1100'0000, registers.F);
        EXPECT_EQ (0x26, registers.B);
        EXPECT_EQ (0x81, registers.C);
        EXPECT_EQ (0x19, registers.D);
        EXPECT_EQ (0x97, registers.E);
        EXPECT_EQ (0xF7, registers.H);
        EXPECT_EQ (0x1A, registers.L);
        EXPECT_EQ (4, cycle);
    }

    TEST(LDTest, LD04A) // 0x4A LD C, D
    {
        // Test from Chapter 4: page 95
        MockMemoryController controller;
        Registers registers{};
        registers.A = 0x12;
        registers.F = 0b1100'0000;
        registers.B = 0x26;
        registers.C = 0x81;
        registers.D = 0x19;
        registers.E = 0x97;
        registers.H = 0xF7;
        registers.L = 0x1A;
        Arguments arguments{};

        const auto cycle = gen::ld_04a(arguments, registers, controller);

        EXPECT_EQ (0x12, registers.A);
        EXPECT_EQ (0b1100'0000, registers.F);
        EXPECT_EQ (0x26, registers.B);
        EXPECT_EQ (0x19, registers.C);
        EXPECT_EQ (0x19, registers.D);
        EXPECT_EQ (0x97, registers.E);
        EXPECT_EQ (0xF7, registers.H);
        EXPECT_EQ (0x1A, registers.L);
        EXPECT_EQ (4, cycle);
    }

    TEST(LDTest, LD04B) // 0x4B LD C, E
    {
        // Test from Chapter 4: page 95
        MockMemoryController controller;
        Registers registers{};
        registers.A = 0x12;
        registers.F = 0b1100'0000;
        registers.B = 0x26;
        registers.C = 0x81;
        registers.D = 0x19;
        registers.E = 0x97;
        registers.H = 0xF7;
        registers.L = 0x1A;
        Arguments arguments{};

        const auto cycle = gen::ld_04b(arguments, registers, controller);

        EXPECT_EQ (0x12, registers.A);
        EXPECT_EQ (0b1100'0000, registers.F);
        EXPECT_EQ (0x26, registers.B);
        EXPECT_EQ (0x97, registers.C);
        EXPECT_EQ (0x19, registers.D);
        EXPECT_EQ (0x97, registers.E);
        EXPECT_EQ (0xF7, registers.H);
        EXPECT_EQ (0x1A, registers.L);
        EXPECT_EQ (4, cycle);
    }

    TEST(LDTest, LD04C) // 0x4C LD C, H
    {
        // Test from Chapter 4: page 95
        MockMemoryController controller;
        Registers registers{};
        registers.A = 0x12;
        registers.F = 0b1100'0000;
        registers.B = 0x26;
        registers.C = 0x81;
        registers.D = 0x19;
        registers.E = 0x97;
        registers.H = 0xF7;
        registers.L = 0x1A;
        Arguments arguments{};

        const auto cycle = gen::ld_04c(arguments, registers, controller);

        EXPECT_EQ (0x12, registers.A);
        EXPECT_EQ (0b1100'0000, registers.F);
        EXPECT_EQ (0x26, registers.B);
        EXPECT_EQ (0xF7, registers.C);
        EXPECT_EQ (0x19, registers.D);
        EXPECT_EQ (0x97, registers.E);
        EXPECT_EQ (0xF7, registers.H);
        EXPECT_EQ (0x1A, registers.L);
        EXPECT_EQ (4, cycle);
    }

    TEST(LDTest, LD04D) // 0x4D LD C, L
    {
        // Test from Chapter 4: page 95
        MockMemoryController controller;
        Registers registers{};
        registers.A = 0x12;
        registers.F = 0b1100'0000;
        registers.B = 0x26;
        registers.C = 0x81;
        registers.D = 0x19;
        registers.E = 0x97;
        registers.H = 0xF7;
        registers.L = 0x1A;
        Arguments arguments{};

        const auto cycle = gen::ld_04d(arguments, registers, controller);

        EXPECT_EQ (0x12, registers.A);
        EXPECT_EQ (0b1100'0000, registers.F);
        EXPECT_EQ (0x26, registers.B);
        EXPECT_EQ (0x1A, registers.C);
        EXPECT_EQ (0x19, registers.D);
        EXPECT_EQ (0x97, registers.E);
        EXPECT_EQ (0xF7, registers.H);
        EXPECT_EQ (0x1A, registers.L);
        EXPECT_EQ (4, cycle);
    }

    TEST(LDTest, LD04E) // 0x4E LD C, (HL)
    {
        // Test from Chapter 4: page 95
        MockMemoryController controller;
        Registers registers{};
        registers.A = 0x12;
        registers.F = 0b1100'0000;
        registers.B = 0x26;
        registers.C = 0x81;
        registers.D = 0x19;
        registers.E = 0x97;
        registers.H = 0x27;
        registers.L = 0x8A;
        Arguments arguments{};

        EXPECT_CALL (controller, get(0x278A)).Times(1).WillOnce(Return(0x7D));

        const auto cycle = gen::ld_04e(arguments, registers, controller);

        EXPECT_EQ (0x12, registers.A);
        EXPECT_EQ (0b1100'0000, registers.F);
        EXPECT_EQ (0x26, registers.B);
        EXPECT_EQ (0x7D, registers.C);
        EXPECT_EQ (0x19, registers.D);
        EXPECT_EQ (0x97, registers.E);
        EXPECT_EQ (0x27, registers.H);
        EXPECT_EQ (0x8A, registers.L);
        EXPECT_EQ (8, cycle);
    }

    TEST(LDTest, LD04F) // 0x4F LD C, A
    {
        // Test from Chapter 4: page 95
        MockMemoryController controller;
        Registers registers{};
        registers.A = 0x12;
        registers.F = 0b1100'0000;
        registers.B = 0x26;
        registers.C = 0x81;
        registers.D = 0x19;
        registers.E = 0x97;
        registers.H = 0xF7;
        registers.L = 0x1A;
        Arguments arguments{};

        const auto cycle = gen::ld_04f(arguments, registers, controller);

        EXPECT_EQ (0x12, registers.A);
        EXPECT_EQ (0b1100'0000, registers.F);
        EXPECT_EQ (0x26, registers.B);
        EXPECT_EQ (0x12, registers.C);
        EXPECT_EQ (0x19, registers.D);
        EXPECT_EQ (0x97, registers.E);
        EXPECT_EQ (0xF7, registers.H);
        EXPECT_EQ (0x1A, registers.L);
        EXPECT_EQ (4, cycle);
    }

    TEST(LDTest, LD050) // 0x50 LD D, B
    {
        // Test from Chapter 4: page 95
        MockMemoryController controller;
        Registers registers{};
        registers.A = 0x12;
        registers.F = 0b1100'0000;
        registers.B = 0x26;
        registers.C = 0x81;
        registers.D = 0x19;
        registers.E = 0x97;
        registers.H = 0xF7;
        registers.L = 0x1A;
        Arguments arguments{};

        const auto cycle = gen::ld_050(arguments, registers, controller);

        EXPECT_EQ (0x12, registers.A);
        EXPECT_EQ (0b1100'0000, registers.F);
        EXPECT_EQ (0x26, registers.B);
        EXPECT_EQ (0x81, registers.C);
        EXPECT_EQ (0x26, registers.D);
        EXPECT_EQ (0x97, registers.E);
        EXPECT_EQ (0xF7, registers.H);
        EXPECT_EQ (0x1A, registers.L);
        EXPECT_EQ (4, cycle);
    }

    TEST(LDTest, LD051) // 0x51 LD D, C
    {
        // Test from Chapter 4: page 95
        MockMemoryController controller;
        Registers registers{};
        registers.A = 0x12;
        registers.F = 0b1100'0000;
        registers.B = 0x26;
        registers.C = 0x81;
        registers.D = 0x19;
        registers.E = 0x97;
        registers.H = 0xF7;
        registers.L = 0x1A;
        Arguments arguments{};

        const auto cycle = gen::ld_051(arguments, registers, controller);

        EXPECT_EQ (0x12, registers.A);
        EXPECT_EQ (0b1100'0000, registers.F);
        EXPECT_EQ (0x26, registers.B);
        EXPECT_EQ (0x81, registers.C);
        EXPECT_EQ (0x81, registers.D);
        EXPECT_EQ (0x97, registers.E);
        EXPECT_EQ (0xF7, registers.H);
        EXPECT_EQ (0x1A, registers.L);
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

