#include "emulator/video/controller.h"
#include "emulator/sound/controller.h"


namespace emulator::video
{
    // As according to https://gbdev.io/pandocs/STAT.html
    // Mode 2 + 3 + 0: 456
    constexpr tick_t MODE_0_TICKS = 173;
    constexpr tick_t MODE_1_TICKS = 456;
    constexpr tick_t MODE_2_TICKS = 80;
    constexpr tick_t MODE_3_TICKS = 202;

    constexpr uint8_t LAST_LY_MODE_0 = 143;
    constexpr uint8_t LAST_LY_MODE_1 = 153;

    std::unordered_set<emulator::cpu::Interrupt> VideoController::tick(const tick_t nb_cycles)
    {
        if (!lcd_control_.enable_lcd)
        {
            // Nothing to do when LCD is not enabled
            return {};
        }
        if (clock_ < next_action_clock_)
        {
            return {};
        }

        clock_ -= next_action_clock_;
        const uint8_t previous_mode = lcd_status_.mode;
        const uint8_t previous_ly = lcd_coordinate_y_;

        update_mode();
        perform_mode_action();
        return compute_interrupts(previous_mode, previous_ly);
    }

    std::unordered_set<cpu::Interrupt>
    VideoController::compute_interrupts(const uint8_t previous_mode, const uint8_t previous_ly)
    {
        std::unordered_set<cpu::Interrupt> interrupts;
        if (previous_mode != lcd_status_.mode)
        {
            // https://gbdev.io/pandocs/Interrupt_Sources.html#int-48---stat-interrupt
            if (does_mode_triggers_interrupt())
                interrupts.insert(cpu::Interrupt::LCD_STAT);
            // https://gbdev.io/pandocs/Interrupt_Sources.html#int-40---vblank-interrupt
            if (MODE_3_TRANSFER == lcd_status_.mode)
                interrupts.insert(cpu::Interrupt::VBLANK);
        }

        // https://gbdev.io/pandocs/Interrupt_Sources.html#int-48---stat-interrupt
        if ((previous_ly != lcd_coordinate_y_) && (lcd_coordinate_y_ == lcd_compare_y_))
            interrupts.insert(cpu::Interrupt::LCD_STAT);
        return interrupts;
    }

    void VideoController::update_mode()
    {
        // https://gbdev.io/pandocs/STAT.html
        //        Mode 2  2_____2_____2_____2_____2_____2___________________2____
        //        Mode 3  _33____33____33____33____33____33__________________3___
        //        Mode 0  ___000___000___000___000___000___000________________000
        //        Mode 1  ____________________________________11111111111111_____

        switch (lcd_status_.mode)
        {
            case MODE_2_SEARCH_OAM:
                lcd_status_.mode = MODE_3_TRANSFER;
                break;
            case MODE_3_TRANSFER:
                lcd_status_.mode = MODE_0_HBLANK;
                break;
            case MODE_0_HBLANK:
                lcd_status_.mode = (LAST_LY_MODE_0 == lcd_coordinate_y_) ? MODE_1_VBLANK : MODE_3_TRANSFER;
                break;
            case MODE_1_VBLANK:
            {
                if (LAST_LY_MODE_1 == lcd_coordinate_y_)
                {
                    lcd_status_.mode = MODE_2_SEARCH_OAM;
                }
            }
        }
    }

