/// TODO: implement CGB behavior
/// Information from https://gbdev.io/pandocs/CGB_Registers.html#ff4d--key1-cgb-mode-only-prepare-speed-switch
pub const KEY_1_ADDRESS: u16 = 0xFF4D;
/// Information from: https://gbdev.io/pandocs/CGB_Registers.html#ff56--rp-cgb-mode-only-infrared-communications-port
pub const INFRARED_CONTROL_ADDRESS: u16 = 0xFF56;

pub enum CGBRegisters {
    Cgb(Cgb),
    Dmg,
}

impl CGBRegisters {
    pub fn new(is_cgb: bool) -> Self {
        if is_cgb {
            Self::Cgb(Cgb::default())
        } else {
            Self::Dmg
        }
    }

    pub fn read_key_1(&self) -> u8 {
        match self {
            Self::Cgb(cgb) => cgb.key_1,
            Self::Dmg => 0xFF,
        }
    }

    pub fn write_key_1(&mut self, value: u8) {
        match self {
            Self::Cgb(cgb) => cgb.key_1 = value,
            Self::Dmg => (),
        }
    }

    pub fn read_infrared_control(&self) -> u8 {
        match self {
            Self::Cgb(cgb) => cgb.infrared_control,
            Self::Dmg => 0xFF,
        }
    }

    pub fn write_infrared_control(&mut self, value: u8) {
        match self {
            Self::Cgb(cgb) => cgb.infrared_control = value,
            Self::Dmg => (),
        }
    }
}

#[derive(Default, Debug)]
pub struct Cgb {
    key_1: u8,
    infrared_control: u8,
}
