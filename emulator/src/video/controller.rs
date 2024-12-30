use crate::interrupts::Interrupt;
use crate::video::memory::{LcdControl, LcdStatus};
use macros::BitAccessor;

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

pub const MODE_0_HBLANK_VALUE: u8 = 0;
pub const MODE_1_VBLANK_VALUE: u8 = 1;
pub const MODE_2_SEARCH_OAM_VALUE: u8 = 2;
pub const MODE_3_TRANSFER_VALUE: u8 = 3;

/// As according to https://gbdev.io/pandocs/STAT.html
/// Mode 2 + 3 + 0: 456
/// Full frame time: 70224
/// * Mode 2 + 3 + 0 for 144 lines (0-143): (173 + 80 + 203) * 144
/// * Mode 1 for 10 lines (144-153): 456 * 10
/// (173 + 80 + 203) * 144 + 456 * 10 = 70224
pub const MODE_0_HBLANK_CYCLES: u64 = 173;
pub const MODE_1_VBLANK_CYCLES: u64 = 456;
pub const MODE_2_SEARCH_OAM_CYCLES: u64 = 80;
pub const MODE_3_TRANSFER_CYCLES: u64 = 203;

pub const MODE_0_HBLANK: VideoMode = VideoMode {
    value: MODE_0_HBLANK_VALUE,
    nb_cycles: MODE_0_HBLANK_CYCLES,
};
pub const MODE_1_VBLANK: VideoMode = VideoMode {
    value: MODE_1_VBLANK_VALUE,
    nb_cycles: MODE_1_VBLANK_CYCLES,
};
pub const MODE_2_SEARCH_OAM: VideoMode = VideoMode {
    value: MODE_2_SEARCH_OAM_VALUE,
    nb_cycles: MODE_2_SEARCH_OAM_CYCLES,
};
pub const MODE_3_TRANSFER: VideoMode = VideoMode {
    value: MODE_3_TRANSFER_VALUE,
    nb_cycles: MODE_3_TRANSFER_CYCLES,
};

pub const MAX_MOD_0_2_3_Y: u8 = 143;

pub const MAX_MOD_1_Y: u8 = 153;

pub struct VideoMode {
    pub value: u8,
    pub nb_cycles: u64,
}

/// Structure defined here: https://gbdev.io/pandocs/Palettes.html
#[derive(BitAccessor, Debug, Copy, Clone, Default)]
pub struct MonochromePalette {
    #[bit_offset_size(color_3, 6, 2)]
    #[bit_offset_size(color_2, 4, 2)]
    #[bit_offset_size(color_1, 2, 2)]
    #[bit_offset_size(color_0, 0, 2)]
    pub value: u8,
}

#[derive(Debug, Copy, Clone, Default)]
struct Triggers {
    pub init_lcd: bool,
    pub should_scanline: bool,
    pub should_update_frame: bool,
}

impl Triggers {
    pub fn new() -> Self {
        Self {
            init_lcd: false,
            should_scanline: false,
            should_update_frame: false,
        }
    }

    pub fn reset(&mut self) {
        self.init_lcd = false;
        self.should_scanline = false;
        self.should_update_frame = false;
    }
}

#[derive(Debug, Copy, Clone, Default)]
pub struct Coordinates {
    pub scroll_y: u8,
    pub scroll_x: u8,
    pub y: u8,
    pub compare_y: u8,
    pub window_position_y: u8,
    pub window_position_x: u8,
}

impl Coordinates {
    pub fn get_lyc_ly_flag(&self) -> bool {
        self.y == self.compare_y
    }
}

pub struct VideoController {
    vram: Vec<u8>,
    oam: Vec<u8>,
    control: LcdControl,
    status: LcdStatus,
    triggers: Triggers,

    coordinates: Coordinates,

    bg_palette_data: MonochromePalette,
    obj_palette_data_0: MonochromePalette,
    obj_palette_data_1: MonochromePalette,

    cycles: u64,
    next_cycles_event: u64,
}

impl VideoController {
    pub fn new() -> Self {
        Self {
            vram: vec![0u8; VRAM_SIZE],
            oam: vec![0u8; OAM_SIZE],

            control: Default::default(),
            status: Default::default(),
            triggers: Triggers::new(),

            coordinates: Coordinates::default(),

            bg_palette_data: Default::default(),
            obj_palette_data_0: Default::default(),
            obj_palette_data_1: Default::default(),

            cycles: 0,
            next_cycles_event: 0,
        }
    }

