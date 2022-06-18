#pragma once

#include <gmock/gmock.h>
#include "emulator/memory/memory.h"

namespace emulator::mocks
{
    class MockMemoryController : public emulator::memory::Memory
    {
    public:
        MOCK_METHOD(void, set, (uint16_t address, uint8_t value), (override));
        MOCK_METHOD(uint8_t, get, (uint16_t  address), (const, override));
    };
}