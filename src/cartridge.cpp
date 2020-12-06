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
        unique_ptr<FirstRomBank>,
        vector<RomBank>&,
        uint8_t,
        CartridgeType
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

    MemoryBankControllerPtr providesNull(unique_ptr<FirstRomBank>, vector<RomBank>&, uint8_t, CartridgeType)
    {
        return nullptr;
    }

    const UnMutableDefaultMap<MemoryBankType, MemoryBankControllerProvider> MEMORY_CONTROLLER_MAP
    { &providesNull, {
        {MemoryBankType::ROM_ONLY, &MemoryControllerNoExternal::create}
    }};

    CartridgeType get_cartridge_type(FirstRomBank* rom_bank)
    {
        auto cartridge_type = rom_bank->get_cartridge_type();
        if (cartridge_type.type == MemoryBankType::UNKNOWN)
            throw LoadCartridgeError(
                "Could not load the cartridge, the cartridge type is unknown: " +
                std::to_string(rom_bank->cartridge_type)
            );
        return cartridge_type;
    }

    uint16_t get_nb_rom_banks(FirstRomBank* rom_bank)
    {
        auto nb_rom_banks = rom_bank->get_nb_rom_banks();
        if (nb_rom_banks == INVALID_ROM_BANK_NB)
            throw LoadCartridgeError(
                "Could not load the cartridge, the rom size is unknown: " +
                std::to_string(rom_bank->rom_size)
            );
        return nb_rom_banks;
    }

    uint8_t get_nb_ram_banks(const std::_MakeUniq<FirstRomBank>::__single_object &rom_bank)
    {
        auto nb_ram_banks = rom_bank->get_nb_ram_banks();
        if (nb_ram_banks == INVALID_RAM_BANK_NB)
            throw LoadCartridgeError(
                "Could not load the cartridge, the ram size is unknown: " +
                std::to_string(rom_bank->ram_size)
            );
        return nb_ram_banks;
    }

    MemoryBankControllerPtr load_cartridge(const std::string &filename)
    {
        auto stream = std::ifstream(filename, std::ios::binary);
        if (!stream.is_open())
            throw LoadCartridgeError("Could not open the file with the given path" + filename);

        auto rom_bank =  std::make_unique<FirstRomBank>();
        stream.read(reinterpret_cast<char *>(rom_bank->data), ROM_BANK_SIZE);

        LOGGER.debug("Name of the game: " + rom_bank->get_title());
        LOGGER.debug("Rom size: " + ROM_SIZE_NAME_MAP.get(rom_bank->get_rom_size()));
        LOGGER.debug("Ram size: " + RAM_SIZE_NAME_MAP.get(rom_bank->get_ram_size()));

        const auto cartridge_type = get_cartridge_type(rom_bank.get());
        const auto nb_rom_banks = get_nb_rom_banks(rom_bank.get());
        const auto nb_ram_banks = get_nb_ram_banks(rom_bank);

        auto roms = load_roms(stream, nb_rom_banks);

        return MEMORY_CONTROLLER_MAP.get(cartridge_type.type)(
            std::move(rom_bank), roms, nb_ram_banks, cartridge_type
        );
    }
}