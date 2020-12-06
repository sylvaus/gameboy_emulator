#include "emulator/memory_map.h"

namespace emulator
{


    CartridgeType FirstRomBank::get_cartridge_type() const
    {
        return CARTRIDGE_TYPE_MAP.get(cartridge_type);
    }

    ROMSize FirstRomBank::get_rom_size() const
    {
        return ROM_SIZE_MAP.get(rom_size);
    }

    uint16_t FirstRomBank::get_nb_rom_banks() const
    {
        return NB_ROM_BANK_MAP.get(get_rom_size());
    }

    RAMSize FirstRomBank::get_ram_size() const
    {
        return RAM_SIZE_MAP.get(ram_size);
    }

    uint8_t FirstRomBank::get_nb_ram_banks() const
    {
        return NB_RAM_BANK_MAP.get(get_ram_size());
    }

    std::string FirstRomBank::get_title() const
    {
        return std::string(short_title, SHORT_TITLE_SIZE);
    }
}