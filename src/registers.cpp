#include "emulator/registers.h"

namespace emulator
{
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
}

