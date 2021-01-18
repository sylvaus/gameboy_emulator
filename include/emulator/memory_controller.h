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

    class MemoryController
    {
    public:
        MemoryController(
            unique_ptr<FirstRomBank> first_rom,
            vector<RomBank> &rom_banks,
            uint8_t nb_ram_banks,
            CartridgeType cartridge_type
        );

        virtual void set(uint32_t address, uint8_t value) = 0;
        void set16bits(uint32_t address, uint16_t value)
        {
            set(address, value & 0xFF);
            set(address + 1, (value >> 8) & 0xFF);
        }
        [[nodiscard]] virtual uint8_t get(uint32_t  address) const = 0;

        virtual ~MemoryController() = default;

    protected:
        vector<RomBank> rom_banks_;
        unique_ptr<FirstRomBank> first_rom_;
        vector<ExternalRam> external_ram_banks_;
        CartridgeType cartridge_type_;
    };

    class MemoryControllerNoExternal : public MemoryController
    {
    public:
        static std::unique_ptr<MemoryController> create(
            unique_ptr<FirstRomBank> first_rom,
            vector<RomBank> &rom_banks,
            uint8_t nb_ram_banks,
            CartridgeType cartridge_type
        );

        MemoryControllerNoExternal(
                unique_ptr<FirstRomBank> first_rom,
                vector<RomBank> &rom_banks,
                uint8_t nb_ram_banks,
                CartridgeType cartridge_type
        );

        void set(uint32_t address, uint8_t value) override;

        [[nodiscard]]  uint8_t get(uint32_t  address) const;
    private:
        static Logger LOGGER;
    };
}


