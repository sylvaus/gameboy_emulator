#pragma once

#include <vector>
#include <unordered_map>
#include <functional>
#include "emulator/memory/registers.h"


namespace emulator::instructions
{
    using std::vector;
    using std::unordered_map;
    using std::function;
    using emulator::memory::Registers;

    const unordered_map<uint16_t, function<void(Registers&, uint8_t value)>> REGISTER_8_BITS_VALUE_REFERENCE_MAP = {
        {0b111, [](Registers& registers, uint8_t value){registers.A = value;}},
        {0b000, [](Registers& registers, uint8_t value){registers.B= value;}},
        {0b001, [](Registers& registers, uint8_t value){registers.C = value;}},
        {0b010, [](Registers& registers, uint8_t value){registers.D = value;}},
        {0b011, [](Registers& registers, uint8_t value){registers.E = value;}},
        {0b100, [](Registers& registers, uint8_t value){registers.H = value;}},
        {0b101, [](Registers& registers, uint8_t value){registers.L = value;}}
    };

    const unordered_map<uint16_t, function<void(Registers&, uint16_t value)>> REGISTER_16_BITS_VALUE_SETTER_MAP = {
        {0b00, [](Registers& registers, uint16_t value){registers.set_BC(value);}},
        {0b01, [](Registers& registers, uint16_t value){registers.set_DE(value);}},
        {0b10, [](Registers& registers, uint16_t value){registers.set_HL(value);}},
        {0b11, [](Registers& registers, uint16_t value){registers.SP = value;}}
    };
}
