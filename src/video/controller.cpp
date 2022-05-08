#include "emulator/video/controller.h"

namespace emulator::video
{
    void VideoController::set_vram(uint16_t address, uint8_t value)
    {
        ram_[address - emulator::memory::START_VRAM] = value;
    }

    uint8_t VideoController::get_vram(uint16_t address) const
    {
        return ram_[address - emulator::memory::START_VRAM];
    }

    void VideoController::set_oam(uint16_t address, uint8_t value)
    {
        ram_[address - emulator::memory::START_OAM] = value;
    }

    uint8_t VideoController::get_oam(uint16_t address) const
    {
        return ram_[address - emulator::memory::START_OAM];
    }
}