    /// Function to call before starting the emulator if data was written to the VideoController
    ///
    /// This function ensures that the change detection works as expected.
    pub fn init(&mut self) {
        if self.coordinates.y <= MAX_MOD_0_2_3_Y {
            self.update_mode(MODE_2_SEARCH_OAM);
        } else {
            self.update_mode(MODE_1_VBLANK);
        }
    }

    pub fn update(&mut self, nb_cycles: u64) -> Vec<Interrupt> {
        // Change from Enable/Disable LCD
        if self.triggers.init_lcd {
            // According to Gameboy Programming Manual page 59:
            // Writing a value of 0 to bit 7 of the LCDC register when its value is 1 stops the LCD controller, and
            // the value of register LY immediately becomes 0
            self.coordinates.y = 0;
            // According to https://www.reddit.com/r/Gameboy/comments/a1c8h0/what_happens_when_a_gameboy_screen_is_disabled/
            // Clock is reset to zero
            self.cycles = 0;
            self.next_cycles_event =
                MODE_0_HBLANK_CYCLES + MODE_2_SEARCH_OAM_CYCLES + MODE_3_TRANSFER_CYCLES;
            self.status.write_mode(MODE_0_HBLANK_VALUE)
        }
        self.triggers.reset();

        // LCD not enabled
        if self.control.read_lcd_enable() == 0 {
            return Vec::default();
        }

        self.cycles += nb_cycles;
        // TODO: Probably should need to have a loop handling all the events if nb_cycles is big
        // Nothing to process
        if self.cycles < self.next_cycles_event {
            return Vec::default();
        }
        self.cycles -= self.next_cycles_event;

        // https://gbdev.io/pandocs/STAT.html
        //        Mode 2  2_____2_____2_____2_____2_____2___________________2____
        //        Mode 3  _33____33____33____33____33____33__________________3___
        //        Mode 0  ___000___000___000___000___000___000________________000
        //        Mode 1  ____________________________________11111111111111_____
        let previous_y = self.coordinates.y;
        let previous_mode = self.status.read_mode();
        match previous_mode {
            MODE_2_SEARCH_OAM_VALUE => self.update_mode(MODE_3_TRANSFER),
            MODE_3_TRANSFER_VALUE => self.update_mode(MODE_0_HBLANK),
            MODE_0_HBLANK_VALUE => {
                self.coordinates.y += 1;
                self.triggers.should_scanline = true;
                if self.coordinates.y <= MAX_MOD_0_2_3_Y {
                    self.update_mode(MODE_2_SEARCH_OAM);
                } else {
                    self.update_mode(MODE_1_VBLANK);
                }
            }
            MODE_1_VBLANK_VALUE => {
                self.coordinates.y += 1;
                if self.coordinates.y <= MAX_MOD_1_Y {
                    self.update_mode(MODE_1_VBLANK);
                } else {
                    self.coordinates.y = 0;
                    self.triggers.should_update_frame = true;
                    self.update_mode(MODE_2_SEARCH_OAM);
                }
            }
            _ => unreachable!("There are only 4 video modes"),
        }

        self.status
            .write_lyc_ly_flag(self.coordinates.get_lyc_ly_flag() as u8);

        let mut interrupts = Vec::new();
        if self.is_stat_interrupt_triggered(previous_y, previous_mode) {
            interrupts.push(Interrupt::LCDStat);
        }
        if (previous_mode != self.status.read_mode())
            && (self.status.read_mode() == MODE_1_VBLANK_VALUE)
        {
            interrupts.push(Interrupt::VBlank);
        }

        interrupts
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
            LCD_CONTROL_ADDRESS => {
                let old = self.control.read_lcd_enable();
                self.control.value = value;
                self.triggers.init_lcd = (old ^ self.control.read_lcd_enable()) == 1;
            },
            LCD_STATUS_ADDRESS => self.status.write(value),
            LCD_SCROLL_Y_ADDRESS => self.coordinates.scroll_y = value,
            LCD_SCROLL_X_ADDRESS => self.coordinates.scroll_x = value,
            LCD_COORDINATE_Y_ADDRESS => self.coordinates.y = value,
            LCD_LY_COMPARE_ADDRESS => self.coordinates.compare_y = value,
            BGP_PALETTE_DATA_ADDRESS => self.bg_palette_data.value = value,
            OBJ_PALETTE_DATA_0_ADDRESS => self.obj_palette_data_0.value = value,
            OBJ_PALETTE_DATA_1_ADDRESS => self.obj_palette_data_1.value = value,
            LCD_WINDOWS_Y_ADDRESS => self.coordinates.window_position_y = value,
            LCD_WINDOWS_X_ADDRESS => self.coordinates.window_position_x = value,
            _ => panic!("Address {} is not valid for the video controller", address),
        }
    }

    pub fn read_lcd(&self, address: u16) -> u8 {
        match address {
            LCD_CONTROL_ADDRESS => self.control.value,
            LCD_STATUS_ADDRESS => self.status.read(),
            LCD_SCROLL_Y_ADDRESS => self.coordinates.scroll_y,
            LCD_SCROLL_X_ADDRESS => self.coordinates.scroll_x,
            LCD_COORDINATE_Y_ADDRESS => self.coordinates.y,
            LCD_LY_COMPARE_ADDRESS => self.coordinates.compare_y,
            BGP_PALETTE_DATA_ADDRESS => self.bg_palette_data.value,
            OBJ_PALETTE_DATA_0_ADDRESS => self.obj_palette_data_0.value,
            OBJ_PALETTE_DATA_1_ADDRESS => self.obj_palette_data_1.value,
            LCD_WINDOWS_Y_ADDRESS => self.coordinates.window_position_y,
            LCD_WINDOWS_X_ADDRESS => self.coordinates.window_position_x,
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

    /// Indicates that the renderer can start generating the image for the current line.
    pub fn should_scanline(&self) -> bool {
        // See: https://gbdev.io/pandocs/Rendering.html?highlight=frame%20update#ppu-modes
        self.triggers.should_scanline
    }

    pub fn should_update_frame(&self) -> bool {
        // See: https://gbdev.io/pandocs/Rendering.html?highlight=frame%20update#ppu-modes
        self.triggers.should_update_frame
    }

    fn update_mode(&mut self, mode: VideoMode) {
        self.status.write_mode(mode.value);
        self.next_cycles_event = mode.nb_cycles;
    }

    fn is_stat_interrupt_triggered(&self, previous_y: u8, previous_mode: u8) -> bool {
        // https://gbdev.io/pandocs/Interrupt_Sources.html#int-48---stat-interrupt
        if (previous_y != self.coordinates.y)
            && (self.status.read_lyc_ly_flag() == 1)
            && (self.status.read_enable_lyc_stat_interrupt() == 1)
        {
            return true;
        }
        let current_mode = self.status.read_mode();
        if previous_mode != current_mode {
            return match current_mode {
                MODE_0_HBLANK_VALUE => self.status.read_mode0_interrupt_source() == 1,
                MODE_1_VBLANK_VALUE => self.status.read_mode1_interrupt_source() == 1,
                MODE_2_SEARCH_OAM_VALUE => self.status.read_mode2_interrupt_source() == 1,
                _ => false,
            };
        }
        false
    }

    pub fn get_vram(&self) -> &[u8] {
        &self.vram
    }

    pub fn get_oam(&self) -> &[u8] {
        &self.oam
    }

    pub fn get_control(&self) -> &LcdControl {
        &self.control
    }

    pub fn get_coordinates(&self) -> &Coordinates {
        &self.coordinates
    }

    pub fn get_bg_palette_data(&self) -> &MonochromePalette {
        &self.bg_palette_data
    }

    pub fn get_obj_palette_data_0(&self) -> &MonochromePalette {
        &self.obj_palette_data_0
    }

    pub fn get_obj_palette_data_1(&self) -> &MonochromePalette {
        &self.obj_palette_data_1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn update_cycles_through_modes() {
        let mut controller = VideoController::new();
        controller.control.write_lcd_enable(1);
        controller.update_mode(MODE_2_SEARCH_OAM);
        controller.init();

        // Mode 2  2_____2_____2_____2_____2_____2___________________2____
        // Mode 3  _33____33____33____33____33____33__________________3___
        // Mode 0  ___000___000___000___000___000___000________________000
        // Mode 1  ____________________________________11111111111111_____
        for i in 0..=143 {
            assert_eq!(controller.status.read_mode(), MODE_2_SEARCH_OAM_VALUE);
            assert_eq!(controller.coordinates.y, i);
            // This behavior seems weird.
            // assert_eq!(controller.should_scanline(), false);
            assert_eq!(controller.should_update_frame(), false);
            controller.update(MODE_2_SEARCH_OAM_CYCLES);
            assert_eq!(controller.status.read_mode(), MODE_3_TRANSFER_VALUE);
            assert_eq!(controller.coordinates.y, i);
            assert_eq!(controller.should_scanline(), false);
            assert_eq!(controller.should_update_frame(), false);
            controller.update(MODE_3_TRANSFER_CYCLES);
            assert_eq!(controller.status.read_mode(), MODE_0_HBLANK_VALUE);
            assert_eq!(controller.coordinates.y, i);
            assert_eq!(controller.should_scanline(), false);
            assert_eq!(controller.should_update_frame(), false);
            controller.update(MODE_0_HBLANK_CYCLES);
        }
        assert_eq!(controller.status.read_mode(), MODE_1_VBLANK_VALUE);
        assert_eq!(controller.coordinates.y, 144);
        assert_eq!(controller.should_scanline(), true);
        assert_eq!(controller.should_update_frame(), false);
        controller.update(MODE_1_VBLANK_CYCLES);
        for i in 145..=153 {
            assert_eq!(controller.status.read_mode(), MODE_1_VBLANK_VALUE);
            assert_eq!(controller.coordinates.y, i);
            assert_eq!(controller.should_scanline(), false);
            assert_eq!(controller.should_update_frame(), false);
            controller.update(MODE_1_VBLANK_CYCLES);
        }
        assert_eq!(controller.status.read_mode(), MODE_2_SEARCH_OAM_VALUE);
        assert_eq!(controller.coordinates.y, 0);
        assert_eq!(controller.should_scanline(), false);
        assert_eq!(controller.should_update_frame(), true);
    }

    #[test]
    fn vblank_interrupt_is_triggered() {
        let mut controller = VideoController::new();
        controller.control.write_lcd_enable(1);
        controller.update_mode(MODE_2_SEARCH_OAM);
        controller.init();

        for _ in 0..143 {
            assert_eq!(controller.update(MODE_2_SEARCH_OAM_CYCLES).len(), 0);
            assert_eq!(controller.update(MODE_3_TRANSFER_CYCLES).len(), 0);
            assert_eq!(controller.update(MODE_0_HBLANK_CYCLES).len(), 0);
        }

        assert_eq!(controller.update(MODE_2_SEARCH_OAM_CYCLES).len(), 0);
        assert_eq!(controller.update(MODE_3_TRANSFER_CYCLES).len(), 0);
        // interrupt is only triggered once.
        let interrupt = controller.update(MODE_0_HBLANK_CYCLES);
        assert_eq!(interrupt.len(), 1);
        assert_eq!(interrupt[0], Interrupt::VBlank);
        for _ in 144..=153 {
            assert_eq!(controller.update(MODE_1_VBLANK_CYCLES).len(), 0);
        }
    }

    #[test]
    fn mode_0_interrupt_is_triggered() {
        let mut controller = VideoController::new();
        controller.control.write_lcd_enable(1);
        controller.status.write_mode0_interrupt_source(1);
        controller.update_mode(MODE_2_SEARCH_OAM);
        controller.init();

        for i in 0..=143 {
            assert_eq!(controller.update(MODE_2_SEARCH_OAM_CYCLES).len(), 0);
            let interrupt = controller.update(MODE_3_TRANSFER_CYCLES);
            assert_eq!(interrupt.len(), 1);
            assert_eq!(interrupt[0], Interrupt::LCDStat);
            assert_eq!(
                controller.update(MODE_0_HBLANK_CYCLES).len(),
                /* vblank interrupt */ (i == 143) as usize
            );
        }

        for _ in 144..=153 {
            assert_eq!(controller.update(MODE_1_VBLANK_CYCLES).len(), 0);
        }
    }

    #[test]
    fn mode_1_interrupt_is_triggered() {
        let mut controller = VideoController::new();
        controller.control.write_lcd_enable(1);
        controller.status.write_mode1_interrupt_source(1);
        controller.update_mode(MODE_2_SEARCH_OAM);
        controller.init();

        for i in 0..=143 {
            assert_eq!(controller.update(MODE_2_SEARCH_OAM_CYCLES).len(), 0);
            assert_eq!(controller.update(MODE_3_TRANSFER_CYCLES).len(), 0);
            let interrupts = controller.update(MODE_0_HBLANK_CYCLES);
            assert_eq!(
                interrupts.len(),
                /* stat + vblank interrupt */ (i == 143) as usize * 2
            );
            assert_eq!(i == 143, interrupts.contains(&Interrupt::VBlank));
            assert_eq!(i == 143, interrupts.contains(&Interrupt::LCDStat));
        }

        for _ in 144..=153 {
            assert_eq!(controller.update(MODE_1_VBLANK_CYCLES).len(), 0);
        }
    }

    #[test]
    fn mode_2_interrupt_is_triggered() {
        let mut controller = VideoController::new();
        controller.control.write_lcd_enable(1);
        controller.status.write_mode2_interrupt_source(1);
        controller.update_mode(MODE_2_SEARCH_OAM);
        controller.init();

        for i in 0..=143 {
            assert_eq!(controller.update(MODE_2_SEARCH_OAM_CYCLES).len(), 0);
            assert_eq!(controller.update(MODE_3_TRANSFER_CYCLES).len(), 0);
            let interrupts = controller.update(MODE_0_HBLANK_CYCLES);
            assert_eq!(interrupts.len(), /* stat or vblank interrupt */ 1);
            assert_eq!(i == 143, interrupts.contains(&Interrupt::VBlank));
            assert_eq!(i != 143, interrupts.contains(&Interrupt::LCDStat));
        }

        for _ in 144..=152 {
            assert_eq!(controller.update(MODE_1_VBLANK_CYCLES).len(), 0);
        }
        // interrupt is only triggered once.
        let interrupt = controller.update(MODE_1_VBLANK_CYCLES);
        assert_eq!(interrupt.len(), 1);
        assert_eq!(interrupt[0], Interrupt::LCDStat);
    }

    #[test]
    fn lcy_lc_interrupt_is_triggered() {
        let mut controller = VideoController::new();
        controller.control.write_lcd_enable(1);
        controller.status.write_enable_lyc_stat_interrupt(1);
        controller.coordinates.compare_y = 125;
        controller.update_mode(MODE_2_SEARCH_OAM);
        controller.init();

        for i in 0..=143 {
            assert_eq!(controller.update(MODE_2_SEARCH_OAM_CYCLES).len(), 0);
            assert_eq!(controller.update(MODE_3_TRANSFER_CYCLES).len(), 0);
            let interrupts = controller.update(MODE_0_HBLANK_CYCLES);
            assert_eq!(
                interrupts.len(),
                /* stat or vblank interrupt */ ((i == 143) || (i == 124)) as usize
            );
            assert_eq!((i == 143), interrupts.contains(&Interrupt::VBlank));
            assert_eq!((i == 124), interrupts.contains(&Interrupt::LCDStat));
        }

        for _ in 144..=153 {
            assert_eq!(controller.update(MODE_1_VBLANK_CYCLES).len(), 0);
        }
    }

    #[test]
    fn nothing_should_happen_lc_is_disabled() {
        let mut controller = VideoController::new();
        controller.control.write_lcd_enable(0);

        assert_eq!(controller.coordinates.y, 0);
        controller.control.write_lcd_enable(0);
        assert_eq!(controller.coordinates.y, 0);
        controller.update(MODE_2_SEARCH_OAM_CYCLES);
        assert_eq!(controller.coordinates.y, 0);
        controller.update(MODE_3_TRANSFER_CYCLES);
        assert_eq!(controller.coordinates.y, 0);
        controller.update(MODE_0_HBLANK_CYCLES);
        assert_eq!(controller.coordinates.y, 0);
    }

    #[test]
    fn disabling_lcd_reset_coordinate_and_mode() {
        let mut controller = VideoController::new();
        controller.control.write_lcd_enable(1);
        controller.update_mode(MODE_2_SEARCH_OAM);
        controller.init();

        for _ in 0..=3 {
            controller.update(MODE_2_SEARCH_OAM_CYCLES);
            controller.update(MODE_3_TRANSFER_CYCLES);
            controller.update(MODE_0_HBLANK_CYCLES);
        }
        controller.update(MODE_2_SEARCH_OAM_CYCLES);
        assert_eq!(controller.status.read_mode(), MODE_3_TRANSFER_VALUE);
        assert_eq!(controller.coordinates.y, 4);

        let mut control = LcdControl::default();
        control.write_lcd_enable(0);
        controller.write_lcd(LCD_CONTROL_ADDRESS, control.value);

        controller.update(1);
        assert_eq!(controller.status.read_mode(), MODE_0_HBLANK_VALUE);
        assert_eq!(controller.coordinates.y, 0);
    }
}
