#include "emulator/io/joypad.h"

namespace emulator::io
{
    uint8_t Joypad::get_flags() const
    {
        return 0;
    }

    void Joypad::select_buttons(uint8_t value)
    {
        // TODO retrieve input value and set flag
    }
}
