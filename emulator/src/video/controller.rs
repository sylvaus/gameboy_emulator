use crate::video::memory::LcdInfo;

struct VideoController {
    vram: Vec<u8>,
    oam: Vec<u8>,
    lcd_info: LcdInfo,
}