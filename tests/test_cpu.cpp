#include "gtest/gtest.h"
#include "emulator/cpu.h"

using emulator::CPURegisters;

namespace
{
    TEST(CPUTest, CPU)
    {
        CPURegisters registers{{0x1234, 0x1598, 0x3578, 0x4568, 0x3219, 0x4571}};
        EXPECT_EQ(0x1234, registers.AF);
        EXPECT_EQ(0x1598, registers.BC);
        EXPECT_EQ(0x3578, registers.DE);
        EXPECT_EQ(0x4568, registers.HL);
        EXPECT_EQ(0x3219, registers.SP);
        EXPECT_EQ(0x4571, registers.PC);
    }

    TEST(CPUTest, CPUAF)
    {
        CPURegisters registers{};
        registers.AF = 0x1234;
        EXPECT_EQ(0x12, registers.A);
        EXPECT_EQ(0x34, registers.F);
        registers.A = 0x59;
        registers.F = 0x78;
        EXPECT_EQ(0x5978, registers.AF);
        registers.F = 0b1010'0000;
        EXPECT_EQ(true, registers.zf);
        EXPECT_EQ(false, registers.n);
        EXPECT_EQ(true, registers.h);
        EXPECT_EQ(false, registers.cy);
        registers.zf = false;
        registers.n = true;
        registers.h = false;
        registers.cy = true;
        EXPECT_EQ(0b0101, (registers.F >> 4) & 0xF);
    }

    TEST(CPUTest, CPUBC)
    {
        CPURegisters registers{};
        registers.BC = 0x1234;
        EXPECT_EQ(0x12, registers.B);
        EXPECT_EQ(0x34, registers.C);
        registers.B = 0x59;
        registers.C = 0x78;
        EXPECT_EQ(0x5978, registers.BC);
    }

    TEST(CPUTest, CPUDE)
    {
        CPURegisters registers{};
        registers.DE = 0x1234;
        EXPECT_EQ(0x12, registers.D);
        EXPECT_EQ(0x34, registers.E);
        registers.D = 0x59;
        registers.E = 0x78;
        EXPECT_EQ(0x5978, registers.DE);
    }

    TEST(CPUTest, CPUHL)
    {
        CPURegisters registers{};
        registers.HL = 0x1234;
        EXPECT_EQ(0x12, registers.H);
        EXPECT_EQ(0x34, registers.L);
        registers.H = 0x59;
        registers.L = 0x78;
        EXPECT_EQ(0x5978, registers.HL);
    }
}