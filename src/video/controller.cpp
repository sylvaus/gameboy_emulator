#include "emulator/video/controller.h"
#include "emulator/sound/controller.h"


namespace emulator::video
{
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
        ram_[address - emulator::memory::START_OAM] = value;
    }

    uint8_t VideoController::get_oam(uint16_t address) const
    {
        return ram_[address - emulator::memory::START_OAM];
    }

    void VideoController::set_lcd_control(uint8_t value)
    {

    }

    uint8_t VideoController::get_lcd_control() const
    {
        return 0;
    }

    void VideoController::set_lcd_status(uint8_t value)
    {

    }

    uint8_t VideoController::get_lcd_status() const
    {
        return 0;
    }

    void VideoController::set_lcd_scroll_y(uint8_t value)
    {

    }

    uint8_t VideoController::get_lcd_scroll_y() const
    {
        return 0;
    }

    void VideoController::set_lcd_scroll_x(uint8_t value)
    {

    }

    uint8_t VideoController::get_lcd_scroll_x() const
    {
        return 0;
    }

    void VideoController::set_lcd_coordinate_y(uint8_t value)
    {

    }

    uint8_t VideoController::get_lcd_coordinate_y() const
    {
        return 0;
    }

    void VideoController::set_lcd_compare_y(uint8_t value)
    {

    }

    uint8_t VideoController::get_lcd_compare_y() const
    {
        return 0;
    }

    void VideoController::set_lcd_window_position_y(uint8_t value)
    {

    }

    uint8_t VideoController::get_lcd_window_position_y() const
    {
        return 0;
    }

    void VideoController::set_lcd_window_position_x(uint8_t value)
    {

    }

    uint8_t VideoController::get_lcd_window_position_x() const
    {
        return 0;
    }

    void VideoController::set_bg_palette_data(uint8_t value)
    {

    }

    uint8_t VideoController::get_bg_palette_data() const
    {
        return 0;
    }

    void VideoController::set_obj_palette_data_0(uint8_t value)
    {

    }

    uint8_t VideoController::get_obj_palette_data_0() const
    {
        return 0;
    }

    void VideoController::set_obj_palette_data_1(uint8_t value)
    {

    }

    uint8_t VideoController::get_obj_palette_data_1() const
    {
        return 0;
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
}
