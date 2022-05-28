#include "emulator/exceptions.h"
#include "emulator/memory/memory.h"

namespace emulator::memory
{
    void Memory::set(uint16_t address, uint8_t value)
    {
        if (address <= END_ROM_1_N)
        {
            mbc_->set_rom(address, value);
        }
        else if (address <= END_VRAM)
        {
            video_->set_vram(address, value);
        }
        else if (address <= END_EXT_RAM)
        {
            mbc_->set_ext_ram(address, value);
        }
        else if (address <= END_SECOND_WORK_RAM)
        {
            ram_->set_wram(address, value);
        }
        else if (address <= END_ECHO_RAM)
        {
            ram_->set_echo_ram(address, value);
        }
        else if (address <= END_OAM)
        {
            video_->set_oam(address, value);
        }
        else if (address <= END_NOT_USABLE)
        {
            throw MemoryAccessError("Trying to access the not usable area of memory");
        }
        else if (address <= END_IO_REGISTER)
        {
            set_io_register(address, value);
        }
        else if (address <= END_HRAM)
        {
            ram_->set_hram(address, value);
        }
        else
        {
            interrupt_enable_register_ = value;
        }
    }

    uint8_t Memory::get(uint16_t address) const
    {
        if (address <= END_ROM_1_N)
        {
            return mbc_->get_rom(address);
        }
        else if (address <= END_VRAM)
        {
            return video_->get_vram(address);
        }
        else if (address <= END_EXT_RAM)
        {
            return mbc_->get_ext_ram(address);
        }
        else if (address <= END_SECOND_WORK_RAM)
        {
            return ram_->get_wram(address);
        }
        else if (address <= END_ECHO_RAM)
        {
            return ram_->get_echo_ram(address);
        }
        else if (address <= END_OAM)
        {
            return video_->get_oam(address);
        }
        else if (address <= END_NOT_USABLE)
        {
            throw MemoryAccessError("Trying to access the not usable area of memory");
        }
        else if (address <= END_IO_REGISTER)
        {
            return get_io_register(address);
        }
        else if (address <= END_HRAM)
        {
            return ram_->get_hram(address);
        }
        else
        {
            return interrupt_enable_register_;
        }
    }

    void Memory::set_io_register(uint16_t address, uint8_t value)
    {
        if (JOYPAD_INPUT_ADDRESS == address)
        {
            joypad_->select_buttons(value);
        }
        else if (address <= END_SERIAL_TRANSFER)
        {
            serial_->set(address, value);
        }
        else if (address <= END_TIME_DIVIDER)
        {
            timer_->set(address, value);
        }
        else if (address <= END_SOUND)
        {
            sound_->set_sound(address, value);
        }
        else if (address <= END_WAVE_PATTERN)
        {
            sound_->set_wave_pattern(address, value);
        }
        else if (address <= END_IO_LCD)
        {
            set_lcd(address, value);
        }
        else if (SELECT_VRAM_BANK_ADDRESS == address)
        {
            video_->select_vram_bank(value);
        }
        else if (DISABLE_BOOT_ROM_ADDRESS == address)
        {
            boot_rom_disabled_ = value;
        }
        else if (address <= END_VRAM_DMA)
        {
            handle_vram_dma(address, value);
        }
        else if (address <= END_BJ_OBJ_PALETTES)
        {
            video_->set_cgb_bj_obj_palettes(address, value);
        }
        else if (SELECT_WRAM_BANK_ADDRESS == address)
        {
            ram_->select_wram_bank(value);
        }
        else
        {
            throw MemoryAccessError("Not handled memory access for address: " + std::to_string(address));
        }
    }

