#pragma once

#include <cstdint>

namespace emulator::memory
{
    constexpr uint8_t OFFSET_CARRY_FLAG = 0x4;
    constexpr uint8_t OFFSET_HALF_CARRY_FLAG = 0x5;
    constexpr uint8_t OFFSET_ADD_SUB_FLAG = 0x6;
    constexpr uint8_t OFFSET_ZERO_FLAG = 0x7;

    [[nodiscard]] uint8_t make_flag(bool zero, bool add_sub, bool half_carry, bool carry);

    /**
     * This structure represents the Gameboy register and flags described here:
     *      https://gbdev.io/pandocs/CPU_Registers_and_Flags.html
     */
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

        bool halted;
        bool stopped;
        bool ime_flag;  // Interrupt Master Enable Flag

        [[nodiscard]] uint16_t get_AF() const;
        void set_AF(uint16_t value);
        [[nodiscard]] uint16_t get_BC() const;
        void set_BC(uint16_t value);
        [[nodiscard]] uint16_t get_DE() const;
        void set_DE(uint16_t value);
        [[nodiscard]] uint16_t get_HL() const;
        void set_HL(uint16_t value);

        [[nodiscard]] bool get_carry_flag() const;
        void set_carry_flag(bool value);
        [[nodiscard]] bool get_half_carry_flag() const;
        void set_half_carry_flag(bool value);
        [[nodiscard]] bool get_add_sub_flag() const;
        void set_add_sub_flag(bool value);
        [[nodiscard]] bool get_zero_flag() const;
        void set_zero_flag(bool value);
        [[nodiscard]] bool get_non_carry_flag() const;
        [[nodiscard]] bool get_non_half_carry_flag() const;
        [[nodiscard]] bool get_non_add_sub_flag() const;
        [[nodiscard]] bool get_non_zero_flag() const;
    };
}
