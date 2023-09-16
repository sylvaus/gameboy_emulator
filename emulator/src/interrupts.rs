/// Interrupts are listed here: https://gbdev.io/pandocs/Interrupts.html#ffff--ie-interrupt-enable

const VBLANK_ENABLE_FLAG: u8 = 1u8 << 0;
const LCD_STATS_ENABLE_FLAG: u8 = 1u8 << 1;
const TIMER_ENABLE_FLAG: u8 = 1u8 << 2;
const SERIAL_ENABLE_FLAG: u8 = 1u8 << 3;
const JOYPAD_ENABLE_FLAG: u8 = 1u8 << 4;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Interrupt {
    VBlank,
    LCDStat,
    Timer,
    Serial,
    Joypad,
}

impl Interrupt {
    pub fn get_address(&self) -> u16 {
        match self {
            Interrupt::VBlank => 0x40,
            Interrupt::LCDStat => 0x48,
            Interrupt::Timer => 0x50,
            Interrupt::Serial => 0x58,
            Interrupt::Joypad => 0x60,
        }
    }

    pub fn is_enabled(&self, interrupt_enable: u8) -> bool {
        match self {
            Interrupt::VBlank => (VBLANK_ENABLE_FLAG & interrupt_enable) > 0,
            Interrupt::LCDStat => (LCD_STATS_ENABLE_FLAG & interrupt_enable) > 0,
            Interrupt::Timer => (TIMER_ENABLE_FLAG & interrupt_enable) > 0,
            Interrupt::Serial => (SERIAL_ENABLE_FLAG & interrupt_enable) > 0,
            Interrupt::Joypad => (JOYPAD_ENABLE_FLAG & interrupt_enable) > 0,
        }
    }

    pub fn enable(&self, interrupt_enable: u8) -> u8 {
        match self {
            Interrupt::VBlank => VBLANK_ENABLE_FLAG | interrupt_enable,
            Interrupt::LCDStat => LCD_STATS_ENABLE_FLAG | interrupt_enable,
            Interrupt::Timer => TIMER_ENABLE_FLAG | interrupt_enable,
            Interrupt::Serial => SERIAL_ENABLE_FLAG | interrupt_enable,
            Interrupt::Joypad => JOYPAD_ENABLE_FLAG | interrupt_enable,
        }
    }
}
