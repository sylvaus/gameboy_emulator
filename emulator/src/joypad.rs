use serde::{Deserialize, Serialize};
use macros::BitAccessor;
use crate::storage::helpers::{load_from_str, store_to_string};
use crate::storage::interface::{LoadError, SaveState, StorageVersion, StoreError};

/// Information from: https://gbdev.io/pandocs/Joypad_Input.html#ff00--p1joyp-joypad
pub const JOYPAD_INPUT_ADDRESS: u16 = 0xFF00;
pub const JOYPAD_INPUT_UNUSED_BIT_MASK: u8 = 0b1100_0000u8;
pub const JOYPAD_INPUT_READ_ONLY_MASK: u8 = 0b0000_1111u8;
pub const JOYPAD_INPUT_WRITE_ONLY_MASK: u8 = 0b0011_0000u8;

#[derive(Debug, Clone, Default)]
pub struct JoypadState {
    pub right: bool,
    pub left: bool,
    pub up: bool,
    pub down: bool,

    pub a: bool,
    pub b: bool,
    pub select: bool,
    pub start: bool,
}

impl JoypadState {
    pub fn to_u8(&self, action: bool, direction: bool) -> u8 {
        // Start with nothing pressed.
        let mut result = 0xF;
        if action {
            result &= (1 - self.a as u8)
                + ((1 - self.b as u8) << 1)
                + ((1 - self.select as u8) << 2)
                + ((1 - self.start as u8) << 3);
        }
        if direction {
            result &= (1 - self.right as u8)
                + ((1 - self.left as u8) << 1)
                + ((1 - self.up as u8) << 2)
                + ((1 - self.down as u8) << 3);
        }
        result
    }
}

#[derive(BitAccessor, Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct JoypadSaveData {
    #[bit_offset_size(unused, 6, 2)]
    #[bit_offset_size(action_buttons, 5, 1)]
    #[bit_offset_size(direction_buttons, 4, 1)]
    #[bit_offset_size(down_start, 3, 1)]
    #[bit_offset_size(up_select, 2, 1)]
    #[bit_offset_size(left_b, 1, 1)]
    #[bit_offset_size(right_a, 0, 1)]
    value: u8
}

#[derive(Debug, Copy, Clone, Default)]
pub struct JoypadInput {
    data: JoypadSaveData,
}
impl JoypadInput {
    pub fn new() -> Self {
        // Init with none of the button pressed: https://gbdev.io/pandocs/Joypad_Input.html#ff00--p1joyp-joypad
        Self { data: JoypadSaveData { value: 0xFF} }
    }

    pub fn write(&mut self, value: u8) {
        // Only set the control bits, the 4 lower bits are read only
        self.data.value &= JOYPAD_INPUT_READ_ONLY_MASK | JOYPAD_INPUT_UNUSED_BIT_MASK;
        self.data.value |= value & JOYPAD_INPUT_WRITE_ONLY_MASK;
    }

    pub fn read(self) -> u8 {
        self.data.value
    }

    /// Set the input value in the joypad memory and return true if the joystick interrupt is raised.
    ///
    /// Joypad interrupt is raised when one of the 4 lower bit of the joypad input switch from high to low.
    /// Doc: https://gbdev.io/pandocs/Interrupt_Sources.html?highlight=joypad%20inte#int-60--joypad-interrupt
    pub fn write_state(&mut self, state: &JoypadState) -> bool {
        let value = state.to_u8(
            self.data.read_action_buttons() == 0,
            self.data.read_direction_buttons() == 0,
        );
        let old_value = self.data.value & JOYPAD_INPUT_READ_ONLY_MASK;
        let modified_bits = value ^ old_value;
        let high_to_low_bits = old_value & modified_bits;

        self.data.value &= JOYPAD_INPUT_WRITE_ONLY_MASK | JOYPAD_INPUT_UNUSED_BIT_MASK;
        self.data.value |= value;
        high_to_low_bits > 0
    }
}

