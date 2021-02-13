#include "emulator/memory/registers.h"

namespace emulator::memory
{
    uint8_t make_flag(bool zero, bool add_sub, bool half_carry, bool carry)
    {
        return (zero << OFFSET_ZERO_FLAG) +
            (add_sub << OFFSET_ADD_SUB_FLAG) +
            (half_carry << OFFSET_HALF_CARRY_FLAG) +
            (carry << OFFSET_CARRY_FLAG);
    }

    uint16_t Registers::get_AF() const
    {
        return (A << 8) + F;
    }

    void Registers::set_AF(const uint16_t value)
    {
        F = value & 0xFF;
        A = value >> 8;
    }

    uint16_t Registers::get_BC() const
    {
        return (B << 8) + C;
    }

    void Registers::set_BC(const uint16_t value)
    {
        C = value & 0xFF;
        B = value >> 8;
    }

    uint16_t Registers::get_DE() const
    {
        return (D << 8) + E;
    }

    void Registers::set_DE(const uint16_t value)
    {
        E = value & 0xFF;
        D = value >> 8;
    }

    uint16_t Registers::get_HL() const
    {
        return (H << 8) + L;
    }

    void Registers::set_HL(const uint16_t value)
    {
        L = value & 0xFF;
        H = value >> 8;
    }

    bool Registers::get_carry_flag() const
    {
        return (F >> OFFSET_CARRY_FLAG) & 0x1;
    }

    bool Registers::get_half_carry_flag() const
    {
        return (F >> OFFSET_HALF_CARRY_FLAG) & 0x1;
    }

    bool Registers::get_add_sub_flag() const
    {
        return (F >> OFFSET_ADD_SUB_FLAG) & 0x1;
    }

    bool Registers::get_zero_flag() const
    {
        return (F >> OFFSET_ZERO_FLAG) & 0x1;
    }

    bool Registers::get_non_carry_flag() const
    {
        return !get_carry_flag();
    }

    bool Registers::get_non_half_carry_flag() const
    {
        return !get_half_carry_flag();
    }

    bool Registers::get_non_add_sub_flag() const
    {
        return !get_add_sub_flag();
    }

    bool Registers::get_non_zero_flag() const
    {
        return !get_zero_flag();
    }
}