    uint8_t Memory::get_io_register(uint16_t address) const
    {
        if (JOYPAD_INPUT_ADDRESS == address)
        {
            return joypad_->get_flags();
        }
        else if (address <= END_SERIAL_TRANSFER)
        {
            return serial_->get(address);
        }
        else if (address <= END_TIME_DIVIDER)
        {
            return timer_->get(address);
        }
        else if (address <= END_SOUND)
        {
            return sound_->get_sound(address);
        }
        else if (address <= END_WAVE_PATTERN)
        {
            return sound_->get_wave_pattern(address);
        }
        else if (address <= END_IO_LCD)
        {
            return get_lcd(address);
        }
        else if (SELECT_VRAM_BANK_ADDRESS == address)
        {
            return video_->get_vram_bank();
        }
        else if (DISABLE_BOOT_ROM_ADDRESS == address)
        {
            return boot_rom_disabled_;
        }
        else if (address <= END_VRAM_DMA)
        {
            // https://gbdev.io/pandocs/CGB_Registers.html#lcd-vram-dma-transfers
            throw MemoryAccessError("VRAM DMA is read only");
        }
        else if (address <= END_BJ_OBJ_PALETTES)
        {
            return video_->get_cgb_bj_obj_palettes(address);
        }
        else if (SELECT_WRAM_BANK_ADDRESS == address)
        {
            return ram_->get_wram_selected();
        }
        else
        {
            throw MemoryAccessError("Not handled memory access for address: " + std::to_string(address));
        }
    }

    void Memory::handle_vram_dma(uint16_t address, uint8_t value)
    {
        // Nothing to do for non CGB
    }

    void Memory::set_lcd(uint16_t address, uint8_t value)
    {
        switch (address)
        {
            case LCD_CONTROL_ADDRESS:
                video_->set_lcd_control(value);
                break;
            case LCD_STATUS_ADDRESS:
                video_->set_lcd_status(value);
                break;
            case LCD_SCROLL_Y_ADDRESS:
                video_->set_lcd_scroll_y(value);
                break;
            case LCD_SCROLL_X_ADDRESS:
                video_->set_lcd_scroll_x(value);
                break;
            case LCD_COORDINATE_Y_ADDRESS:
                video_->set_lcd_coordinate_y(value);
                break;
            case LCD_LY_COMPARE_ADDRESS:
                video_->set_lcd_compare_y(value);
                break;
            case OAM_DMA_ADDRESS:
                set_oam_dma(value);
                break;
            case BGP_PALETTE_DATA_ADDRESS:
                video_->set_bg_palette_data(value);
                break;
            case OBJ_PALETTE_DATA_0_ADDRESS:
                video_->set_obj_palette_data_0(value);
                break;
            case OBJ_PALETTE_DATA_1_ADDRESS:
                video_->set_obj_palette_data_1(value);
                break;
            case LCD_WINDOWS_Y_ADDRESS:
                video_->set_lcd_window_position_y(value);
                break;
            case LCD_WINDOWS_X_ADDRESS:
                video_->set_lcd_window_position_x(value);
                break;
            default:
                throw MemoryAccessError("Invalid lcd address: " + std::to_string(address));
        }

    }

    uint8_t Memory::get_lcd(uint16_t address) const
    {
        switch (address)
        {
            case LCD_CONTROL_ADDRESS:
                return video_->get_lcd_control();
            case LCD_STATUS_ADDRESS:
                return video_->get_lcd_status();
            case LCD_SCROLL_Y_ADDRESS:
                return video_->get_lcd_scroll_y();
            case LCD_SCROLL_X_ADDRESS:
                return video_->get_lcd_scroll_x();
            case LCD_COORDINATE_Y_ADDRESS:
                return video_->get_lcd_coordinate_y();
            case LCD_LY_COMPARE_ADDRESS:
                return video_->get_lcd_compare_y();
            case OAM_DMA_ADDRESS:
                return get_oam_dma();
            case BGP_PALETTE_DATA_ADDRESS:
                return video_->get_bg_palette_data();
            case OBJ_PALETTE_DATA_0_ADDRESS:
                return video_->get_obj_palette_data_0();
            case OBJ_PALETTE_DATA_1_ADDRESS:
                return video_->get_obj_palette_data_1();
            case LCD_WINDOWS_Y_ADDRESS:
                return video_->get_lcd_window_position_y();
            case LCD_WINDOWS_X_ADDRESS:
                return video_->get_lcd_window_position_x();
            default:
                throw MemoryAccessError("Invalid lcd address: " + std::to_string(address));
        }
    }

    void Memory::set_oam_dma(uint8_t value)
    {
        // This should add some machine cycles somewhere
        // Source: https://gbdev.io/pandocs/OAM_DMA_Transfer.html
        oam_dma_high_bits_ = value;
        uint16_t high_bits = value << 8;
        for (uint16_t i = 0; i < OAM_SIZE; ++i)
            set(START_OAM + i, get(high_bits + i));
    }

    uint8_t Memory::get_oam_dma() const
    {
        return oam_dma_high_bits_;
    }
}