#include "emulator/memory/ram.h"

namespace emulator::memory
{
    void RamController::set_wram(uint16_t address, uint8_t value)
    {
        wram_[address - START_FIRST_WORK_RAM] = value;
    }

    uint8_t RamController::get_wram(uint16_t address) const
    {
        return wram_[address - START_FIRST_WORK_RAM];
    }

    void RamController::set_echo_ram(uint16_t address, uint8_t value)
    {
        set_wram(address - 0x2000, value);
    }

    uint8_t RamController::get_echo_ram(uint16_t address) const
    {
        return get_wram(address - 0x2000);
    }

    void RamController::set_hram(uint16_t address, uint8_t value)
    {
        hram_[address - START_HRAM] = value;
    }

    uint8_t RamController::get_hram(uint16_t address) const
    {
        return hram_[address - START_HRAM];
    }

    void RamController::select_wram_bank(uint8_t value)
    {
        // Nothing to do for non cgb
    }

    uint8_t RamController::get_wram_selected() const
    {
        // Nothing to do for non cgb
        return 0;
    }
}
