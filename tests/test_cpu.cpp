#include "gtest/gtest.h"
#include "emulator/memory/registers.h"

namespace
{
    using emulator::memory::Registers;
    TEST(CPUTest, CPUAF)
    {
        Registers registers{};
        registers.set_AF(0x1234);
        EXPECT_EQ(0x12, registers.A);
        EXPECT_EQ(0x34, registers.F);
        registers.A = 0x59;
        registers.F = 0x78;
        EXPECT_EQ(0x5978, registers.get_AF());
        registers.F = 0b1010'0000;
        EXPECT_EQ(true, registers.get_zero_flag());
        EXPECT_EQ(false, registers.get_add_sub_flag());
        EXPECT_EQ(true, registers.get_half_carry_flag());
        EXPECT_EQ(false, registers.get_carry_flag());
    }

    TEST(CPUTest, CPUBC)
    {
        Registers registers{};
        registers.set_BC(0x1234);
        EXPECT_EQ(0x12, registers.B);
        EXPECT_EQ(0x34, registers.C);
        registers.B = 0x59;
        registers.C = 0x78;
        EXPECT_EQ(0x5978, registers.get_BC());
    }

    TEST(CPUTest, CPUDE)
    {
        Registers registers{};
        registers.set_DE(0x1234);
        EXPECT_EQ(0x12, registers.D);
        EXPECT_EQ(0x34, registers.E);
        registers.D = 0x59;
        registers.E = 0x78;
        EXPECT_EQ(0x5978, registers.get_DE());
    }

    TEST(CPUTest, CPUHL)
    {
        Registers registers{};
        registers.set_HL(0x1234);
        EXPECT_EQ(0x12, registers.H);
        EXPECT_EQ(0x34, registers.L);
        registers.H = 0x59;
        registers.L = 0x78;
        EXPECT_EQ(0x5978, registers.get_HL());
    }
}