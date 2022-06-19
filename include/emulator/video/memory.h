#pragma once

#include <cstdint>

namespace emulator::video
{
    union LcdControl
    {
        // Structure defined here: https://gbdev.io/pandocs/LCDC.html
        uint8_t value;
        struct
        {
            uint8_t enable_lcd: 1;
            uint8_t window_tile_map_area: 1;
            uint8_t window_enable: 1;
            uint8_t bg_tile_map_area: 1;
            uint8_t obj_size: 1;
            uint8_t enable_obj: 1;
            uint8_t enable_bg_and_window: 1;
        };
    };

    union LcdStatus
    {
        // Structure defined here: https://gbdev.io/pandocs/STAT.html
        uint8_t value;
        struct
        {
            uint8_t mode: 2;
            uint8_t lyc_ly: 1;
            uint8_t mode0_interrupt: 1;
            uint8_t mode1_interrupt: 1;
            uint8_t mode2_interrupt: 1;
            uint8_t lyc_ly_interrupt: 1;
        };
    };

    struct LcdAllControlAndStatus
    {
        LcdControl control{};
        LcdStatus status{};
        uint8_t scroll_y{};
        uint8_t scroll_x{};
        uint8_t coordinate_y{};
        uint8_t compare_y{};
        uint8_t window_position_y{};
        uint8_t window_position_x{};
        uint8_t bg_palette_data{};
        uint8_t obj_palette_data_0{};
        uint8_t obj_palette_data_1{};
    };

    constexpr uint8_t MODE_0_HBLANK = 0;
    constexpr uint8_t MODE_1_VBLANK = 1;
    constexpr uint8_t MODE_2_SEARCH_OAM = 2;
    constexpr uint8_t MODE_3_TRANSFER = 3;
}