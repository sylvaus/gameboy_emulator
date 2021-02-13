#pragma once

#include "gmock/gmock.h"
#include "emulator/memory/memory_controller.h"

namespace emulator::mocks
{
class MockMemoryController : public emulator::memory::MemoryController
    {
    public:
        MOCK_METHOD(void, set, (uint32_t address, uint8_t value), (override));
        MOCK_METHOD(uint8_t, get, (uint32_t  address), (const, override));
    };
}