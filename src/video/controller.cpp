#include "emulator/video/controller.h"
#include "emulator/sound/controller.h"


namespace emulator::video
{
    bool is_interrupt_triggered_by_mode(LcdStatus status);

    VideoControllerImpl::VideoControllerImpl(std::shared_ptr<Renderer> renderer): renderer_(std::move(renderer))
    {}

    cpu::Interrupts VideoControllerImpl::tick(const tick_t nb_cycles)
    {
        clock_ += nb_cycles;
        if (!lcd_bits_.control.enable_lcd)
        {
            // Nothing to do when LCD is not enabled
            return {};
        }
        if (clock_ < next_action_clock_)
        {
            return {};
        }

        clock_ -= next_action_clock_;
        const uint8_t previous_mode = lcd_bits_.status.mode;
        update_mode();
        perform_mode_action();
        return compute_interrupts(previous_mode);
    }

    cpu::Interrupts VideoControllerImpl::compute_interrupts(const uint8_t previous_mode)
    {
        cpu::Interrupts interrupts;
        if (previous_mode != lcd_bits_.status.mode)
        {
            // https://gbdev.io/pandocs/Interrupt_Sources.html#int-48---stat-interrupt
            if (is_interrupt_triggered_by_mode(lcd_bits_.status))
                interrupts.lcd_stat = true;
            // https://gbdev.io/pandocs/Interrupt_Sources.html#int-40---vblank-interrupt
            if (MODE_1_VBLANK == lcd_bits_.status.mode)
                interrupts.vblank = true;
        }

        // https://gbdev.io/pandocs/Interrupt_Sources.html#int-48---stat-interrupt
        if ((MODE_2_SEARCH_OAM == lcd_bits_.status.mode) && (lcd_bits_.coordinate_y == lcd_bits_.compare_y))
        {
            lcd_bits_.status.lyc_ly = true;
            interrupts.lcd_stat |= lcd_bits_.status.lyc_ly_interrupt;
        }
        return interrupts;
    }

    void VideoControllerImpl::update_mode()
    {
        // https://gbdev.io/pandocs/STAT.html
        //        Mode 2  2_____2_____2_____2_____2_____2___________________2____
        //        Mode 3  _33____33____33____33____33____33__________________3___
        //        Mode 0  ___000___000___000___000___000___000________________000
        //        Mode 1  ____________________________________11111111111111_____

        switch (lcd_bits_.status.mode)
        {
            case MODE_2_SEARCH_OAM:
                lcd_bits_.status.mode = MODE_3_TRANSFER;
                break;
            case MODE_3_TRANSFER:
                lcd_bits_.status.mode = MODE_0_HBLANK;
                break;
            case MODE_0_HBLANK:
                lcd_bits_.status.mode = (LAST_LY_MODE_0 == lcd_bits_.coordinate_y) ? MODE_1_VBLANK : MODE_2_SEARCH_OAM;
                break;
            case MODE_1_VBLANK:
            {
                if (LAST_LY_MODE_1 == lcd_bits_.coordinate_y)
                {
                    start_frame_ = true;
                    lcd_bits_.status.mode = MODE_2_SEARCH_OAM;
                }
            }
        }
    }

    void VideoControllerImpl::perform_mode_action()
    {
        switch (lcd_bits_.status.mode)
        {
            case MODE_2_SEARCH_OAM:
            {
                lcd_bits_.coordinate_y = start_frame_ ? 0 : (1 + lcd_bits_.coordinate_y);
                start_frame_ = false;
                next_action_clock_ = MODE_2_TICKS;
                break;
            }
            case MODE_3_TRANSFER:
                next_action_clock_ = MODE_3_TICKS;
                break;
            case MODE_0_HBLANK:
                renderer_->render_line(ram_, oam_, lcd_bits_);
                next_action_clock_ = MODE_0_TICKS;
                break;
            case MODE_1_VBLANK:
                next_action_clock_ = MODE_1_TICKS;
                ++lcd_bits_.coordinate_y;
        }
    }

    void VideoControllerImpl::set_vram(uint16_t address, uint8_t value)
    {
        ram_[address - emulator::memory::START_VRAM] = value;
    }

    uint8_t VideoControllerImpl::get_vram(uint16_t address) const
    {
        return ram_[address - emulator::memory::START_VRAM];
    }

    void VideoControllerImpl::set_oam(uint16_t address, uint8_t value)
    {
        oam_[address - emulator::memory::START_OAM] = value;
    }

    uint8_t VideoControllerImpl::get_oam(uint16_t address) const
    {
        return oam_[address - emulator::memory::START_OAM];
    }

