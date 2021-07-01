#include "emulator/memory/ram.h"

namespace emulator::memory
{
    RamControllerBase::RamControllerBase(uint8_t nb_ram_banks): external_ram_banks_(nb_ram_banks)
    {}

    void RamControllerBase::set(uint16_t address, uint8_t value)
    {
    }

    uint8_t RamControllerBase::get(uint16_t address) const
    {
        return 0;
    }

    std::unique_ptr<memory::RamController> RamControllerNoCGBOnly::create(uint8_t nb_ram_banks)
    {
        return std::unique_ptr<memory::RamController>();
    }

    RamControllerNoCGBOnly::RamControllerNoCGBOnly(uint8_t nb_ram_banks) : RamControllerBase(nb_ram_banks)
    {

    }

    void RamControllerNoCGBOnly::set(uint16_t address, uint8_t value)
    {
        RamControllerBase::set(address, value);
    }

    uint8_t RamControllerNoCGBOnly::get(uint16_t address) const
    {
        return RamControllerBase::get(address);
    }

    void RamControllerNoCGBOnly::set_work_ram(uint16_t address, uint8_t value)
    {

    }

    uint8_t RamControllerNoCGBOnly::get_work_ram(uint16_t address) const
    {
        return 0;
    }

    std::unique_ptr<memory::RamController> RamControllerCGBOnly::create(uint8_t nb_ram_banks)
    {
        return std::unique_ptr<memory::RamController>();
    }

    RamControllerCGBOnly::RamControllerCGBOnly(uint8_t nb_ram_banks) : RamControllerBase(nb_ram_banks)
    {

    }

    void RamControllerCGBOnly::set(uint16_t address, uint8_t value)
    {
        RamControllerBase::set(address, value);
    }

    uint8_t RamControllerCGBOnly::get(uint16_t address) const
    {
        return RamControllerBase::get(address);
    }

    void RamControllerCGBOnly::set_work_ram(uint16_t address, uint8_t value)
    {

    }

    uint8_t RamControllerCGBOnly::get_work_ram(uint16_t address) const
    {
        return 0;
    }
}