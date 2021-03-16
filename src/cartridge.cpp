#include <fstream>
#include <iostream>
#include <vector>
#include <memory>
#include <functional>

#include "emulator/exceptions.h"
#include "emulator/logging.h"
#include "emulator/memory/memory_map.h"
#include "emulator/memory/romram_controller.h"
#include "emulator/cartridge.h"


namespace emulator
{
    using std::vector;
    using std::unique_ptr;
    using memory::RomRamController;
    using memory::RomBank;
    using memory::ROM_BANK_SIZE;
    using utils::UnMutableDefaultMap;
    using MemoryBankControllerPtr = std::unique_ptr<memory::RomRamController>;
    using MemoryBankControllerProvider = MemoryBankControllerPtr(*)(vector<memory::RomBank>&, uint8_t nb_ram_banks);

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

    const UnMutableDefaultMap<memory::MemoryBankType, MemoryBankControllerProvider> MEMORY_CONTROLLER_MAP
    {&provides_null, {
        {memory::MemoryBankType::ROM_ONLY, &memory::RomRamControllerNoExternal::create}
    }};


    MemoryBankControllerPtr load_cartridge(const std::string &filename)
    {
        auto stream = std::ifstream(filename, std::ios::binary);
        if (!stream.is_open())
            throw LoadCartridgeError("Could not open the file with the given path" + filename);

        auto rom_banks =  vector<RomBank>(1);
        stream.read(reinterpret_cast<char *>(rom_banks[0].data()), ROM_BANK_SIZE);

        LOGGER.debug("Name of the game: " + memory::get_title(rom_banks[0]));
        LOGGER.debug("Rom size: " + memory::ROM_SIZE_NAME_MAP.get(memory::get_rom_size(rom_banks[0])));
        LOGGER.debug("Ram size: " + memory::RAM_SIZE_NAME_MAP.get(memory::get_ram_size(rom_banks[0])));

        const auto cartridge_type = memory::get_cartridge_type(rom_banks[0]);
        const auto nb_rom_banks = memory::get_nb_rom_banks(rom_banks[0]);
        const auto nb_ram_banks = memory::get_nb_ram_banks(rom_banks[0]);

        auto roms = load_roms(stream, nb_rom_banks);

        return MEMORY_CONTROLLER_MAP.get(cartridge_type.type)(roms, nb_ram_banks);
    }
}