    void VideoControllerImpl::set_lcd_control(uint8_t value)
    {
        const auto previous_lcd_enable = lcd_bits_.control.enable_lcd;
        lcd_bits_.control.value = value;

        if (!previous_lcd_enable && lcd_bits_.control.enable_lcd)
        {
            lcd_bits_.coordinate_y = 0;
            clock_ = 0;
            next_action_clock_ = 0;
            // MODE_0_HBLANK so, we would update right away to MODE_2
            lcd_bits_.status.mode = MODE_0_HBLANK;
            start_frame_ = true;
        }
        // disabling lcd
        else if (previous_lcd_enable && !lcd_bits_.control.enable_lcd)
        {
            // According to Gameboy Programming Manual page 59:
            // Writing a value of 0 to bit 7 of the LCDC register when its value is 1 stops the LCD controller, and
            // the value of register LY immediately becomes 0
            lcd_bits_.coordinate_y = 0;

            // According to https://www.reddit.com/r/Gameboy/comments/a1c8h0/what_happens_when_a_gameboy_screen_is_disabled/
            // Clock is reset to zero
            clock_ = 0;
            // LCD enters mode 0
            lcd_bits_.status.mode = MODE_0_HBLANK;
        }
    }

    uint8_t VideoControllerImpl::get_lcd_control() const
    {
        return lcd_bits_.control.value;
    }

    void VideoControllerImpl::set_lcd_status(uint8_t value)
    {
        lcd_bits_.status.value = value;
    }

    uint8_t VideoControllerImpl::get_lcd_status() const
    {
        return lcd_bits_.status.value;
    }

    void VideoControllerImpl::set_lcd_scroll_y(uint8_t value)
    {
        lcd_bits_.scroll_y = value;
    }

    uint8_t VideoControllerImpl::get_lcd_scroll_y() const
    {
        return lcd_bits_.scroll_y;
    }

    void VideoControllerImpl::set_lcd_scroll_x(uint8_t value)
    {
        lcd_bits_.scroll_x = value;
    }

    uint8_t VideoControllerImpl::get_lcd_scroll_x() const
    {
        return lcd_bits_.scroll_x;
    }

    void VideoControllerImpl::set_lcd_coordinate_y(uint8_t value)
    {
        lcd_bits_.coordinate_y = value;
    }

    uint8_t VideoControllerImpl::get_lcd_coordinate_y() const
    {
        return lcd_bits_.coordinate_y;
    }

    void VideoControllerImpl::set_lcd_compare_y(uint8_t value)
    {
        lcd_bits_.compare_y = value;
    }

    uint8_t VideoControllerImpl::get_lcd_compare_y() const
    {
        return lcd_bits_.compare_y;
    }

    void VideoControllerImpl::set_lcd_window_position_y(uint8_t value)
    {
        lcd_bits_.window_position_y = value;
    }

    uint8_t VideoControllerImpl::get_lcd_window_position_y() const
    {
        return lcd_bits_.window_position_y;
    }

    void VideoControllerImpl::set_lcd_window_position_x(uint8_t value)
    {
        lcd_bits_.window_position_x = value;
    }

    uint8_t VideoControllerImpl::get_lcd_window_position_x() const
    {
        return lcd_bits_.window_position_x;
    }

    void VideoControllerImpl::set_bg_palette_data(uint8_t value)
    {
        lcd_bits_.bg_palette_data = value;
    }

    uint8_t VideoControllerImpl::get_bg_palette_data() const
    {
        return lcd_bits_.bg_palette_data;
    }

    void VideoControllerImpl::set_obj_palette_data_0(uint8_t value)
    {
        lcd_bits_.obj_palette_data_0 = value;
    }

    uint8_t VideoControllerImpl::get_obj_palette_data_0() const
    {
        return lcd_bits_.obj_palette_data_0;
    }

    void VideoControllerImpl::set_obj_palette_data_1(uint8_t value)
    {
        lcd_bits_.obj_palette_data_1 = value;
    }

    uint8_t VideoControllerImpl::get_obj_palette_data_1() const
    {
        return lcd_bits_.obj_palette_data_1;
    }

    void VideoControllerImpl::select_vram_bank(uint8_t)
    {
        // Nothing to be done for non CGB implementation
    }

    uint8_t VideoControllerImpl::get_vram_bank() const
    {
        return 0; // Nothing to be done for non CGB implementation
    }

    void VideoControllerImpl::set_cgb_bj_obj_palettes(uint16_t, uint8_t)
    {
        // Nothing to be done for non CGB implementation
    }

    uint8_t VideoControllerImpl::get_cgb_bj_obj_palettes(uint16_t) const
    {
        return 0; // Nothing to be done for non CGB implementation
    }

    bool is_interrupt_triggered_by_mode(const LcdStatus status)
    {
        switch (status.mode)
        {
            case MODE_2_SEARCH_OAM:
                return status.mode2_interrupt;
            case MODE_0_HBLANK:
                return status.mode0_interrupt;
            case MODE_1_VBLANK:
                return status.mode1_interrupt;
            default:
                return false;
        }
    }
}
