#pragma once


#include <cstdint>
#include "memory_map.h"

namespace emulator::memory
{
    // TODO: Add another controller for CGB mode when we need to be able to switch RAM
    class RamController
    {
    public:
        virtual void set_wram(uint16_t address, uint8_t value);
        [[nodiscard]] virtual uint8_t get_wram(uint16_t address) const;
        virtual void set_echo_ram(uint16_t address, uint8_t value);
        [[nodiscard]] virtual uint8_t get_echo_ram(uint16_t address) const;
        virtual void set_hram(uint16_t address, uint8_t value);
        [[nodiscard]] virtual uint8_t get_hram(uint16_t address) const;

        virtual void select_wram_bank(uint8_t value);
        [[nodiscard]] virtual uint8_t get_wram_selected() const;

        virtual ~RamController() = default;

    private:
        std::array<uint8_t, 2 * WORK_RAM_SIZE> wram_{};
        HighRam hram_{};

    };
}
