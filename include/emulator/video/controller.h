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
    class VideoController
    {
    public:
        virtual std::unordered_set<emulator::cpu::Interrupt> tick(const tick_t nb_cycles) = 0;

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
        std::unordered_set<emulator::cpu::Interrupt> tick(const tick_t nb_cycles) override;

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
    private:
        emulator::memory::VideoRam ram_{};
        emulator::memory::SpriteAttributeTable oam_{};
        std::shared_ptr<Renderer> renderer_;

        tick_t clock_;
        tick_t next_action_clock_;

        LcdAllControlAndStatus lcd_bits_;
        bool start_frame_;

        void update_mode();
        void perform_mode_action();
        bool does_mode_triggers_interrupt();

        std::unordered_set<cpu::Interrupt> compute_interrupts(const uint8_t previous_mode, const uint8_t previous_ly);
    };
}
