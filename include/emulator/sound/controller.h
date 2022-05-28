#pragma once

#include <cstdint>

namespace emulator::sound
{
    class SoundController
    {
    public:
        virtual void set_sound(uint16_t address, uint8_t value);
        [[nodiscard]] virtual uint8_t get_sound(uint16_t address) const;
        virtual void set_wave_pattern(uint16_t address, uint8_t value);
        [[nodiscard]] virtual uint8_t get_wave_pattern(uint16_t address) const;

        virtual ~SoundController() = default;
    };
}
