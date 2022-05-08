#include "emulator/memory/mbc.h"
#include "emulator/exceptions.h"

namespace emulator::memory
{
    using std::vector;
    using std::unique_ptr;
    MemoryBankController::MemoryBankController(uint8_t nb_external_ram_banks):
        external_ram_banks_{nb_external_ram_banks}
    {}

    MemoryBankControllerPtr NoMemoryBankController::create(
        vector<memory::RomBank>& rom_banks, uint8_t nb_external_ram_banks
    )
    {
        return std::make_unique<memory::NoMemoryBankController>(rom_banks, nb_external_ram_banks);
    }

    NoMemoryBankController::NoMemoryBankController(vector<RomBank> &rom_banks, uint8_t nb_external_ram_banks):
        MemoryBankController(nb_external_ram_banks), rom_banks_( std::move(rom_banks))
    {
        // https://gbdev.io/pandocs/nombc.html
        if (1 < nb_external_ram_banks)
            throw MemoryConfigurationError("No MBC cannot have more than 1 external ram bank");
    }

    void NoMemoryBankController::set_rom(uint16_t address, uint8_t value)
    {
        if (address <= END_ROM_0)
            rom_banks_[0][address] = value;
        else if (address <= END_ROM_1_N)
            rom_banks_[1][address - END_ROM_0] = value;

        throw MemoryAccessError("No MBC is not supposed to access: " + std::to_string(address));
    }

    uint8_t NoMemoryBankController::get_rom(uint16_t address) const
    {
        if (address <= END_ROM_0)
            return rom_banks_[0][address];
        else if (address <= END_ROM_1_N)
            return rom_banks_[1][address - END_ROM_0];

        throw MemoryAccessError("No MBC is not supposed to access: " + std::to_string(address));
    }

    void NoMemoryBankController::set_ext_ram(uint16_t address, uint8_t value)
    {
        if (!rom_banks_.empty() && START_EXT_RAM <= address && address <= END_EXT_RAM)
            external_ram_banks_[0][address - END_EXT_RAM] = value;

        throw MemoryAccessError("No MBC is not supposed to access: " + std::to_string(address));
    }

    uint8_t NoMemoryBankController::get_ext_ram(uint16_t address) const
    {
        if (!rom_banks_.empty() && START_EXT_RAM <= address && address <= END_EXT_RAM)
            return external_ram_banks_[0][address - END_EXT_RAM];

        throw MemoryAccessError("No MBC is not supposed to access: " + std::to_string(address));
    }
}