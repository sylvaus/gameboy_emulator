#pragma once

#include <cstdint>

namespace emulator::time
{
    class Timer
    {
    public:
        virtual void set(uint16_t address, uint8_t value);
        [[nodiscard]] virtual uint8_t get(uint16_t address) const;

        virtual ~Timer() = default;

    private:
        uint8_t divide_register_{0};
        uint8_t timer_counter_{0};
        uint8_t timer_modulo_{0};
        uint8_t timer_control_{0};

    };
}
