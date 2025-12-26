use serde::{Deserialize, Serialize};
use crate::storage::helpers::{load_from_str, store_to_string};
use crate::storage::interface::{LoadError, SaveState, StorageVersion, StoreError};

/// TODO: implement CGB behavior
/// Information from https://gbdev.io/pandocs/CGB_Registers.html#ff4d--key1-cgb-mode-only-prepare-speed-switch
pub const KEY_1_ADDRESS: u16 = 0xFF4D;
/// Information from: https://gbdev.io/pandocs/CGB_Registers.html#ff56--rp-cgb-mode-only-infrared-communications-port
pub const INFRARED_CONTROL_ADDRESS: u16 = 0xFF56;

#[derive(Debug, Clone)]
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
            Self::Cgb(cgb) => cgb.data.key_1,
            Self::Dmg => 0xFF,
        }
    }

    pub fn write_key_1(&mut self, value: u8) {
        match self {
            Self::Cgb(cgb) => cgb.data.key_1 = value,
            Self::Dmg => (),
        }
    }

    pub fn read_infrared_control(&self) -> u8 {
        match self {
            Self::Cgb(cgb) => cgb.data.infrared_control,
            Self::Dmg => 0xFF,
        }
    }

    pub fn write_infrared_control(&mut self, value: u8) {
        match self {
            Self::Cgb(cgb) => cgb.data.infrared_control = value,
            Self::Dmg => (),
        }
    }
}

#[derive(Default, Debug, Clone)]
pub struct Cgb {
    data: CgbSaveData,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct CgbSaveData {
    key_1: u8,
    infrared_control: u8
}

impl SaveState for CGBRegisters {
    fn save(&self) -> Result<String, StoreError> {
        match self {
            CGBRegisters::Cgb(cgb) => cgb.save(),
            CGBRegisters::Dmg => Ok("".to_string()),
        }
    }

    fn load(&mut self, state: &str, version: StorageVersion) -> Result<(), LoadError> {
        match self {
            CGBRegisters::Cgb(cgb) => cgb.load(state, version),
            CGBRegisters::Dmg => Ok(())
        }
    }
}

impl SaveState for Cgb {
    fn save(&self) -> Result<String, StoreError> {
        store_to_string(&self.data)
    }

    fn load(&mut self, state: &str, _version: StorageVersion) -> Result<(), LoadError> {
        self.data = load_from_str(state)?;

        Ok(())
    }
}
