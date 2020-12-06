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

        virtual void set_rom(uint32_t address, uint8_t value) = 0;
        virtual void set_ram(uint32_t address, uint8_t value) = 0;

        [[nodiscard]] virtual uint8_t get_rom(uint32_t  address) const = 0;
        [[nodiscard]] virtual uint8_t get_ram(uint32_t  address) const = 0;

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

        void set_rom(uint32_t address, uint8_t value) override;
        void set_ram(uint32_t address, uint8_t value) override;

        [[nodiscard]]  uint8_t get_rom(uint32_t  address) const;
        [[nodiscard]]  uint8_t get_ram(uint32_t  address) const;
    private:
        static Logger LOGGER;
    };
}


