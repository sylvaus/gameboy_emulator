#pragma once

#include <cstdint>

namespace emulator::io
{
    class Joypad
    {
    public:
        [[nodiscard]] virtual uint8_t get_flags() const;
        void select_buttons(uint8_t value);

        virtual ~Joypad() = default;
    private:
        uint8_t flag;
    };
}

