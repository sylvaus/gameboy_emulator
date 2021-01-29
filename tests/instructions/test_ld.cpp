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
        // Test from Chapter 4: page 96
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
        // Test from Chapter 4: page 96
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

    TEST(LDTest, LD052) // 0x52 LD D, D
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

        const auto cycle = gen::ld_052(arguments, registers, controller);

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

    TEST(LDTest, LD053) // 0x53 LD D, E
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

        const auto cycle = gen::ld_053(arguments, registers, controller);

        EXPECT_EQ (0x12, registers.A);
        EXPECT_EQ (0b1100'0000, registers.F);
        EXPECT_EQ (0x26, registers.B);
        EXPECT_EQ (0x81, registers.C);
        EXPECT_EQ (0x97, registers.D);
        EXPECT_EQ (0x97, registers.E);
        EXPECT_EQ (0xF7, registers.H);
        EXPECT_EQ (0x1A, registers.L);
        EXPECT_EQ (4, cycle);
    }

    TEST(LDTest, LD054) // 0x54 LD D, H
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

        const auto cycle = gen::ld_054(arguments, registers, controller);

        EXPECT_EQ (0x12, registers.A);
        EXPECT_EQ (0b1100'0000, registers.F);
        EXPECT_EQ (0x26, registers.B);
        EXPECT_EQ (0x81, registers.C);
        EXPECT_EQ (0xF7, registers.D);
        EXPECT_EQ (0x97, registers.E);
        EXPECT_EQ (0xF7, registers.H);
        EXPECT_EQ (0x1A, registers.L);
        EXPECT_EQ (4, cycle);
    }

    TEST(LDTest, LD055) // 0x55 LD D, L
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

        const auto cycle = gen::ld_055(arguments, registers, controller);

        EXPECT_EQ (0x12, registers.A);
        EXPECT_EQ (0b1100'0000, registers.F);
        EXPECT_EQ (0x26, registers.B);
        EXPECT_EQ (0x81, registers.C);
        EXPECT_EQ (0x1A, registers.D);
        EXPECT_EQ (0x97, registers.E);
        EXPECT_EQ (0xF7, registers.H);
        EXPECT_EQ (0x1A, registers.L);
        EXPECT_EQ (4, cycle);
    }

    TEST(LDTest, LD056) // 0x56 LD D, (HL)
    {
        // Test from Chapter 4: page 96
        MockMemoryController controller;
        Registers registers{};
        registers.A = 0x12;
        registers.F = 0b1100'0000;
        registers.B = 0x26;
        registers.C = 0x81;
        registers.D = 0x19;
        registers.E = 0x97;
        registers.H = 0x89;
        registers.L = 0x21;
        Arguments arguments{};

        EXPECT_CALL (controller, get(0x8921)).Times(1).WillOnce(Return(0x1E));

        const auto cycle = gen::ld_056(arguments, registers, controller);

        EXPECT_EQ (0x12, registers.A);
        EXPECT_EQ (0b1100'0000, registers.F);
        EXPECT_EQ (0x26, registers.B);
        EXPECT_EQ (0x81, registers.C);
        EXPECT_EQ (0x1E, registers.D);
        EXPECT_EQ (0x97, registers.E);
        EXPECT_EQ (0x89, registers.H);
        EXPECT_EQ (0x21, registers.L);
        EXPECT_EQ (8, cycle);
    }

    TEST(LDTest, LD057) // 0x57 LD D, A
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

        const auto cycle = gen::ld_057(arguments, registers, controller);

        EXPECT_EQ (0x12, registers.A);
        EXPECT_EQ (0b1100'0000, registers.F);
        EXPECT_EQ (0x26, registers.B);
        EXPECT_EQ (0x81, registers.C);
        EXPECT_EQ (0x12, registers.D);
        EXPECT_EQ (0x97, registers.E);
        EXPECT_EQ (0xF7, registers.H);
        EXPECT_EQ (0x1A, registers.L);
        EXPECT_EQ (4, cycle);
    }

    TEST(LDTest, LD058) // 0x58 LD E, B
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

        const auto cycle = gen::ld_058(arguments, registers, controller);

        EXPECT_EQ (0x12, registers.A);
        EXPECT_EQ (0b1100'0000, registers.F);
        EXPECT_EQ (0x26, registers.B);
        EXPECT_EQ (0x81, registers.C);
        EXPECT_EQ (0x19, registers.D);
        EXPECT_EQ (0x26, registers.E);
        EXPECT_EQ (0xF7, registers.H);
        EXPECT_EQ (0x1A, registers.L);
        EXPECT_EQ (4, cycle);
    }

    TEST(LDTest, LD059) // 0x59 LD E, C
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

        const auto cycle = gen::ld_059(arguments, registers, controller);

        EXPECT_EQ (0x12, registers.A);
        EXPECT_EQ (0b1100'0000, registers.F);
        EXPECT_EQ (0x26, registers.B);
        EXPECT_EQ (0x81, registers.C);
        EXPECT_EQ (0x19, registers.D);
        EXPECT_EQ (0x81, registers.E);
        EXPECT_EQ (0xF7, registers.H);
        EXPECT_EQ (0x1A, registers.L);
        EXPECT_EQ (4, cycle);
    }

    TEST(LDTest, LD05A) // 0x5A LD E, D
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

        const auto cycle = gen::ld_05a(arguments, registers, controller);

        EXPECT_EQ (0x12, registers.A);
        EXPECT_EQ (0b1100'0000, registers.F);
        EXPECT_EQ (0x26, registers.B);
        EXPECT_EQ (0x81, registers.C);
        EXPECT_EQ (0x19, registers.D);
        EXPECT_EQ (0x19, registers.E);
        EXPECT_EQ (0xF7, registers.H);
        EXPECT_EQ (0x1A, registers.L);
        EXPECT_EQ (4, cycle);
    }

    TEST(LDTest, LD05B) // 0x5B LD E, E
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

        const auto cycle = gen::ld_05b(arguments, registers, controller);

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

    TEST(LDTest, LD05C) // 0x5C LD E, H
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

        const auto cycle = gen::ld_05c(arguments, registers, controller);

        EXPECT_EQ (0x12, registers.A);
        EXPECT_EQ (0b1100'0000, registers.F);
        EXPECT_EQ (0x26, registers.B);
        EXPECT_EQ (0x81, registers.C);
        EXPECT_EQ (0x19, registers.D);
        EXPECT_EQ (0xF7, registers.E);
        EXPECT_EQ (0xF7, registers.H);
        EXPECT_EQ (0x1A, registers.L);
        EXPECT_EQ (4, cycle);
    }

    TEST(LDTest, LD05D) // 0x5D LD E, L
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

        const auto cycle = gen::ld_05d(arguments, registers, controller);

        EXPECT_EQ (0x12, registers.A);
        EXPECT_EQ (0b1100'0000, registers.F);
        EXPECT_EQ (0x26, registers.B);
        EXPECT_EQ (0x81, registers.C);
        EXPECT_EQ (0x19, registers.D);
        EXPECT_EQ (0x1A, registers.E);
        EXPECT_EQ (0xF7, registers.H);
        EXPECT_EQ (0x1A, registers.L);
        EXPECT_EQ (4, cycle);
    }

    TEST(LDTest, LD05E) // 0x5E LD E, (HL)
    {
        // Test from Chapter 4: page 96
        MockMemoryController controller;
        Registers registers{};
        registers.A = 0x12;
        registers.F = 0b1100'0000;
        registers.B = 0x26;
        registers.C = 0x81;
        registers.D = 0x19;
        registers.E = 0x97;
        registers.H = 0xA9;
        registers.L = 0x3C;
        Arguments arguments{};

        EXPECT_CALL (controller, get(0xA93C)).Times(1).WillOnce(Return(0xB3));

        const auto cycle = gen::ld_05e(arguments, registers, controller);

        EXPECT_EQ (0x12, registers.A);
        EXPECT_EQ (0b1100'0000, registers.F);
        EXPECT_EQ (0x26, registers.B);
        EXPECT_EQ (0x81, registers.C);
        EXPECT_EQ (0x19, registers.D);
        EXPECT_EQ (0xB3, registers.E);
        EXPECT_EQ (0xA9, registers.H);
        EXPECT_EQ (0x3C, registers.L);
        EXPECT_EQ (8, cycle);
    }

    TEST(LDTest, LD05F) // 0x5F LD E, A
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

        const auto cycle = gen::ld_05f(arguments, registers, controller);

        EXPECT_EQ (0x12, registers.A);
        EXPECT_EQ (0b1100'0000, registers.F);
        EXPECT_EQ (0x26, registers.B);
        EXPECT_EQ (0x81, registers.C);
        EXPECT_EQ (0x19, registers.D);
        EXPECT_EQ (0x12, registers.E);
        EXPECT_EQ (0xF7, registers.H);
        EXPECT_EQ (0x1A, registers.L);
        EXPECT_EQ (4, cycle);
    }

    TEST(LDTest, LD060) // 0x60 LD H, B
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

        const auto cycle = gen::ld_060(arguments, registers, controller);

        EXPECT_EQ (0x12, registers.A);
        EXPECT_EQ (0b1100'0000, registers.F);
        EXPECT_EQ (0x26, registers.B);
        EXPECT_EQ (0x81, registers.C);
        EXPECT_EQ (0x19, registers.D);
        EXPECT_EQ (0x97, registers.E);
        EXPECT_EQ (0x26, registers.H);
        EXPECT_EQ (0x1A, registers.L);
        EXPECT_EQ (4, cycle);
    }

    TEST(LDTest, LD061) // 0x61 LD H, C
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

        const auto cycle = gen::ld_061(arguments, registers, controller);

        EXPECT_EQ (0x12, registers.A);
        EXPECT_EQ (0b1100'0000, registers.F);
        EXPECT_EQ (0x26, registers.B);
        EXPECT_EQ (0x81, registers.C);
        EXPECT_EQ (0x19, registers.D);
        EXPECT_EQ (0x97, registers.E);
        EXPECT_EQ (0x81, registers.H);
        EXPECT_EQ (0x1A, registers.L);
        EXPECT_EQ (4, cycle);
    }

    TEST(LDTest, LD062) // 0x62 LD H, D
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

        const auto cycle = gen::ld_062(arguments, registers, controller);

        EXPECT_EQ (0x12, registers.A);
        EXPECT_EQ (0b1100'0000, registers.F);
        EXPECT_EQ (0x26, registers.B);
        EXPECT_EQ (0x81, registers.C);
        EXPECT_EQ (0x19, registers.D);
        EXPECT_EQ (0x97, registers.E);
        EXPECT_EQ (0x19, registers.H);
        EXPECT_EQ (0x1A, registers.L);
        EXPECT_EQ (4, cycle);
    }

    TEST(LDTest, LD063) // 0x63 LD H, E
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

        const auto cycle = gen::ld_063(arguments, registers, controller);

        EXPECT_EQ (0x12, registers.A);
        EXPECT_EQ (0b1100'0000, registers.F);
        EXPECT_EQ (0x26, registers.B);
        EXPECT_EQ (0x81, registers.C);
        EXPECT_EQ (0x19, registers.D);
        EXPECT_EQ (0x97, registers.E);
        EXPECT_EQ (0x97, registers.H);
        EXPECT_EQ (0x1A, registers.L);
        EXPECT_EQ (4, cycle);
    }

    TEST(LDTest, LD064) // 0x64 LD H, H
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

        const auto cycle = gen::ld_064(arguments, registers, controller);

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

    TEST(LDTest, LD065) // 0x65 LD H, L
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

        const auto cycle = gen::ld_065(arguments, registers, controller);

        EXPECT_EQ (0x12, registers.A);
        EXPECT_EQ (0b1100'0000, registers.F);
        EXPECT_EQ (0x26, registers.B);
        EXPECT_EQ (0x81, registers.C);
        EXPECT_EQ (0x19, registers.D);
        EXPECT_EQ (0x97, registers.E);
        EXPECT_EQ (0x1A, registers.H);
        EXPECT_EQ (0x1A, registers.L);
        EXPECT_EQ (4, cycle);
    }

    TEST(LDTest, LD066) // 0x66 LD H, (HL)
    {
        // Test from Chapter 4: page 96
        MockMemoryController controller;
        Registers registers{};
        registers.A = 0x12;
        registers.F = 0b1100'0000;
        registers.B = 0x26;
        registers.C = 0x81;
        registers.D = 0x19;
        registers.E = 0x97;
        registers.H = 0x59;
        registers.L = 0x73;
        Arguments arguments{};

        EXPECT_CALL (controller, get(0x5973)).Times(1).WillOnce(Return(0x46));

        const auto cycle = gen::ld_066(arguments, registers, controller);

        EXPECT_EQ (0x12, registers.A);
        EXPECT_EQ (0b1100'0000, registers.F);
        EXPECT_EQ (0x26, registers.B);
        EXPECT_EQ (0x81, registers.C);
        EXPECT_EQ (0x19, registers.D);
        EXPECT_EQ (0x97, registers.E);
        EXPECT_EQ (0x46, registers.H);
        EXPECT_EQ (0x73, registers.L);
        EXPECT_EQ (8, cycle);
    }

    TEST(LDTest, LD067) // 0x67 LD H, A
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

        const auto cycle = gen::ld_067(arguments, registers, controller);

        EXPECT_EQ (0x12, registers.A);
        EXPECT_EQ (0b1100'0000, registers.F);
        EXPECT_EQ (0x26, registers.B);
        EXPECT_EQ (0x81, registers.C);
        EXPECT_EQ (0x19, registers.D);
        EXPECT_EQ (0x97, registers.E);
        EXPECT_EQ (0x12, registers.H);
        EXPECT_EQ (0x1A, registers.L);
        EXPECT_EQ (4, cycle);
    }

    TEST(LDTest, LD068) // 0x68 LD L, B
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

        const auto cycle = gen::ld_068(arguments, registers, controller);

        EXPECT_EQ (0x12, registers.A);
        EXPECT_EQ (0b1100'0000, registers.F);
        EXPECT_EQ (0x26, registers.B);
        EXPECT_EQ (0x81, registers.C);
        EXPECT_EQ (0x19, registers.D);
        EXPECT_EQ (0x97, registers.E);
        EXPECT_EQ (0xF7, registers.H);
        EXPECT_EQ (0x26, registers.L);
        EXPECT_EQ (4, cycle);
    }

    TEST(LDTest, LD069) // 0x69 LD L, C
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

        const auto cycle = gen::ld_069(arguments, registers, controller);

        EXPECT_EQ (0x12, registers.A);
        EXPECT_EQ (0b1100'0000, registers.F);
        EXPECT_EQ (0x26, registers.B);
        EXPECT_EQ (0x81, registers.C);
        EXPECT_EQ (0x19, registers.D);
        EXPECT_EQ (0x97, registers.E);
        EXPECT_EQ (0xF7, registers.H);
        EXPECT_EQ (0x81, registers.L);
        EXPECT_EQ (4, cycle);
    }

    TEST(LDTest, LD06A) // 0x6A LD L, D
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

        const auto cycle = gen::ld_06a(arguments, registers, controller);

        EXPECT_EQ (0x12, registers.A);
        EXPECT_EQ (0b1100'0000, registers.F);
        EXPECT_EQ (0x26, registers.B);
        EXPECT_EQ (0x81, registers.C);
        EXPECT_EQ (0x19, registers.D);
        EXPECT_EQ (0x97, registers.E);
        EXPECT_EQ (0xF7, registers.H);
        EXPECT_EQ (0x19, registers.L);
        EXPECT_EQ (4, cycle);
    }

    TEST(LDTest, LD06B) // 0x6B LD L, E
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

        const auto cycle = gen::ld_06b(arguments, registers, controller);

        EXPECT_EQ (0x12, registers.A);
        EXPECT_EQ (0b1100'0000, registers.F);
        EXPECT_EQ (0x26, registers.B);
        EXPECT_EQ (0x81, registers.C);
        EXPECT_EQ (0x19, registers.D);
        EXPECT_EQ (0x97, registers.E);
        EXPECT_EQ (0xF7, registers.H);
        EXPECT_EQ (0x97, registers.L);
        EXPECT_EQ (4, cycle);
    }

    TEST(LDTest, LD06C) // 0x6C LD L, H
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

        const auto cycle = gen::ld_06c(arguments, registers, controller);

        EXPECT_EQ (0x12, registers.A);
        EXPECT_EQ (0b1100'0000, registers.F);
        EXPECT_EQ (0x26, registers.B);
        EXPECT_EQ (0x81, registers.C);
        EXPECT_EQ (0x19, registers.D);
        EXPECT_EQ (0x97, registers.E);
        EXPECT_EQ (0xF7, registers.H);
        EXPECT_EQ (0xF7, registers.L);
        EXPECT_EQ (4, cycle);
    }

    TEST(LDTest, LD06D) // 0x6D LD L, L
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

        const auto cycle = gen::ld_06d(arguments, registers, controller);

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

    TEST(LDTest, LD06E) // 0x6E LD L, (HL)
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
        registers.H = 0x58;
        registers.L = 0x52;
        Arguments arguments{};

        EXPECT_CALL (controller, get(0x5852)).Times(1).WillOnce(Return(0x11));

        const auto cycle = gen::ld_06e(arguments, registers, controller);

        EXPECT_EQ (0x12, registers.A);
        EXPECT_EQ (0b1100'0000, registers.F);
        EXPECT_EQ (0x26, registers.B);
        EXPECT_EQ (0x81, registers.C);
        EXPECT_EQ (0x19, registers.D);
        EXPECT_EQ (0x97, registers.E);
        EXPECT_EQ (0x58, registers.H);
        EXPECT_EQ (0x11, registers.L);
        EXPECT_EQ (8, cycle);
    }

    TEST(LDTest, LD06F) // 0x6F LD L, A
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

        const auto cycle = gen::ld_06f(arguments, registers, controller);

        EXPECT_EQ (0x12, registers.A);
        EXPECT_EQ (0b1100'0000, registers.F);
        EXPECT_EQ (0x26, registers.B);
        EXPECT_EQ (0x81, registers.C);
        EXPECT_EQ (0x19, registers.D);
        EXPECT_EQ (0x97, registers.E);
        EXPECT_EQ (0xF7, registers.H);
        EXPECT_EQ (0x12, registers.L);
        EXPECT_EQ (4, cycle);
    }

    TEST(LDTest, LD070) // 0x70 LD (HL), B
    {
        // Test from Chapter 4: page 96
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

        EXPECT_CALL (controller, set(0xF71A, 0x26)).Times(1);

        const auto cycle = gen::ld_070(arguments, registers, controller);

        EXPECT_EQ (0x12, registers.A);
        EXPECT_EQ (0b1100'0000, registers.F);
        EXPECT_EQ (0x26, registers.B);
        EXPECT_EQ (0x81, registers.C);
        EXPECT_EQ (0x19, registers.D);
        EXPECT_EQ (0x97, registers.E);
        EXPECT_EQ (0xF7, registers.H);
        EXPECT_EQ (0x1A, registers.L);
        EXPECT_EQ (8, cycle);
    }

    TEST(LDTest, LD071) // 0x71 LD (HL), C
    {
        // Test from Chapter 4: page 96
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

        EXPECT_CALL (controller, set(0xF71A, 0x81)).Times(1);

        const auto cycle = gen::ld_071(arguments, registers, controller);

        EXPECT_EQ (0x12, registers.A);
        EXPECT_EQ (0b1100'0000, registers.F);
        EXPECT_EQ (0x26, registers.B);
        EXPECT_EQ (0x81, registers.C);
        EXPECT_EQ (0x19, registers.D);
        EXPECT_EQ (0x97, registers.E);
        EXPECT_EQ (0xF7, registers.H);
        EXPECT_EQ (0x1A, registers.L);
        EXPECT_EQ (8, cycle);
    }

    TEST(LDTest, LD072) // 0x72 LD (HL), D
    {
        // Test from Chapter 4: page 96
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

        EXPECT_CALL (controller, set(0xF71A, 0x19)).Times(1);

        const auto cycle = gen::ld_072(arguments, registers, controller);

        EXPECT_EQ (0x12, registers.A);
        EXPECT_EQ (0b1100'0000, registers.F);
        EXPECT_EQ (0x26, registers.B);
        EXPECT_EQ (0x81, registers.C);
        EXPECT_EQ (0x19, registers.D);
        EXPECT_EQ (0x97, registers.E);
        EXPECT_EQ (0xF7, registers.H);
        EXPECT_EQ (0x1A, registers.L);
        EXPECT_EQ (8, cycle);
    }

    TEST(LDTest, LD073) // 0x73 LD (HL), E
    {
        // Test from Chapter 4: page 96
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

        EXPECT_CALL (controller, set(0xF71A, 0x97)).Times(1);

        const auto cycle = gen::ld_073(arguments, registers, controller);

        EXPECT_EQ (0x12, registers.A);
        EXPECT_EQ (0b1100'0000, registers.F);
        EXPECT_EQ (0x26, registers.B);
        EXPECT_EQ (0x81, registers.C);
        EXPECT_EQ (0x19, registers.D);
        EXPECT_EQ (0x97, registers.E);
        EXPECT_EQ (0xF7, registers.H);
        EXPECT_EQ (0x1A, registers.L);
        EXPECT_EQ (8, cycle);
    }

    TEST(LDTest, LD074) // 0x74 LD (HL), H
    {
        // Test from Chapter 4: page 96
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

        EXPECT_CALL (controller, set(0xF71A, 0xF7)).Times(1);

        const auto cycle = gen::ld_074(arguments, registers, controller);

        EXPECT_EQ (0x12, registers.A);
        EXPECT_EQ (0b1100'0000, registers.F);
        EXPECT_EQ (0x26, registers.B);
        EXPECT_EQ (0x81, registers.C);
        EXPECT_EQ (0x19, registers.D);
        EXPECT_EQ (0x97, registers.E);
        EXPECT_EQ (0xF7, registers.H);
        EXPECT_EQ (0x1A, registers.L);
        EXPECT_EQ (8, cycle);
    }

    TEST(LDTest, LD075) // 0x75 LD (HL), L
    {
        // Test from Chapter 4: page 96
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

        EXPECT_CALL (controller, set(0xF71A, 0x1A)).Times(1);

        const auto cycle = gen::ld_075(arguments, registers, controller);

        EXPECT_EQ (0x12, registers.A);
        EXPECT_EQ (0b1100'0000, registers.F);
        EXPECT_EQ (0x26, registers.B);
        EXPECT_EQ (0x81, registers.C);
        EXPECT_EQ (0x19, registers.D);
        EXPECT_EQ (0x97, registers.E);
        EXPECT_EQ (0xF7, registers.H);
        EXPECT_EQ (0x1A, registers.L);
        EXPECT_EQ (8, cycle);
    }

    TEST(LDTest, LD077) // 0x77 LD (HL), A
    {
        // Test from Chapter 4: page 96
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

        EXPECT_CALL (controller, set(0xF71A, 0x12)).Times(1);

        const auto cycle = gen::ld_077(arguments, registers, controller);

        EXPECT_EQ (0x12, registers.A);
        EXPECT_EQ (0b1100'0000, registers.F);
        EXPECT_EQ (0x26, registers.B);
        EXPECT_EQ (0x81, registers.C);
        EXPECT_EQ (0x19, registers.D);
        EXPECT_EQ (0x97, registers.E);
        EXPECT_EQ (0xF7, registers.H);
        EXPECT_EQ (0x1A, registers.L);
        EXPECT_EQ (8, cycle);
    }

    TEST(LDTest, LD078) // 0x78 LD A, B
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

        const auto cycle = gen::ld_078(arguments, registers, controller);

        EXPECT_EQ (0x26, registers.A);
        EXPECT_EQ (0b1100'0000, registers.F);
        EXPECT_EQ (0x26, registers.B);
        EXPECT_EQ (0x81, registers.C);
        EXPECT_EQ (0x19, registers.D);
        EXPECT_EQ (0x97, registers.E);
        EXPECT_EQ (0xF7, registers.H);
        EXPECT_EQ (0x1A, registers.L);
        EXPECT_EQ (4, cycle);
    }

    TEST(LDTest, LD079) // 0x79 LD A, C
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

        const auto cycle = gen::ld_079(arguments, registers, controller);

        EXPECT_EQ (0x81, registers.A);
        EXPECT_EQ (0b1100'0000, registers.F);
        EXPECT_EQ (0x26, registers.B);
        EXPECT_EQ (0x81, registers.C);
        EXPECT_EQ (0x19, registers.D);
        EXPECT_EQ (0x97, registers.E);
        EXPECT_EQ (0xF7, registers.H);
        EXPECT_EQ (0x1A, registers.L);
        EXPECT_EQ (4, cycle);
    }

    TEST(LDTest, LD07A) // 0x7A LD A, D
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

        const auto cycle = gen::ld_07a(arguments, registers, controller);

        EXPECT_EQ (0x19, registers.A);
        EXPECT_EQ (0b1100'0000, registers.F);
        EXPECT_EQ (0x26, registers.B);
        EXPECT_EQ (0x81, registers.C);
        EXPECT_EQ (0x19, registers.D);
        EXPECT_EQ (0x97, registers.E);
        EXPECT_EQ (0xF7, registers.H);
        EXPECT_EQ (0x1A, registers.L);
        EXPECT_EQ (4, cycle);
    }

    TEST(LDTest, LD07B) // 0x7B LD A, E
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

        const auto cycle = gen::ld_07b(arguments, registers, controller);

        EXPECT_EQ (0x97, registers.A);
        EXPECT_EQ (0b1100'0000, registers.F);
        EXPECT_EQ (0x26, registers.B);
        EXPECT_EQ (0x81, registers.C);
        EXPECT_EQ (0x19, registers.D);
        EXPECT_EQ (0x97, registers.E);
        EXPECT_EQ (0xF7, registers.H);
        EXPECT_EQ (0x1A, registers.L);
        EXPECT_EQ (4, cycle);
    }

    TEST(LDTest, LD07C) // 0x7C LD A, H
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

        const auto cycle = gen::ld_07c(arguments, registers, controller);

        EXPECT_EQ (0xF7, registers.A);
        EXPECT_EQ (0b1100'0000, registers.F);
        EXPECT_EQ (0x26, registers.B);
        EXPECT_EQ (0x81, registers.C);
        EXPECT_EQ (0x19, registers.D);
        EXPECT_EQ (0x97, registers.E);
        EXPECT_EQ (0xF7, registers.H);
        EXPECT_EQ (0x1A, registers.L);
        EXPECT_EQ (4, cycle);
    }

    TEST(LDTest, LD07D) // 0x7D LD A, L
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

        const auto cycle = gen::ld_07d(arguments, registers, controller);

        EXPECT_EQ (0x1A, registers.A);
        EXPECT_EQ (0b1100'0000, registers.F);
        EXPECT_EQ (0x26, registers.B);
        EXPECT_EQ (0x81, registers.C);
        EXPECT_EQ (0x19, registers.D);
        EXPECT_EQ (0x97, registers.E);
        EXPECT_EQ (0xF7, registers.H);
        EXPECT_EQ (0x1A, registers.L);
        EXPECT_EQ (4, cycle);
    }

    TEST(LDTest, LD07E) // 0x7E LD A, (HL)
    {
        // Test from Chapter 4: page 96
        MockMemoryController controller;
        Registers registers{};
        registers.A = 0x12;
        registers.F = 0b1100'0000;
        registers.B = 0x26;
        registers.C = 0x81;
        registers.D = 0x19;
        registers.E = 0x97;
        registers.H = 0x51;
        registers.L = 0x73;
        Arguments arguments{};

        EXPECT_CALL (controller, get(0x5173)).Times(1).WillOnce(Return(0xD8));

        const auto cycle = gen::ld_07e(arguments, registers, controller);

        EXPECT_EQ (0xD8, registers.A);
        EXPECT_EQ (0b1100'0000, registers.F);
        EXPECT_EQ (0x26, registers.B);
        EXPECT_EQ (0x81, registers.C);
        EXPECT_EQ (0x19, registers.D);
        EXPECT_EQ (0x97, registers.E);
        EXPECT_EQ (0x51, registers.H);
        EXPECT_EQ (0x73, registers.L);
        EXPECT_EQ (8, cycle);
    }

    TEST(LDTest, LD07F) // 0x7F LD A, A
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

        const auto cycle = gen::ld_07f(arguments, registers, controller);

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

