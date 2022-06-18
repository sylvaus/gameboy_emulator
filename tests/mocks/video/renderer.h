#pragma once

#include <gmock/gmock.h>
#include "emulator/video/renderer.h"

namespace emulator::video
{
    class MockRenderer: public Renderer
    {
    public:
        MOCK_METHOD(void, render_line, (
                const emulator::memory::VideoRam& ram,
                const emulator::memory::SpriteAttributeTable& oam,
                const LcdAllControlAndStatus& lcd_bits
        ), (override));
    };
} // emulator::video