    void VideoController::perform_mode_action()
    {
        switch (lcd_status_.mode)
        {
            case MODE_2_SEARCH_OAM:
                ++lcd_coordinate_y_;
                next_action_clock_ = MODE_2_TICKS;
                break;
            case MODE_3_TRANSFER:
                next_action_clock_ = MODE_3_TICKS;
                break;
            case MODE_0_HBLANK:
                // TODO: render line
                next_action_clock_ = MODE_0_TICKS;
                break;
            case MODE_1_VBLANK:
                next_action_clock_ = MODE_1_TICKS;
                ++lcd_coordinate_y_;
        }
    }

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
        oam_[address - emulator::memory::START_OAM] = value;
    }

    uint8_t VideoController::get_oam(uint16_t address) const
    {
        return oam_[address - emulator::memory::START_OAM];
    }

    void VideoController::set_lcd_control(uint8_t value)
    {
        const auto previous_lcd_enable = lcd_control_.enable_lcd;
        lcd_control_.value = value;

        if (!previous_lcd_enable && lcd_control_.enable_lcd)
        {
            lcd_coordinate_y_ = 0;
            clock_ = 0;
            lcd_status_.mode = MODE_2_SEARCH_OAM;
            start_frame = true;
        }
        // disabling lcd
        else if (previous_lcd_enable && !lcd_control_.enable_lcd)
        {
            // According to Gameboy Programming Manual page 59:
            // Writing a value of 0 to bit 7 of the LCDC register when its value is 1 stops the LCD controller, and
            // the value of register LY immediately becomes 0
            lcd_coordinate_y_ = 0;

            // According to https://www.reddit.com/r/Gameboy/comments/a1c8h0/what_happens_when_a_gameboy_screen_is_disabled/
            // Clock is reset to zero
            clock_ = 0;
            // LCD enters mode 0
            lcd_status_.mode = MODE_0_HBLANK;
        }
    }

    uint8_t VideoController::get_lcd_control() const
    {
        return lcd_control_.value;
    }

    void VideoController::set_lcd_status(uint8_t value)
    {
        lcd_status_.value = value;
    }

    uint8_t VideoController::get_lcd_status() const
    {
        return lcd_status_.value;
    }

    void VideoController::set_lcd_scroll_y(uint8_t value)
    {
        lcd_scroll_y_ = value;
    }

    uint8_t VideoController::get_lcd_scroll_y() const
    {
        return lcd_scroll_y_;
    }

    void VideoController::set_lcd_scroll_x(uint8_t value)
    {
        lcd_scroll_x_ = value;
    }

    uint8_t VideoController::get_lcd_scroll_x() const
    {
        return lcd_scroll_x_;
    }

    void VideoController::set_lcd_coordinate_y(uint8_t value)
    {
        lcd_coordinate_y_ = value;
    }

    uint8_t VideoController::get_lcd_coordinate_y() const
    {
        return lcd_coordinate_y_;
    }

    void VideoController::set_lcd_compare_y(uint8_t value)
    {
        lcd_compare_y_ = value;
    }

    uint8_t VideoController::get_lcd_compare_y() const
    {
        return lcd_compare_y_;
    }

    void VideoController::set_lcd_window_position_y(uint8_t value)
    {
        lcd_window_position_y_ = value;
    }

    uint8_t VideoController::get_lcd_window_position_y() const
    {
        return lcd_window_position_y_;
    }

    void VideoController::set_lcd_window_position_x(uint8_t value)
    {
        lcd_window_position_x_ = value;
    }

    uint8_t VideoController::get_lcd_window_position_x() const
    {
        return lcd_window_position_x_;
    }

    void VideoController::set_bg_palette_data(uint8_t value)
    {
        bg_palette_data_ = value;
    }

    uint8_t VideoController::get_bg_palette_data() const
    {
        return bg_palette_data_;
    }

    void VideoController::set_obj_palette_data_0(uint8_t value)
    {
        obj_palette_data_0_ = value;
    }

    uint8_t VideoController::get_obj_palette_data_0() const
    {
        return obj_palette_data_0_;
    }

    void VideoController::set_obj_palette_data_1(uint8_t value)
    {
        obj_palette_data_1_ = value;
    }

    uint8_t VideoController::get_obj_palette_data_1() const
    {
        return obj_palette_data_1_;
    }

    void VideoController::select_vram_bank(uint8_t value)
    {
        // Nothing to be done for non CGB implementation
    }

    uint8_t VideoController::get_vram_bank() const
    {
        return 0; // Nothing to be done for non CGB implementation
    }

    void VideoController::set_cgb_bj_obj_palettes(uint16_t address, uint8_t value)
    {
        // Nothing to be done for non CGB implementation
    }

    uint8_t VideoController::get_cgb_bj_obj_palettes(uint16_t address) const
    {
        return 0; // Nothing to be done for non CGB implementation
    }

    bool VideoController::does_mode_triggers_interrupt()
    {
        switch (lcd_status_.mode)
        {
            case MODE_2_SEARCH_OAM:
                return lcd_status_.mode2_interrupt;
            case MODE_0_HBLANK:
                return lcd_status_.mode0_interrupt;
            case MODE_1_VBLANK:
                return lcd_status_.mode1_interrupt;
            default:
                return false;
        }
    }
}
