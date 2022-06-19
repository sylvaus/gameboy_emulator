#pragma once

namespace emulator::cpu
{
    struct Interrupts
    {
        bool vblank{false};
        bool lcd_stat{false};
        bool timer{false};
        bool serial{false};
        bool joypad{false};

        Interrupts& operator|= (const Interrupts& other)
        {
            vblank |= other.vblank;
            lcd_stat |= other.lcd_stat;
            timer |= other.timer;
            serial |= other.serial;
            joypad |= other.joypad;

            return *this;
        }

        Interrupts operator| (const Interrupts& rhs) const
        {
            Interrupts result = *this;
            return result |= rhs;
        }

        bool operator==(const Interrupts& rhs) const
        {
            return vblank == rhs.vblank &&
                   lcd_stat == rhs.lcd_stat &&
                   timer == rhs.timer &&
                   serial == rhs.serial &&
                   joypad == rhs.joypad;
        }

        bool operator!=(const Interrupts& rhs) const
        {
            return !(rhs == *this);
        }
    };
} // emulator::cpu
