/// TODO: implement CGB behavior
/// Information from https://gbdev.io/pandocs/CGB_Registers.html#ff4d--key1-cgb-mode-only-prepare-speed-switch
pub const KEY_1_ADDRESS: u16 = 0xFF4D;
/// Information from: https://gbdev.io/pandocs/CGB_Registers.html#ff56--rp-cgb-mode-only-infrared-communications-port
pub const INFRARED_CONTROL_ADDRESS: u16 = 0xFF56;

#[derive(Default, Debug)]
pub struct CGBRegisters {
    key_1: u8,
    infrared_control: u8,
}

impl CGBRegisters {
    pub fn read_key_1(&self) -> u8 {
        self.key_1
    }

    pub fn write_key_1(&mut self, value: u8) {
        self.key_1 = value;
    }

    pub fn read_infrared_control(&self) -> u8 {
        self.infrared_control
    }

    pub fn write_infrared_control(&mut self, value: u8) {
        self.infrared_control = value;
    }
}
