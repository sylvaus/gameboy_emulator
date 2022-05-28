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
        virtual void set_lcd_control(uint8_t value);
        [[nodiscard]] virtual uint8_t get_lcd_control() const;
        virtual void set_lcd_status(uint8_t value);
        [[nodiscard]] virtual uint8_t get_lcd_status() const;
        virtual void set_lcd_scroll_y(uint8_t value);
        [[nodiscard]] virtual uint8_t get_lcd_scroll_y() const;
        virtual void set_lcd_scroll_x(uint8_t value);
        [[nodiscard]] virtual uint8_t get_lcd_scroll_x() const;
        virtual void set_lcd_coordinate_y(uint8_t value);
        [[nodiscard]] virtual uint8_t get_lcd_coordinate_y() const;
        virtual void set_lcd_compare_y(uint8_t value);
        [[nodiscard]] virtual uint8_t get_lcd_compare_y() const;
        virtual void set_bg_palette_data(uint8_t value);
        [[nodiscard]] virtual uint8_t get_bg_palette_data() const;
        virtual void set_obj_palette_data_0(uint8_t value);
        [[nodiscard]] virtual uint8_t get_obj_palette_data_0() const;
        virtual void set_obj_palette_data_1(uint8_t value);
        [[nodiscard]] virtual uint8_t get_obj_palette_data_1() const;
        virtual void set_lcd_window_position_y(uint8_t value);
        [[nodiscard]] virtual uint8_t get_lcd_window_position_y() const;
        virtual void set_lcd_window_position_x(uint8_t value);
        [[nodiscard]] virtual uint8_t get_lcd_window_position_x() const;
        virtual void select_vram_bank(uint8_t value);
        [[nodiscard]] virtual uint8_t get_vram_bank() const;
        virtual void set_cgb_bj_obj_palettes(uint16_t address, uint8_t value);
        [[nodiscard]] virtual uint8_t get_cgb_bj_obj_palettes(uint16_t address) const;

        virtual ~VideoController() = default;

    private:
        emulator::memory::VideoRam ram_{};
    };
}
