use crate::video::memory::{LcdControl, LcdStatus};

/// Information from: https://gbdev.io/pandocs/Memory_Map.html#memory-map
pub const VRAM_SIZE: usize = 0x2000;
pub const VRAM_START_ADDRESS: u16 = 0x8000;
pub const VRAM_END_ADDRESS: u16 = 0x9FFF;
pub const OAM_SIZE: usize = 0xA0;
pub const OAM_START_ADDRESS: u16 = 0xFE00;
pub const OAM_END_ADDRESS: u16 = 0xFE9F;

/// Information from:  https://gbdev.io/pandocs/Memory_Map.html#io-ranges
pub const IO_LCD_START_ADDRESS: u16 = 0xFF40;
/// Information from: https://gbdev.io/pandocs/LCDC.html
pub const LCD_CONTROL_ADDRESS: u16 = 0xFF40;
/// Information from: https://gbdev.io/pandocs/STAT.html
pub const LCD_STATUS_ADDRESS: u16 = 0xFF41;
/// Information from: https://gbdev.io/pandocs/Scrolling.html#lcd-position-and-scrolling
pub const LCD_SCROLL_Y_ADDRESS: u16 = 0xFF42;
pub const LCD_SCROLL_X_ADDRESS: u16 = 0xFF43;
/// Information from: https://gbdev.io/pandocs/STAT.html
pub const LCD_COORDINATE_Y_ADDRESS: u16 = 0xFF44;
pub const LCD_LY_COMPARE_ADDRESS: u16 = 0xFF45;
/// Information from: https://gbdev.io/pandocs/OAM_DMA_Transfer.html
pub const OAM_DMA_ADDRESS: u16 = 0xFF46;
/// Information from: https://gbdev.io/pandocs/Palettes.html
pub const BGP_PALETTE_DATA_ADDRESS: u16 = 0xFF47;
pub const OBJ_PALETTE_DATA_0_ADDRESS: u16 = 0xFF48;
pub const OBJ_PALETTE_DATA_1_ADDRESS: u16 = 0xFF49;
/// Information from: https://gbdev.io/pandocs/Scrolling.html#ff4aff4b--wy-wx-window-y-position-x-position-plus-7
pub const LCD_WINDOWS_Y_ADDRESS: u16 = 0xFF4A;
pub const LCD_WINDOWS_X_ADDRESS: u16 = 0xFF4B;
pub const IO_LCD_END_ADDRESS: u16 = 0xFF4B;
pub const VRAM_BANK_SELECT: u16 = 0xFF4F;

/// Information from: https://gbdev.io/pandocs/Memory_Map.html#memory-map
pub const BG_OBJ_PALETTES_START_ADDRESS: u16 = 0xFF68;
pub const BG_OBJ_PALETTES__END_ADDRESS: u16 = 0xFF6B;

pub struct VideoController {
    vram: Vec<u8>,
    oam: Vec<u8>,
    control: LcdControl,
    // This is necessary to handle enabling/disabling lcd.
    previous_control: LcdControl,
    status: LcdStatus,

    scroll_y: u8,
    scroll_x: u8,
    coordinate_y: u8,
    compare_y: u8,
    bg_palette_data: u8,
    obj_palette_data_0: u8,
    obj_palette_data_1: u8,
    window_position_y: u8,
    window_position_x: u8,
}

impl VideoController {
    pub fn new() -> Self {
        Self {
            vram: vec![0u8; VRAM_SIZE],
            oam: vec![0u8; OAM_SIZE],

            control: Default::default(),
            previous_control: Default::default(),
            status: Default::default(),

            scroll_y: Default::default(),
            scroll_x: Default::default(),
            coordinate_y: Default::default(),
            compare_y: Default::default(),
            bg_palette_data: Default::default(),
            obj_palette_data_0: Default::default(),
            obj_palette_data_1: Default::default(),
            window_position_y: Default::default(),
            window_position_x: Default::default(),
        }
    }

