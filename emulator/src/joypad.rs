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
    value: u8,
}
impl JoypadInput {
    pub fn write(&mut self, value: u8) {
        // Only set the control bits, the 4 lower bits are read only
        self.value &= 0b00001111u8;
        self.value |= (value & 0b11110000u8);
    }

    pub fn read(self) -> u8 {
        self.value
    }
}

impl JoypadInput {
    pub fn new() -> Self {
        // Init with none of the button pressed: https://gbdev.io/pandocs/Joypad_Input.html#ff00--p1joyp-joypad
        Self { value: 0xFF }
    }
}

pub trait InputProvider {
    /// Updates the internal state
    fn update_inputs(&mut self);

    /// Set the input value in the joypad memory and return true if the joystick interrupt is raised.
    ///
    /// Joypad interrupt is raised when one of the 4 lower bit of the joypad input switch from high to low.
    /// Doc: https://gbdev.io/pandocs/Interrupt_Sources.html?highlight=joypad%20inte#int-60--joypad-interrupt
    fn set_inputs(&self, input: &mut JoypadInput) -> bool;

    fn is_quit_pressed(&self) -> bool;
}
