use macros::BitAccessor;

/// Structure defined here: https://gbdev.io/pandocs/LCDC.html
#[derive(BitAccessor, Debug, Copy, Clone, Default)]
pub struct LcdControl {
    #[bit_offset_size(lcd_enable, 7, 1)]
    #[bit_offset_size(window_tile_map_area, 6, 1)]
    #[bit_offset_size(window_enable, 5, 1)]
    #[bit_offset_size(bg_window_tile_data_area, 4, 1)]
    #[bit_offset_size(bg_tile_map_area, 3, 1)]
    #[bit_offset_size(obj_size, 2, 1)]
    #[bit_offset_size(obj_enable, 1, 1)]
    #[bit_offset_size(bg_window_enable, 0, 1)]
    pub value: u8,
}

pub const MODE_0_HBLANK: u8 = 0;
pub const MODE_1_VBLANK: u8 = 1;
pub const MODE_2_SEARCH_OAM: u8 = 2;
pub const MODE_3_TRANSFER: u8 = 3;

/// Structure defined here: https://gbdev.io/pandocs/STAT.html
#[derive(BitAccessor, Debug, Copy, Clone, Default)]
pub struct LcdStatus {
    #[bit_offset_size(enable_lyc_stat_interrupt, 6, 1)]
    #[bit_offset_size(mode2_interrupt_source, 5, 1)]
    #[bit_offset_size(mode1_interrupt_source, 4, 1)]
    #[bit_offset_size(mode0_interrupt_source, 3, 1)]
    #[bit_offset_size(lyc_ly_flag, 2, 1)]
    #[bit_offset_size(mode, 0, 2)]
    pub value: u8,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lcd_control_bg_window_enable() {
        let value = LcdControl{value: 0xFE};
        assert_eq!(value.read_bg_window_enable(), 0);
        let value = LcdControl{value: 0x01};
        assert_eq!(value.read_bg_window_enable(), 1);
    }

    #[test]
    fn lcd_control_lcd_enable() {
        let value = LcdControl{value: 0x7F};
        assert_eq!(value.read_lcd_enable(), 0);
        let value = LcdControl{value: 0x80};
        assert_eq!(value.read_lcd_enable(), 1);
    }

    #[test]
    fn lcd_status_mode() {
        let value = LcdStatus{value: 0xFC};
        assert_eq!(value.read_mode(), 0);
        let value = LcdStatus{value: 0x01};
        assert_eq!(value.read_mode(), 1);
        let value = LcdStatus{value: 0x02};
        assert_eq!(value.read_mode(), 2);
    }

    #[test]
    fn lcd_status_enable_lyc_stat_interrupt() {
        let value = LcdStatus{value: 0b10111111};
        assert_eq!(value.read_enable_lyc_stat_interrupt(), 0);
        let value = LcdStatus{value: 0b01000000};
        assert_eq!(value.read_enable_lyc_stat_interrupt(), 1);
    }
}
