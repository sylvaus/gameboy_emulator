use macros::BitAccessor;

/// Information from: https://gbdev.io/pandocs/Joypad_Input.html#ff00--p1joyp-joypad
pub const JOYPAD_INPUT_ADDRESS: u16 = 0xFF00;

#[derive(BitAccessor, Debug, Copy, Clone, Default)]
pub struct JoypadInput {
    #[bit_offset_size(action_buttons, 5, 1)]
    #[bit_offset_size(direction_buttons, 4, 1)]
    #[bit_offset_size(down_start, 3, 1)]
    #[bit_offset_size(up_select, 2, 1)]
    #[bit_offset_size(left_b, 1, 1)]
    #[bit_offset_size(right_a, 0, 1)]
    pub value: u8,
}

impl JoypadInput {
    pub fn new() -> Self {
        Self { value: 0 }
    }
}
