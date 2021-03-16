#pragma once

#include <cstdint>
#include <memory>
#include <vector>

#include "memory_map.h"
#include "emulator/logging.h"

namespace emulator::memory
{
    using std::vector;
    using std::unique_ptr;

    class RomRamController
    {
    public:
        static std::unique_ptr<memory::RomRamController> create(vector<memory::RomBank>&, uint8_t nb_ram_banks);

        virtual void set(uint16_t address, uint8_t value) = 0;
        void set16bits(uint16_t address, uint16_t value)
        {
            set(address, value & 0xFF);
            set(address + 1, (value >> 8) & 0xFF);
        }
        [[nodiscard]] virtual uint8_t get(uint16_t  address) const = 0;

        virtual ~RomRamController() = default;
    };

    class RomRamControllerNoExternal : public RomRamController
    {
    public:
        RomRamControllerNoExternal(vector<RomBank> &rom_banks, uint8_t nb_ram_banks);

        void set(uint16_t address, uint8_t value) override;

        [[nodiscard]]  uint8_t get(uint16_t  address) const;

    protected:
        vector<RomBank> rom_banks_;
        vector<RamBank> ram_banks_;
    };
}


