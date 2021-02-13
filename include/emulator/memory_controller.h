#pragma once

#include <cstdint>
#include <memory>
#include <vector>

#include "emulator/romram_controller.h"
#include "logging.h"

namespace emulator
{
    using std::vector;
    using std::unique_ptr;

    class MemoryController
    {
    public:
        virtual void set(uint32_t address, uint8_t value) = 0;

        void set16bits(uint32_t address, uint16_t value)
        {
            set(address, value & 0xFF);
            set(address + 1, (value >> 8) & 0xFF);
        }

        [[nodiscard]] virtual uint8_t get(uint32_t address) const = 0;

        virtual ~MemoryController() = default;

    private:
        unique_ptr<RomRamController> rom_ram_;
    };
}
