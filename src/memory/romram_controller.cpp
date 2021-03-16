#include "emulator/memory/romram_controller.h"

namespace emulator::memory
{
    std::unique_ptr<memory::RomRamController> RomRamController::create(vector<memory::RomBank>& rom_banks, uint8_t nb_ram_banks)
    {
        return std::make_unique<memory::RomRamControllerNoExternal>(rom_banks, nb_ram_banks);
    }

    RomRamControllerNoExternal::RomRamControllerNoExternal(vector<RomBank> &rom_banks, uint8_t nb_ram_banks):
    rom_banks_(std::move(rom_banks)), ram_banks_(nb_ram_banks)
    {}

    void RomRamControllerNoExternal::set(uint16_t address, uint8_t value)
    {
        if (address < END_ROM_1_N)
        {
            const auto [rom_index, rel_address] = std::div(address, ROM_BANK_SIZE);
            rom_banks_[rom_index][rel_address] = value;
        }
        else if (address < END_EXT_RAM)
        {
            ram_banks_[0][address - END_EXT_RAM] = value;
        }
    }

    uint8_t RomRamControllerNoExternal::get(uint16_t address) const
    {
        if (address < END_ROM_1_N)
        {
            const auto [rom_index, rel_address] = std::div(address, ROM_BANK_SIZE);
            return rom_banks_[rom_index][rel_address];
        }
        else if (address < END_EXT_RAM)
        {
            return ram_banks_[0][address - END_EXT_RAM];
        }
        return 0;
    }
}