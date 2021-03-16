#pragma once


#include "gtest/gtest.h"

#include "emulator/memory/registers.h"
#include "emulator/generated/instructions.h"

#include "../../mocks/romram_controller.h"

using emulator::generated::Arguments;
using emulator::memory::Registers;
using emulator::mocks::MockMemoryController;

namespace emulator::instructions::fixtures
{
    class InstructionTestFixture: public testing::Test
    {
    protected:
        void SetUp() override;
        void TearDown() override;
        void set_expected_pc_increase(int16_t length);
        void set_expected_pc(uint16_t length);
        void set_expected_sp_increase(int16_t length);
        void set_expected_sp(uint16_t length);

        MockMemoryController controller;
        Registers registers;
        Registers expected_registers;
        Arguments arguments;
    };
}


