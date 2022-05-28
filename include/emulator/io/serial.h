#pragma once

#include <cstdint>

namespace emulator::io
{
    class SerialTransfer
    {
    public:
        virtual void set(uint16_t address, uint8_t value);
        [[nodiscard]] virtual uint8_t get(uint16_t address) const;

        virtual ~SerialTransfer() = default;
    };
}

