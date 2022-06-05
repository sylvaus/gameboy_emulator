#pragma once

#include <cstdint>
#include <unordered_set>

#include "emulator/types.h"
#include "emulator/memory/memory_map.h"
#include "emulator/cpu/interrupt.h"

namespace emulator::video
{
    union LcdControl
    {
        uint8_t value;
        struct
        {
            uint8_t enable_lcd:1;
            uint8_t window_tile_map_area:1;
            uint8_t window_enable:1;
            uint8_t bg_tile_map_area:1;
            uint8_t obj_size:1;
            uint8_t enable_obj:1;
            uint8_t enable_bg_and_window:1;
        };
    };

    union LcdStatus
    {
        uint8_t value;
        struct
        {
            uint8_t mode:2;
            uint8_t lyc_ly:1;
            uint8_t mode0_interrupt:1;
            uint8_t mode1_interrupt:1;
            uint8_t mode2_interrupt:1;
            uint8_t lyc_ly_interrupt:1;
        };
    };

    constexpr uint8_t MODE_0_HBLANK = 0;
    constexpr uint8_t MODE_1_VBLANK = 1;
    constexpr uint8_t MODE_2_SEARCH_OAM = 2;
    constexpr uint8_t MODE_3_TRANSFER = 3;

    class VideoController
    {
    public:
        std::unordered_set<emulator::cpu::Interrupt> tick(const tick_t nb_cycles);

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
        emulator::memory::SpriteAttributeTable oam_{};

        tick_t clock_;
        tick_t next_action_clock_;

        LcdControl lcd_control_;
        LcdStatus lcd_status_;
        uint8_t lcd_scroll_y_;
        uint8_t lcd_scroll_x_;
        uint8_t lcd_coordinate_y_;
        uint8_t lcd_compare_y_;
        uint8_t lcd_window_position_y_;
        uint8_t lcd_window_position_x_;
        uint8_t bg_palette_data_;
        uint8_t obj_palette_data_0_;
        uint8_t obj_palette_data_1_;
        bool start_frame;

        void update_mode();
        void perform_mode_action();
        bool does_mode_triggers_interrupt();

        std::unordered_set<cpu::Interrupt> compute_interrupts(const uint8_t previous_mode, const uint8_t previous_ly);
    };
}