pub trait InputProvider {
    /// Updates the internal state
    fn update_inputs(&mut self);

    fn get_inputs(&self) -> JoypadState;
}

impl SaveState for JoypadInput {
    fn save(&self) -> Result<String, StoreError> {
        store_to_string(&self.data)
    }

    fn load(&mut self, state: &str, _version: StorageVersion) -> Result<(), LoadError> {
        self.data = load_from_str(state)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn joypad_input_new_should_start_with_no_buttons_pressed() {
        let joypad = JoypadInput::new();
        assert_eq!(joypad.data.read_action_buttons(), 1);
        assert_eq!(joypad.data.read_direction_buttons(), 1);
        assert_eq!(joypad.data.read_down_start(), 1);
        assert_eq!(joypad.data.read_up_select(), 1);
        assert_eq!(joypad.data.read_left_b(), 1);
        assert_eq!(joypad.data.read_right_a(), 1);
        assert_eq!(joypad.data.read_unused(), 0b11);
    }

    #[test]
    fn joypad_input_write_only_updates_action_direction_button_selectors() {
        let mut joypad = JoypadInput::new();

        joypad.write(0);

        assert_eq!(joypad.data.read_action_buttons(), 0);
        assert_eq!(joypad.data.read_direction_buttons(), 0);
        assert_eq!(joypad.data.read_down_start(), 1);
        assert_eq!(joypad.data.read_up_select(), 1);
        assert_eq!(joypad.data.read_left_b(), 1);
        assert_eq!(joypad.data.read_right_a(), 1);
        assert_eq!(joypad.data.read_unused(), 0b11);
    }

    #[test]
    fn joypad_input_write_state_no_selection() {
        let mut joypad = JoypadInput::new();

        joypad.write_state(&JoypadState {
            right: true,
            left: true,
            down: true,
            up: true,
            a: true,
            b: true,
            select: true,
            start: true,
        });

        assert_eq!(joypad.data.read_action_buttons(), 1);
        assert_eq!(joypad.data.read_direction_buttons(), 1);
        assert_eq!(joypad.data.read_down_start(), 1);
        assert_eq!(joypad.data.read_up_select(), 1);
        assert_eq!(joypad.data.read_left_b(), 1);
        assert_eq!(joypad.data.read_right_a(), 1);
        assert_eq!(joypad.data.read_unused(), 0b11);
    }

    #[test]
    fn joypad_input_write_state_action_selected() {
        let mut joypad = JoypadInput::new();
        joypad.data.write_action_buttons(0);

        joypad.write_state(&JoypadState {
            right: true,
            left: true,
            down: true,
            up: true,
            a: true,
            b: false,
            select: true,
            start: false,
        });

        assert_eq!(joypad.data.read_action_buttons(), 0);
        assert_eq!(joypad.data.read_direction_buttons(), 1);
        assert_eq!(joypad.data.read_down_start(), 1);
        assert_eq!(joypad.data.read_up_select(), 0);
        assert_eq!(joypad.data.read_left_b(), 1);
        assert_eq!(joypad.data.read_right_a(), 0);
        assert_eq!(joypad.data.read_unused(), 0b11);
    }

    #[test]
    fn joypad_input_write_state_direction_selected() {
        let mut joypad = JoypadInput::new();
        joypad.data.write_direction_buttons(0);

        joypad.write_state(&JoypadState {
            right: true,
            left: false,
            up: true,
            down: false,
            a: true,
            b: true,
            select: true,
            start: true,
        });

        assert_eq!(joypad.data.read_action_buttons(), 1);
        assert_eq!(joypad.data.read_direction_buttons(), 0);
        assert_eq!(joypad.data.read_down_start(), 1);
        assert_eq!(joypad.data.read_up_select(), 0);
        assert_eq!(joypad.data.read_left_b(), 1);
        assert_eq!(joypad.data.read_right_a(), 0);
        assert_eq!(joypad.data.read_unused(), 0b11);
    }
}
