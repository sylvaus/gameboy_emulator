#include "emulator/memory_controller.h"

namespace emulator
{
    MemoryControllerNoExternal::MemoryControllerNoExternal(
        unique_ptr<FirstRomBank> first_rom,
        vector<RomBank> &rom_banks,
        uint8_t nb_ram_banks,
        CartridgeType cartridge_type
    ): first_rom_(std::move(first_rom)), rom_banks_(std::move(rom_banks)),
       external_ram_banks_(nb_ram_banks), cartridge_type_(cartridge_type)
    {}

    void MemoryControllerNoExternal::set(uint32_t address, uint8_t value)
    {
        // TODO
    }

    uint8_t MemoryControllerNoExternal::get(uint32_t address) const
    {
        // TODO
        return 0;
    }
}