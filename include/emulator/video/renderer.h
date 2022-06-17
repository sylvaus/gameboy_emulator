#pragma once

#include "emulator/memory/memory_map.h"
#include "memory.h"

namespace emulator::video
{
    class Renderer
    {
    public:
        virtual void render_line(
            const emulator::memory::VideoRam& ram,
            const emulator::memory::SpriteAttributeTable& oam,
            const LcdAllControlAndStatus& lcd_bits
        ) = 0;

        virtual ~Renderer() = default;
    };
} // emulator::video
