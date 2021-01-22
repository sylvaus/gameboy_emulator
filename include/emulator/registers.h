#pragma once

#include <cstdint>

namespace emulator
{
    constexpr uint8_t OFFSET_CARRY_FLAG = 0x4;
    constexpr uint8_t OFFSET_HALF_CARRY_FLAG = 0x5;
    constexpr uint8_t OFFSET_ADD_SUB_FLAG = 0x6;
    constexpr uint8_t OFFSET_ZERO_FLAG = 0x7;

    struct Registers
    {
        uint8_t F;
        uint8_t A;
        uint8_t C;
        uint8_t B;
        uint8_t E;
        uint8_t D;
        uint8_t L;
        uint8_t H;
        uint16_t SP;
        uint16_t PC;

        uint16_t get_AF() const;
        void set_AF(const uint16_t value);
        uint16_t get_BC() const;
        void set_BC(const uint16_t value);
        uint16_t get_DE() const;
        void set_DE(const uint16_t value);
        uint16_t get_HL() const;
        void set_HL(const uint16_t value);

        bool get_carry_flag() const;
        bool get_half_carry_flag() const;
        bool get_add_sub_flag() const;
        bool get_zero_flag() const;
    };
}
