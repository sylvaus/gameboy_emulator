#pragma once

#include <cstdint>

namespace emulator
{
    union CPURegisters
    {
        uint16_t data[8];
        struct
        {
            union
            {
                uint16_t AF;
                struct
                {
                    uint8_t A;
                    uint8_t unused:4;
                    uint8_t cy:1;
                    uint8_t h:1;
                    uint8_t n:1;
                    uint8_t zf:1;
                };
            };
            union
            {
                uint16_t BC;
                struct
                {
                    uint8_t B;
                    uint8_t C;
                };
            };
            union
            {
                uint16_t DE;
                struct
                {
                    uint8_t D;
                    uint8_t E;
                };
            };
            union
            {
                uint16_t HL;
                struct
                {
                    uint8_t H;
                    uint8_t L;
                };
            };
            uint32_t SP;
            uint32_t PC;
        };

    };
}
