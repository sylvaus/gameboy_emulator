#pragma once

namespace emulator::cpu
{
    enum class Interrupt
    {
        VBLANK,
        LCD_STAT,
        TIMER,
        SERIAL,
        JOYPAD
    };
} // emulator::cpu
