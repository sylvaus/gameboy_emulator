#include "emulator/memory_map.h"

#include "emulator/exceptions.h"

namespace emulator
{
    CartridgeType get_cartridge_type(const RomBank& rom)
    {
        auto cartridge_type = CARTRIDGE_TYPE_MAP.get(rom[ADDRESS_CARTRIDGE_TYPE]);
        if (cartridge_type.type == MemoryBankType::UNKNOWN)
            throw LoadCartridgeError(
                "Could not load the cartridge, the cartridge type is unknown: " +
                std::to_string(rom[ADDRESS_CARTRIDGE_TYPE])
            );
        return cartridge_type;
    }

    ROMSize get_rom_size(const RomBank& rom)
    {
        return ROM_SIZE_MAP.get(rom[ADDRESS_ROM_SIZE]);
    }

    uint16_t get_nb_rom_banks(const RomBank& rom)
    {
        auto nb_rom_banks = NB_ROM_BANK_MAP.get(get_rom_size(rom));
        if (nb_rom_banks == INVALID_ROM_BANK_NB)
            throw LoadCartridgeError(
                "Could not load the cartridge, the rom size is unknown: " +
                std::to_string(rom[ADDRESS_ROM_SIZE])
            );
        return nb_rom_banks;
    }

    RAMSize get_ram_size(const RomBank& rom)
    {
        return RAM_SIZE_MAP.get(rom[ADDRESS_RAM_SIZE]);
    }

    uint8_t get_nb_ram_banks(const RomBank& rom)
    {
        auto nb_ram_banks = NB_RAM_BANK_MAP.get(get_ram_size(rom));
        if (nb_ram_banks == INVALID_RAM_BANK_NB)
            throw LoadCartridgeError(
                "Could not load the cartridge, the ram size is unknown: " +
                std::to_string(rom[ADDRESS_RAM_SIZE])
            );
        return nb_ram_banks;
    }

    std::string get_new_licensee_code(RomBank rom)
    {
        return std::string(reinterpret_cast<char *>(&rom[ADDRESS_NEW_LICENSEE_CODE]), SIZE_NEW_LICENSEE_CODE);
    }

    uint8_t get_sgb_flag(const RomBank& rom)
    {
        return rom[ADDRESS_SGB_FLAG];
    }

    std::string get_short_title(RomBank rom)
    {
        return std::string(reinterpret_cast<char *>(&rom[ADDRESS_SHORT_TITLE]), SIZE_SHORT_TITLE);
    }

    std::string get_manufacturer_code(RomBank rom)
    {
        return std::string(reinterpret_cast<char *>(&rom[ADDRESS_MANUFACTURER_CODE]), SIZE_MANUFACTURER_CODE);
    }

    uint8_t get_cbg_flag(const RomBank& rom)
    {
        return rom[ADDRESS_CBG_FLAG];
    }

    std::string get_title(RomBank rom)
    {
        return std::string(reinterpret_cast<char *>(&rom[ADDRESS_TITLE]), SIZE_TITLE);
    }

    uint8_t get_destination_code(const RomBank& rom)
    {
        return rom[ADDRESS_DESTINATION_CODE];
    }

    uint8_t get_old_license_code(const RomBank& rom)
    {
        return rom[ADDRESS_OLD_LICENSE_CODE];
    }

    uint8_t get_mask_rom_version_number(const RomBank& rom)
    {
        return rom[ADDRESS_MASK_ROM_VERSION_NUMBER];
    }

    uint8_t get_header_checksum(const RomBank& rom)
    {
        return rom[ADDRESS_HEADER_CHECKSUM];
    }

    uint16_t get_global_checksum(const RomBank& rom)
    {
        return rom[ADDRESS_GLOBAL_CHECKSUM] << 8 + rom[ADDRESS_GLOBAL_CHECKSUM + 1];
    }
}