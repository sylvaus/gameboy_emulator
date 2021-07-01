#include "emulator/memory/mbc.h"

namespace emulator::memory
{
    using std::vector;
    using std::unique_ptr;

    MemoryBankControllerPtr NoMemoryBankController::create(vector<memory::RomBank>& rom_banks)
    {
        return std::make_unique<memory::NoMemoryBankController>(rom_banks);
    }

    NoMemoryBankController::NoMemoryBankController(vector <RomBank> &rom_banks): rom_banks_( std::move(rom_banks))
    {}

    void NoMemoryBankController::set(uint16_t address, uint8_t value)
    {
        if (address < END_ROM_0)
            rom_banks_[0][address] = value;
        else if (address < END_ROM_1_N)
            rom_banks_[1][address] = value;
    }

    uint8_t NoMemoryBankController::get(uint16_t address) const
    {
        if (address < END_ROM_0)
            return rom_banks_[0][address];
        else if (address < END_ROM_1_N)
            return rom_banks_[1][address];
        return 0;
    }
}