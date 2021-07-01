#pragma once

#include <cstdint>
#include <memory>
#include <vector>

#include "mbc.h"
#include "emulator/logging.h"

namespace emulator::memory
{
    using std::vector;
    using std::unique_ptr;

    class Memory
    {
    public:
        virtual void set(uint16_t address, uint8_t value) = 0;

        void set16bits(uint16_t address, uint16_t value)
        {
            set(address, value & 0xFF);
            set(address + 1, (value >> 8) & 0xFF);
        }

        [[nodiscard]] virtual uint8_t get(uint16_t address) const = 0;

        virtual ~Memory() = default;

    private:
        unique_ptr<MemoryBankController> rom_ram_;
    };
}
