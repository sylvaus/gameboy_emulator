#pragma once

#include <cstdint>

#include "emulator/memory/memory_map.h"

namespace emulator::video
{
    class VideoController
    {
    public:
        virtual void set_vram(uint16_t address, uint8_t value);
        [[nodiscard]] virtual uint8_t get_vram(uint16_t address) const;
        virtual void set_oam(uint16_t address, uint8_t value);
        [[nodiscard]] virtual uint8_t get_oam(uint16_t address) const;

        virtual ~VideoController() = default;

    private:
        emulator::memory::VideoRam ram_{};
    };
}
