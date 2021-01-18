#pragma once

#include <cstdint>

namespace emulator
{
    union CPURegisters
    {
        uint16_t data[6];
        struct
        {
            union
            {
                uint16_t AF;
                struct
                {
                    union
                    {
                        uint8_t F;
                        struct
                        {
                            uint8_t unused: 4;
                            bool cy: 1;
                            bool h: 1;
                            bool n: 1;
                            bool zf: 1;
                        };
                    };
                    uint8_t A;
                };
            };
            union
            {
                uint16_t BC;
                struct
                {
                    uint8_t C;
                    uint8_t B;
                };
            };
            union
            {
                uint16_t DE;
                struct
                {
                    uint8_t E;
                    uint8_t D;
                };
            };
            union
            {
                uint16_t HL;
                struct
                {
                    uint8_t L;
                    uint8_t H;
                };
            };
            uint16_t SP;
            uint16_t PC;
        };

    };
}
