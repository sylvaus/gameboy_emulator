#include <fstream>
#include <iostream>
#include <vector>
#include <memory>
#include <functional>

#include "emulator/exceptions.h"
#include "emulator/logging.h"
#include "emulator/memory/memory_map.h"
#include "emulator/memory/mbc.h"
#include "emulator/cartridge.h"


namespace emulator
{
    using std::vector;
    using std::unique_ptr;
    using memory::MemoryBankController;
    using memory::MemoryBankControllerPtr;
    using memory::RomBank;
    using memory::ROM_BANK_SIZE;
    using utils::UnMutableDefaultMap;
    using MemoryBankControllerProvider = MemoryBankControllerPtr(*)(vector<memory::RomBank>&, uint8_t);

    auto LOGGER = Logging::get_logger("Cartridge");

    std::vector<RomBank> load_roms(std::ifstream& stream, uint16_t nb_roms)
    {
        std::vector<RomBank> rom_banks(nb_roms);
        for (int i = 0; i < nb_roms; ++i)
        {
            stream.read(reinterpret_cast<char *>(rom_banks[i].data()), memory::ROM_BANK_SIZE);
        }
        return rom_banks;
    }

    MemoryBankControllerPtr provides_null(vector<RomBank>&, uint8_t)
    {
        return nullptr;
    }

    const UnMutableDefaultMap<memory::MemoryBankType, MemoryBankControllerProvider> ROM_RAM_CONTROLLER_MAP
    {&provides_null, {
        {memory::MemoryBankType::ROM_ONLY, &memory::NoMemoryBankController::create}
    }};


    MemoryBankControllerPtr load_cartridge(const std::string &filename)
    {
        auto stream = std::ifstream(filename, std::ios::binary);
        if (!stream.is_open())
            throw LoadCartridgeError("Could not open the file with the given path" + filename);

        auto rom_banks =  vector<RomBank>(1);
        stream.read(reinterpret_cast<char *>(rom_banks[0].data()), ROM_BANK_SIZE);

        const auto cartridge_type = memory::get_cartridge_type(rom_banks[0]);
        const auto nb_rom_banks = memory::get_nb_rom_banks(rom_banks[0]);
        const auto nb_ram_banks = memory::get_nb_ram_banks(rom_banks[0]);
        const auto checksum = memory::get_header_checksum(rom_banks[0]);
        const auto computed_checksum = memory::compute_header_checksum(rom_banks[0]);

        LOGGER.debug("Name of the game: " + memory::get_title(rom_banks[0]));
        LOGGER.debug("Rom size: " + std::to_string(nb_rom_banks));
        LOGGER.debug("Ram size: " + std::to_string(nb_ram_banks));
        LOGGER.debug("Checksum matches: " + std::to_string(checksum == computed_checksum));

        for (size_t i = 1; i < nb_rom_banks; ++i)
        {
            rom_banks.emplace_back();
            stream.read(reinterpret_cast<char *>(rom_banks.back().data()), ROM_BANK_SIZE);
        }

        return ROM_RAM_CONTROLLER_MAP.get(cartridge_type.type)(rom_banks, nb_ram_banks);
    }
}