    pub fn tick(&mut self, nb_cycles: usize) {
        // Enable LCD
        if self.previous_control.read_lcd_enable() == 0 && self.control.read_lcd_enable() == 1 {

            // Disable LCD
        } else if self.previous_control.read_lcd_enable() == 1
            && self.control.read_lcd_enable() == 0
        {
        }
        self.previous_control = self.control;

        todo!("Finish implementation")
    }

    pub fn write_vram(&mut self, address: u16, value: u8) {
        self.vram[(address - VRAM_START_ADDRESS) as usize] = value;
    }

    pub fn read_vram(&self, address: u16) -> u8 {
        self.vram[(address - VRAM_START_ADDRESS) as usize]
    }

    pub fn write_oam(&mut self, address: u16, value: u8) {
        self.oam[(address - OAM_START_ADDRESS) as usize] = value;
    }

    pub fn read_oam(&self, address: u16) -> u8 {
        self.oam[(address - OAM_START_ADDRESS) as usize]
    }

    pub fn write_lcd(&mut self, address: u16, value: u8) {
        match address {
            LCD_CONTROL_ADDRESS => self.control.value = value,
            LCD_STATUS_ADDRESS => self.status.value = value,
            LCD_SCROLL_Y_ADDRESS => self.scroll_y = value,
            LCD_SCROLL_X_ADDRESS => self.scroll_x = value,
            LCD_COORDINATE_Y_ADDRESS => self.coordinate_y = value,
            LCD_LY_COMPARE_ADDRESS => self.compare_y = value,
            BGP_PALETTE_DATA_ADDRESS => self.bg_palette_data = value,
            OBJ_PALETTE_DATA_0_ADDRESS => self.obj_palette_data_0 = value,
            OBJ_PALETTE_DATA_1_ADDRESS => self.obj_palette_data_1 = value,
            LCD_WINDOWS_Y_ADDRESS => self.window_position_y = value,
            LCD_WINDOWS_X_ADDRESS => self.window_position_x = value,
            _ => panic!("Address {} is not valid for the video controller", address),
        }
    }

    pub fn read_lcd(&self, address: u16) -> u8 {
        match address {
            LCD_CONTROL_ADDRESS => self.control.value,
            LCD_STATUS_ADDRESS => self.status.value,
            LCD_SCROLL_Y_ADDRESS => self.scroll_y,
            LCD_SCROLL_X_ADDRESS => self.scroll_x,
            LCD_COORDINATE_Y_ADDRESS => self.coordinate_y,
            LCD_LY_COMPARE_ADDRESS => self.compare_y,
            BGP_PALETTE_DATA_ADDRESS => self.bg_palette_data,
            OBJ_PALETTE_DATA_0_ADDRESS => self.obj_palette_data_0,
            OBJ_PALETTE_DATA_1_ADDRESS => self.obj_palette_data_1,
            LCD_WINDOWS_Y_ADDRESS => self.window_position_y,
            LCD_WINDOWS_X_ADDRESS => self.window_position_x,
            _ => panic!("Address {} is not valid for the video controller", address),
        }
    }

    pub fn write_vram_bank(&mut self, _value: u8) {
        // TODO: implement CGB mode https://gbdev.io/pandocs/CGB_Registers.html#ff4f--vbk-cgb-mode-only-vram-bank
    }

    pub fn read_vram_bank(&self) -> u8 {
        // TODO: implement CGB mode https://gbdev.io/pandocs/CGB_Registers.html#ff4f--vbk-cgb-mode-only-vram-bank
        0
    }

    pub fn write_cgb_lcd_color_palette(&mut self, _address: u16, _value: u8) {
        // TODO: implement CGB mode https://gbdev.io/pandocs/Palettes.html#lcd-color-palettes-cgb-only
    }

    pub fn read_cgb_lcd_color_palette(&self, _address: u16) -> u8 {
        // TODO: implement CGB mode https://gbdev.io/pandocs/Palettes.html#lcd-color-palettes-cgb-only
        0
    }
}
