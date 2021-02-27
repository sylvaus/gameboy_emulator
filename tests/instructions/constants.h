#pragma once

#include <string>
#include <vector>
#include <unordered_map>
#include <functional>

#include "gtest/gtest.h"

#include "emulator/memory/registers.h"


namespace emulator::instructions
{
    using std::vector;
    using std::unordered_map;
    using std::function;
    using emulator::memory::Registers;

    const auto REGISTER_8_BITS_VALUES = ::testing::Values(
        0b111, 0b000, 0b001, 0b010, 0b011, 0b100, 0b101
    );
    // Table Chapter 4: section 2.1 in GameBoyProgManVer1.1.pdf  symbol: r
    const unordered_map<uint16_t, function<void(Registers&, uint8_t value)>> REGISTER_8_BITS_VALUE_SETTER_MAP = {
        {0b111, [](Registers& registers, uint8_t value){registers.A = value;}},
        {0b000, [](Registers& registers, uint8_t value){registers.B= value;}},
        {0b001, [](Registers& registers, uint8_t value){registers.C = value;}},
        {0b010, [](Registers& registers, uint8_t value){registers.D = value;}},
        {0b011, [](Registers& registers, uint8_t value){registers.E = value;}},
        {0b100, [](Registers& registers, uint8_t value){registers.H = value;}},
        {0b101, [](Registers& registers, uint8_t value){registers.L = value;}}
    };
    const unordered_map<uint16_t, std::string> REGISTER_8_BITS_VALUE_NAME_MAP = {
        {0b111, "Register_A"},
        {0b000, "Register_B"},
        {0b001, "Register_C"},
        {0b010, "Register_D"},
        {0b011, "Register_E"},
        {0b100, "Register_H"},
        {0b101, "Register_L"}
    };

    const auto REGISTER_16_BITS_VALUES = ::testing::Values(0b00, 0b01, 0b10, 0b11);
    // Table Chapter 4: section 2.4 in GameBoyProgManVer1.1.pdf symbol: ss
    const unordered_map<uint16_t, function<void(Registers&, uint16_t value)>> REGISTER_16_BITS_VALUE_SETTER_MAP = {
        {0b00, [](Registers& registers, uint16_t value){registers.set_BC(value);}},
        {0b01, [](Registers& registers, uint16_t value){registers.set_DE(value);}},
        {0b10, [](Registers& registers, uint16_t value){registers.set_HL(value);}},
        {0b11, [](Registers& registers, uint16_t value){registers.SP = value;}}
    };
    const unordered_map<uint16_t, std::string> REGISTER_16_BITS_VALUE_NAME_MAP = {
        {0b00, "Register_BC"},
        {0b01, "Register_DE"},
        {0b10, "Register_HL"},
        {0b11, "Register_SP"}
    };

    const auto REGISTER_FLAG_VALUES = ::testing::Values(0b00, 0b01, 0b10, 0b11);
    // Table Chapter 4: section 2.7 in GameBoyProgManVer1.1.pdf  symbol: cc
    const unordered_map<uint16_t, function<void(Registers&, bool value)>> REGISTER_FLAG_VALUE_SETTER_MAP = {
        {0b00, [](Registers& registers, bool value){registers.set_zero_flag(!value);}},
        {0b01, [](Registers& registers, bool value){registers.set_zero_flag(value);}},
        {0b10, [](Registers& registers, bool value){registers.set_carry_flag(!value);}},
        {0b11, [](Registers& registers, bool value){registers.set_carry_flag(value);}}
    };
    const unordered_map<uint16_t, std::string> REGISTER_FLAG_VALUE_NAME_MAP = {
        {0b00, "NonZero"},
        {0b01, "Zero"},
        {0b10, "NoCarry"},
        {0b11, "Carry"}
    };


    template<class ParamType>
    class NameMapPrinter
    {
    public:
        explicit NameMapPrinter(const std::unordered_map<ParamType, std::string>& name_map): name_map_(name_map) {}
        std::string operator()(testing::TestParamInfo<ParamType> info) const
        {
            return name_map_.at(info.param);
        }

    private:
        const std::unordered_map<ParamType, std::string>& name_map_;
    };

}
