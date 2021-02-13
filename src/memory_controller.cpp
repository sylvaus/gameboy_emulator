#include "emulator/romram_controller.h"

namespace emulator
{
    std::unique_ptr<RomRamController>
    RomRamControllerNoExternal::create(vector<RomBank> &rom_banks, uint8_t nb_ram_banks, CartridgeType cartridge_type)
    {
        return std::make_unique<RomRamControllerNoExternal>(rom_banks, nb_ram_banks, cartridge_type);
    }

    RomRamControllerNoExternal::RomRamControllerNoExternal(
        vector<RomBank> &rom_banks, uint8_t nb_ram_banks, CartridgeType cartridge_type
    ): rom_banks_(std::move(rom_banks)), ram_banks_(nb_ram_banks), cartridge_type_(cartridge_type)
    {}

    void RomRamControllerNoExternal::set(uint32_t address, uint8_t value)
    {
        // TODO
    }

    uint8_t RomRamControllerNoExternal::get(uint32_t address) const
    {
        // TODO
        return 0;
    }


}