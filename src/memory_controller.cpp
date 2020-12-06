#include "emulator/memory_controller.h"

namespace emulator
{
    MemoryController::MemoryController(
        unique_ptr<FirstRomBank> first_rom,
        vector<RomBank> &rom_banks,
        uint8_t nb_ram_banks,
        CartridgeType cartridge_type
    ): first_rom_(std::move(first_rom)), rom_banks_(std::move(rom_banks)),
       external_ram_banks_(nb_ram_banks), cartridge_type_(cartridge_type)
    {}

    Logger MemoryControllerNoExternal::LOGGER = Logging::get_logger("MemoryControllerNoExternal");

    std::unique_ptr<MemoryController>
    MemoryControllerNoExternal::create(
        unique_ptr<FirstRomBank> first_rom,
        vector<RomBank> &rom_banks,
        uint8_t nb_ram_banks,
        CartridgeType cartridge_type)
    {
        return std::make_unique<MemoryControllerNoExternal>(std::move(first_rom), rom_banks, nb_ram_banks, cartridge_type);
    }

    MemoryControllerNoExternal::MemoryControllerNoExternal(
        unique_ptr<FirstRomBank> first_rom,
        vector<RomBank> &rom_banks,
        uint8_t nb_ram_banks,
        CartridgeType cartridge_type
    ): MemoryController(std::move(first_rom), rom_banks, nb_ram_banks, cartridge_type)
    {}

    void MemoryControllerNoExternal::set_rom(uint32_t address, uint8_t value)
    {
        if (address < START_ROM_1_N)
            first_rom_->data[address] = value;
        rom_banks_[0][address - START_ROM_1_N] = value;
    }

    void MemoryControllerNoExternal::set_ram(uint32_t address, uint8_t value)
    {
        if (external_ram_banks_.empty())
        {
            LOGGER.warn("Trying to set value in non existing ram");
            return;
        }

        external_ram_banks_[0][address - START_EXT_RAM] = value;
    }

    uint8_t MemoryControllerNoExternal::get_rom(uint32_t address) const
    {
        if (address < START_ROM_1_N)
            return first_rom_->data[address];
        return rom_banks_[0][address - START_ROM_1_N];
    }

    uint8_t MemoryControllerNoExternal::get_ram(uint32_t address) const
    {
        if (external_ram_banks_.empty())
        {
            LOGGER.warn("Trying to access non existing ram");
            return 0xFF;
        }

        return external_ram_banks_[0][address - START_EXT_RAM];
    }
}