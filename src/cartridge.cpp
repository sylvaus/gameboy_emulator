#include <fstream>
#include <iostream>
#include <vector>
#include <memory>
#include <functional>

#include "emulator/exceptions.h"
#include "emulator/logging.h"
#include "emulator/memory_map.h"
#include "emulator/memory_controller.h"
#include "emulator/cartridge.h"


namespace emulator
{
    using std::unique_ptr;
    using MemoryBankControllerPtr = std::unique_ptr<MemoryController>;
    using MemoryBankControllerProvider = std::function<MemoryBankControllerPtr(
        vector<RomBank>&, uint8_t nb_ram_banks, CartridgeType
    )>;

    auto LOGGER = Logging::get_logger("Cartridge");

    std::vector<RomBank> load_roms(std::ifstream& stream, uint16_t nb_roms)
    {
        std::vector<RomBank> rom_banks(nb_roms);
        for (int i = 0; i < nb_roms; ++i)
        {
            stream.read(reinterpret_cast<char *>(rom_banks[i]), ROM_BANK_SIZE);
        }
        return rom_banks;
    }

    MemoryBankControllerPtr provides_null(vector<RomBank>&, uint8_t, CartridgeType)
    {
        return nullptr;
    }



    const UnMutableDefaultMap<MemoryBankType, MemoryBankControllerProvider> MEMORY_CONTROLLER_MAP
    {&provides_null, {
        {MemoryBankType::ROM_ONLY, &MemoryControllerNoExternal::create}
    }};


    MemoryBankControllerPtr load_cartridge(const std::string &filename)
    {
        auto stream = std::ifstream(filename, std::ios::binary);
        if (!stream.is_open())
            throw LoadCartridgeError("Could not open the file with the given path" + filename);

        auto rom_banks =  vector<RomBank>(1);
        stream.read(reinterpret_cast<char *>(rom_banks[0]), ROM_BANK_SIZE);

        LOGGER.debug("Name of the game: " + get_title(rom_banks[0]));
        LOGGER.debug("Rom size: " + ROM_SIZE_NAME_MAP.get(get_rom_size(rom_banks[0])));
        LOGGER.debug("Ram size: " + RAM_SIZE_NAME_MAP.get(get_ram_size(rom_banks[0])));

        const auto cartridge_type = get_cartridge_type(rom_banks[0]);
        const auto nb_rom_banks = get_nb_rom_banks(rom_banks[0]);
        const auto nb_ram_banks = get_nb_ram_banks(rom_banks[0]);

        auto roms = load_roms(stream, nb_rom_banks);

        return MEMORY_CONTROLLER_MAP.get(cartridge_type.type)(roms, nb_ram_banks, cartridge_type);
    }
}