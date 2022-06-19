#pragma once

#include <cstdint>
#include <memory>
#include <unordered_set>

#include "emulator/types.h"
#include "emulator/memory/memory_map.h"
#include "emulator/cpu/interrupt.h"
#include "renderer.h"

namespace emulator::video
{
    // As according to https://gbdev.io/pandocs/STAT.html
    // Mode 2 + 3 + 0: 456
    // Full frame time: 70224
    // * Mode 2 + 3 + 0 for 143 lines: (173 + 80 + 203) * 144
    // * Mode 1 for 10 lines: 456 * 10
    constexpr tick_t MODE_0_TICKS = 173;
    constexpr tick_t MODE_1_TICKS = 456;
    constexpr tick_t MODE_2_TICKS = 80;
    constexpr tick_t MODE_3_TICKS = 203;

    constexpr uint8_t LAST_LY_MODE_0 = 143;
    constexpr uint8_t LAST_LY_MODE_1 = 153;

    class VideoController
    {
    public:
        virtual cpu::Interrupts tick(const tick_t nb_cycles) = 0;

        virtual void set_vram(uint16_t address, uint8_t value) = 0;
        [[nodiscard]] virtual uint8_t get_vram(uint16_t address) const = 0;
        virtual void set_oam(uint16_t address, uint8_t value) = 0;
        [[nodiscard]] virtual uint8_t get_oam(uint16_t address) const = 0;
        virtual void set_lcd_control(uint8_t value) = 0;
        [[nodiscard]] virtual uint8_t get_lcd_control() const = 0;
        virtual void set_lcd_status(uint8_t value) = 0;
        [[nodiscard]] virtual uint8_t get_lcd_status() const = 0;
        virtual void set_lcd_scroll_y(uint8_t value) = 0;
        [[nodiscard]] virtual uint8_t get_lcd_scroll_y() const = 0;
        virtual void set_lcd_scroll_x(uint8_t value) = 0;
        [[nodiscard]] virtual uint8_t get_lcd_scroll_x() const = 0;
        virtual void set_lcd_coordinate_y(uint8_t value) = 0;
        [[nodiscard]] virtual uint8_t get_lcd_coordinate_y() const = 0;
        virtual void set_lcd_compare_y(uint8_t value) = 0;
        [[nodiscard]] virtual uint8_t get_lcd_compare_y() const = 0;
        virtual void set_bg_palette_data(uint8_t value) = 0;
        [[nodiscard]] virtual uint8_t get_bg_palette_data() const = 0;
        virtual void set_obj_palette_data_0(uint8_t value) = 0;
        [[nodiscard]] virtual uint8_t get_obj_palette_data_0() const = 0;
        virtual void set_obj_palette_data_1(uint8_t value) = 0;
        [[nodiscard]] virtual uint8_t get_obj_palette_data_1() const = 0;
        virtual void set_lcd_window_position_y(uint8_t value) = 0;
        [[nodiscard]] virtual uint8_t get_lcd_window_position_y() const = 0;
        virtual void set_lcd_window_position_x(uint8_t value) = 0;
        [[nodiscard]] virtual uint8_t get_lcd_window_position_x() const = 0;
        virtual void select_vram_bank(uint8_t value) = 0;
        [[nodiscard]] virtual uint8_t get_vram_bank() const = 0;
        virtual void set_cgb_bj_obj_palettes(uint16_t address, uint8_t value) = 0;
        [[nodiscard]] virtual uint8_t get_cgb_bj_obj_palettes(uint16_t address) const = 0;

        virtual ~VideoController() = default;
    };

    class VideoControllerImpl: public VideoController
    {
    public:
        VideoControllerImpl(std::shared_ptr<Renderer> renderer);

        cpu::Interrupts tick(const tick_t nb_cycles) override;

        void set_vram(uint16_t address, uint8_t value) override;
        [[nodiscard]] uint8_t get_vram(uint16_t address) const override;
        void set_oam(uint16_t address, uint8_t value) override;
        [[nodiscard]] uint8_t get_oam(uint16_t address) const override;
        void set_lcd_control(uint8_t value) override;
        [[nodiscard]] uint8_t get_lcd_control() const override;
        void set_lcd_status(uint8_t value) override;
        [[nodiscard]] uint8_t get_lcd_status() const override;
        void set_lcd_scroll_y(uint8_t value) override;
        [[nodiscard]] uint8_t get_lcd_scroll_y() const override;
        void set_lcd_scroll_x(uint8_t value) override;
        [[nodiscard]] uint8_t get_lcd_scroll_x() const override;
        void set_lcd_coordinate_y(uint8_t value) override;
        [[nodiscard]] uint8_t get_lcd_coordinate_y() const override;
        void set_lcd_compare_y(uint8_t value) override;
        [[nodiscard]] uint8_t get_lcd_compare_y() const override;
        void set_bg_palette_data(uint8_t value) override;
        [[nodiscard]] uint8_t get_bg_palette_data() const override;
        void set_obj_palette_data_0(uint8_t value) override;
        [[nodiscard]] uint8_t get_obj_palette_data_0() const override;
        void set_obj_palette_data_1(uint8_t value) override;
        [[nodiscard]] uint8_t get_obj_palette_data_1() const override;
        void set_lcd_window_position_y(uint8_t value) override;
        [[nodiscard]] uint8_t get_lcd_window_position_y() const override;
        void set_lcd_window_position_x(uint8_t value) override;
        [[nodiscard]] uint8_t get_lcd_window_position_x() const override;
        void select_vram_bank(uint8_t value) override;
        [[nodiscard]] uint8_t get_vram_bank() const override;
        void set_cgb_bj_obj_palettes(uint16_t address, uint8_t value) override;
        [[nodiscard]] uint8_t get_cgb_bj_obj_palettes(uint16_t address) const override;

        ~VideoControllerImpl() override = default;
    private:
        emulator::memory::VideoRam ram_{};
        emulator::memory::SpriteAttributeTable oam_{};
        std::shared_ptr<Renderer> renderer_;

        tick_t clock_{};
        tick_t next_action_clock_{};

        LcdAllControlAndStatus lcd_bits_{};
        bool start_frame_{};

        void update_mode();
        void perform_mode_action();

        cpu::Interrupts compute_interrupts(const uint8_t previous_mode);
    };
}
