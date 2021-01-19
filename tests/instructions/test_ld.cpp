#include "gtest/gtest.h"
#include "emulator/registers.h"

#include "../mocks/memory_controller.h"

using emulator::Registers;
using emulator::mocks::MockMemoryController;

namespace
{
    TEST(LDTest, LD)
    {
        MockMemoryController controller;
        // TODO implement some ld tests
    }
}

