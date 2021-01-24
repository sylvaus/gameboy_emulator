#pragma once

#include <cstdint>
#include <memory>
#include <vector>

#include "emulator/memory_map.h"
#include "logging.h"

namespace emulator
{
    using std::vector;
    using std::unique_ptr;

    class RomRamController
    {
    public:
        virtual void set(uint32_t address, uint8_t value) = 0;
        void set16bits(uint32_t address, uint16_t value)
        {
            set(address, value & 0xFF);
            set(address + 1, (value >> 8) & 0xFF);
        }
        [[nodiscard]] virtual uint8_t get(uint32_t  address) const = 0;

        virtual ~RomRamController() = default;
    };

    class RomRamControllerNoExternal : public RomRamController
    {
    public:
        static std::unique_ptr<RomRamController>
        create(vector<RomBank> &rom_banks, uint8_t nb_ram_banks, CartridgeType cartridge_type);

        RomRamControllerNoExternal(vector<RomBank> &rom_banks, uint8_t nb_ram_banks, CartridgeType cartridge_type);

        void set(uint32_t address, uint8_t value) override;

        [[nodiscard]]  uint8_t get(uint32_t  address) const;

    protected:
        vector<RomBank> rom_banks_;
        vector<RamBank> ram_banks_;
        CartridgeType cartridge_type_;

    private:
        static Logger LOGGER;
    };
}


