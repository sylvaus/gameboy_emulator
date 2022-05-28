#include "emulator/time/timer.h"
#include "emulator/exceptions.h"

namespace emulator::time
{
    constexpr uint16_t DIVIDE_REGISTER_ADDRESS = 0xFF04;
    constexpr uint16_t TIME_COUNTER_ADDRESS = 0xFF05;
    constexpr uint16_t TIMER_MODULO_ADDRESS = 0xFF06;
    constexpr uint16_t TIMER_CONTROL_ADDRESS = 0xFF07;

    void Timer::set(uint16_t address, uint8_t value)
    {
        switch (address)
        {
            case DIVIDE_REGISTER_ADDRESS:
                divide_register_ = 0;
                break;
            case TIME_COUNTER_ADDRESS:
                timer_counter_ = value;
                break;
            case TIMER_MODULO_ADDRESS:
                timer_modulo_ = value;
                break;
            case TIMER_CONTROL_ADDRESS:
                timer_control_ = value;
                break;
            default:
                throw MemoryAccessError("Invalid timer address: " + std::to_string(address));
        }
    }

    uint8_t Timer::get(uint16_t address) const
    {
        switch (address)
        {
            case DIVIDE_REGISTER_ADDRESS:
                return divide_register_;
            case TIME_COUNTER_ADDRESS:
                return timer_counter_;
            case TIMER_MODULO_ADDRESS:
                return timer_modulo_;
            case TIMER_CONTROL_ADDRESS:
                return timer_control_;
            default:
                throw MemoryAccessError("Invalid timer address: " + std::to_string(address));
        }
    }
}