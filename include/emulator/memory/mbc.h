#pragma once

#include <cstdint>
#include <memory>
#include <vector>
#include "emulator/logging.h"
#include "emulator/memory/memory_map.h"
#include "emulator/memory/ram.h"

namespace emulator::memory
{
    class MemoryBankController
    {
    public:
        virtual void set(uint16_t address, uint8_t value) = 0;
        void set16bits(uint16_t address, uint16_t value)
        {
            set(address, value & 0xFF);
            set(address + 1, (value >> 8) & 0xFF);
        }
        [[nodiscard]] virtual uint8_t get(uint16_t  address) const = 0;
        virtual ~MemoryBankController() = default;
    };

    using MemoryBankControllerPtr = std::unique_ptr<MemoryBankController>;

    class NoMemoryBankController : public MemoryBankController
    {
    public:
        static MemoryBankControllerPtr create(std::vector<memory::RomBank>&);

        NoMemoryBankController(vector<RomBank> &rom_banks);

        void set(uint16_t address, uint8_t value) override;
        [[nodiscard]]  uint8_t get(uint16_t  address) const override;

    protected:
        std::vector<RomBank> rom_banks_;
    };
}


