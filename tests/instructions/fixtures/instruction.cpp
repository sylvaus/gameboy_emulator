#include "instruction.h"

namespace emulator::instructions::fixtures
{

    void InstructionTestFixture::SetUp()
    {
        registers = Registers{};
        registers.PC = 0x15FA;
        registers.SP = 0xB2F5;
        registers.A = 0x12;
        registers.F = 0b1100'0000;
        registers.B = 0x26;
        registers.C = 0x81;
        registers.D = 0x19;
        registers.E = 0x97;
        registers.H = 0xF7;
        registers.L = 0x1A;
        expected_registers = registers;

        arguments = Arguments{};
    }

    void InstructionTestFixture::TearDown()
    {
        EXPECT_EQ(expected_registers.PC, registers.PC);
        EXPECT_EQ(expected_registers.SP, registers.SP);
        EXPECT_EQ(expected_registers.A, registers.A);
        EXPECT_EQ(expected_registers.F, registers.F);
        EXPECT_EQ(expected_registers.B, registers.B);
        EXPECT_EQ(expected_registers.C, registers.C);
        EXPECT_EQ(expected_registers.D, registers.D);
        EXPECT_EQ(expected_registers.E, registers.E);
        EXPECT_EQ(expected_registers.H, registers.H);
        EXPECT_EQ(expected_registers.L, registers.L);
    }

    void InstructionTestFixture::set_expected_pc_increase(int16_t length)
    {
        expected_registers.PC = registers.PC + length;
    }

    void InstructionTestFixture::set_expected_pc(uint16_t value)
    {
        expected_registers.PC = value;
    }

    void InstructionTestFixture::set_expected_sp_increase(int16_t length)
    {
        expected_registers.SP = registers.SP + length;
    }

    void InstructionTestFixture::set_expected_sp(uint16_t value)
    {
        expected_registers.SP = value;
    }
}