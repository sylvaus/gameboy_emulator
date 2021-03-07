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
using emulator::instructions::REGISTER_FLAG_VALUES;
using emulator::instructions::REGISTER_FLAG_VALUE_SETTER_MAP;
using emulator::instructions::REGISTER_FLAG_VALUE_NAME_MAP;
using emulator::instructions::NameMapPrinter;
using emulator::instructions::fixtures::InstructionTestFixture;

namespace gen = emulator::generated;

namespace
{
    class RetTestFixture : public InstructionTestFixture, public ::testing::WithParamInterface<uint16_t>{};

    TEST_P(RetTestFixture, RETConditionalTrue)
    {
        // Test from Chapter 4: page 119
        uint16_t register_index = GetParam();
        registers.F = emulator::memory::make_flag(true, true, false, false);
        registers.SP = 0x1234;
        registers.PC = 0x2458;
        REGISTER_FLAG_VALUE_SETTER_MAP.at(register_index)(registers, true);
        expected_registers.F = emulator::memory::make_flag(true, true, false, false);
        expected_registers.SP = registers.SP + 2;
        expected_registers.PC = 0xCAFE;
        REGISTER_FLAG_VALUE_SETTER_MAP.at(register_index)(expected_registers, true);
        EXPECT_CALL (controller, get(0x1234)).Times(1).WillOnce(::testing::Return(0xFE));
        EXPECT_CALL (controller, get(0x1235)).Times(1).WillOnce(::testing::Return(0xCA));

        uint16_t instruction_index = (register_index << 3) + 0b1100'0000;
        const auto cycle = gen::INSTRUCTION_FUNCTIONS[instruction_index](arguments, registers, controller);

        EXPECT_EQ(20, cycle);
    }

    TEST_P(RetTestFixture, RETConditionalFalse)
    {
        // Test from Chapter 4: page 119
        uint16_t register_index = GetParam();
        registers.F = emulator::memory::make_flag(true, true, false, false);
        registers.SP = 0x1234;
        registers.PC = 0x2458;
        REGISTER_FLAG_VALUE_SETTER_MAP.at(register_index)(registers, false);
        expected_registers.F = emulator::memory::make_flag(true, true, false, false);
        expected_registers.SP = registers.SP;
        expected_registers.PC = 0x2458;
        REGISTER_FLAG_VALUE_SETTER_MAP.at(register_index)(expected_registers, false);

        set_expected_pc_increase(1);

        uint16_t instruction_index = (register_index << 3) + 0b1100'0000;
        const auto cycle = gen::INSTRUCTION_FUNCTIONS[instruction_index](arguments, registers, controller);

        EXPECT_EQ(8, cycle);
    }

    INSTANTIATE_TEST_SUITE_P(
        RetConditionalTest, RetTestFixture, REGISTER_FLAG_VALUES, NameMapPrinter(REGISTER_FLAG_VALUE_NAME_MAP)
    );

    TEST_F(RetTestFixture, RETUnconditional)
    {
        // Test from Chapter 4: page 119
        registers.F = emulator::memory::make_flag(true, true, false, false);
        registers.SP = 0x1234;
        registers.PC = 0x2458;
        expected_registers.F = emulator::memory::make_flag(true, true, false, false);
        expected_registers.SP = registers.SP + 2;
        expected_registers.PC = 0xCAFE;
        EXPECT_CALL (controller, get(0x1234)).Times(1).WillOnce(::testing::Return(0xFE));
        EXPECT_CALL (controller, get(0x1235)).Times(1).WillOnce(::testing::Return(0xCA));

        const auto cycle = gen::INSTRUCTION_FUNCTIONS[0b1100'1001](arguments, registers, controller);

        EXPECT_EQ(16, cycle);
    }

    TEST_F(RetTestFixture, RETI)
    {
        // Test from Chapter 4: page 119
        registers.F = emulator::memory::make_flag(true, true, false, false);
        registers.SP = 0x78AD;
        registers.PC = 0x2458;
        registers.ime_flag = false;
        expected_registers.F = emulator::memory::make_flag(true, true, false, false);
        expected_registers.SP = registers.SP + 2;
        expected_registers.PC = 0x1256;
        expected_registers.ime_flag = true;
        EXPECT_CALL (controller, get(0x78AD)).Times(1).WillOnce(::testing::Return(0x56));
        EXPECT_CALL (controller, get(0x78AE)).Times(1).WillOnce(::testing::Return(0x12));

        const auto cycle = gen::INSTRUCTION_FUNCTIONS[0b1101'1001](arguments, registers, controller);

        EXPECT_EQ(16, cycle);
    }
}