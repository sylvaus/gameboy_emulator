#include "emulator/exceptions.h"
#include "emulator/memory/memory.h"

namespace emulator::memory
{
    void Memory::set(uint16_t address, uint8_t value)
    {
        if (address <= END_ROM_1_N)
        {
            mbc_->set_rom(address, value);
        }
        else if (address <= END_VRAM)
        {
            video_->set_vram(address, value);
        }
        else if (address <= END_EXT_RAM)
        {
            mbc_->set_ext_ram(address, value);
        }
        else if (address <= END_SECOND_WORK_RAM)
        {
            ram_->set_wram(address, value);
        }
        else if (address <= END_ECHO_RAM)
        {
            ram_->set_echo_ram(address, value);
        }
        else if (address <= END_OAM)
        {
            video_->set_oam(address, value);
        }
        else if (address <= END_NOT_USABLE)
        {
            throw MemoryAccessError("Trying to access the not usable area of memory");
        }
        else if (address <= END_IO_REGISTER)
        {
            set_io_register(address, value);
        }
        else if (address <= END_HRAM)
        {
            ram_->set_hram(address, value);
        }
        else
        {
            // TODO handling Interrupt enable register section
        }
    }

    uint8_t Memory::get(uint16_t address) const
    {
        if (address <= END_ROM_1_N)
        {
            return mbc_->get_rom(address);
        }
        else if (address <= END_VRAM)
        {
            return video_->get_vram(address);
        }
        else if (address <= END_EXT_RAM)
        {
            return mbc_->get_ext_ram(address);
        }
        else if (address <= END_SECOND_WORK_RAM)
        {
            return ram_->get_wram(address);
        }
        else if (address <= END_ECHO_RAM)
        {
            return ram_->get_echo_ram(address);
        }
        else if (address <= END_OAM)
        {
            return video_->get_oam(address);
        }
        else if (address <= END_NOT_USABLE)
        {
            throw MemoryAccessError("Trying to access the not usable area of memory");
        }
        else if (address <= END_IO_REGISTER)
        {
            return get_io_register(address);
        }
        else if (address <= END_HRAM)
        {
            return ram_->get_hram(address);
        }
        else
        {
            return 0; // TODO handling Interrupt enable register section
        }
    }

    void Memory::set_io_register(uint16_t address, uint8_t value)
    {
        // TODO: complete using https://gbdev.io/pandocs/Memory_Map.html#io-ranges
    }

    uint8_t Memory::get_io_register(uint16_t address) const
    {
        // TODO: complete using https://gbdev.io/pandocs/Memory_Map.html#io-ranges
        return 0;
    }
}