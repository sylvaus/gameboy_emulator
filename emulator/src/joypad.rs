use macros::BitAccessor;

/// Information from: https://gbdev.io/pandocs/Joypad_Input.html#ff00--p1joyp-joypad
const JOYPAD_ADDRESS: u16 = 0xFF00;

#[derive(BitAccessor, Debug, Copy, Clone, Default)]
pub struct Joypad {
    #[bit_offset_size(action_buttons, 5, 1)]
    #[bit_offset_size(direction_buttons, 4, 1)]
    #[bit_offset_size(mode0_interrupt_source, 3, 1)]
    #[bit_offset_size(lyc_ly_flag, 2, 1)]
    #[bit_offset_size(mode, 1, 1)]
    #[bit_offset_size(mode, 0, 1)]
    pub value: u8,
}

impl Joypad {
    pub fn new() -> Self {
        Self { value: 0 }
    }

    pub fn read(&self) -> u8 {
        self.value
    }

    pub fn write(&mut self, value: u8) {
        self.value = value
    }
}
