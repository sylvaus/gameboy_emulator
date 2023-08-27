use macros::BitAccessor;

/// Structure defined here: https://gbdev.io/pandocs/LCDC.html
#[derive(BitAccessor, Debug, Copy, Clone)]
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
#[derive(BitAccessor, Debug, Copy, Clone)]
pub struct LcdStatus {
    #[bit_offset_size(enable_lyc_stat_interrupt, 6, 1)]
    #[bit_offset_size(mode2_interrupt_source, 5, 1)]
    #[bit_offset_size(mode1_interrupt_source, 4, 1)]
    #[bit_offset_size(mode0_interrupt_source, 3, 1)]
    #[bit_offset_size(lyc_ly_flag, 2, 1)]
    #[bit_offset_size(mode, 0, 2)]
    pub value: u8,
}

#[derive(Debug, Copy, Clone)]
pub struct LcdInfo {
    pub control: LcdControl,
    pub status: LcdStatus,

    pub scroll_y: u8,
    pub scroll_x: u8,
    pub coordinate_y: u8,
    pub compare_y: u8,
    pub window_position_y: u8,
    pub window_position_x: u8,
    pub bg_palette_data: u8,
    pub obj_palette_data_0: u8,
    pub obj_palette_data_1: u8,
}
