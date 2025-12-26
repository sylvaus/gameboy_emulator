//! TODO: Implement the CGB variant
//! Information from: https://gbdev.io/pandocs/Memory_Map.html#memory-map

use serde::{Deserialize, Serialize};
use crate::storage::helpers::{load_from_str, store_to_string};
use crate::storage::interface::{LoadError, SaveState, StorageVersion, StoreError};

pub const WORK_RAM_SIZE: usize = 0x1000;
pub const WORK_RAM_START_ADDRESS: u16 = 0xC000;
pub const WORK_RAM_END_ADDRESS: u16 = 0xDFFF;
pub const ECHO_RAM_START_ADDRESS: u16 = 0xE000;
pub const ECHO_RAM_END_ADDRESS: u16 = 0xFDFF;

pub const HIGH_RAM_SIZE: usize = 0x7F;
pub const HIGH_RAM_START_ADDRESS: u16 = 0xFF80;
pub const HIGH_RAM_END_ADDRESS: u16 = 0xFFFE;
/// Information from: https://gbdev.io/pandocs/CGB_Registers.html#ff70--svbk-cgb-mode-only-wram-bank
pub const SELECT_WORK_RAM_BANK_ADDRESS: u16 = 0xFF70;

pub struct RamController {
    data: RamControllerSaveData,
}

#[derive(Serialize, Deserialize)]
pub struct RamControllerSaveData {
    work_ram: Vec<u8>,
    high_ram: Vec<u8>
}

impl RamController {
    pub fn new() -> Self {
        Self {
            data: RamControllerSaveData {
            work_ram: vec![0u8; WORK_RAM_SIZE * 2usize],
            high_ram: vec![0u8; HIGH_RAM_SIZE],
        }
    }
    }

    pub fn write_work_ram(&mut self, address: u16, value: u8) {
        self.data.work_ram[(address - WORK_RAM_START_ADDRESS) as usize] = value;
    }

    pub fn read_work_ram(&self, address: u16) -> u8 {
        self.data.work_ram[(address - WORK_RAM_START_ADDRESS) as usize]
    }

    pub fn write_echo_ram(&mut self, address: u16, value: u8) {
        // Just echo work ram
        self.write_work_ram(address - 0x2000, value)
    }

    pub fn read_echo_ram(&self, address: u16) -> u8 {
        self.read_work_ram(address - 0x2000)
    }

    pub fn write_high_ram(&mut self, address: u16, value: u8) {
        self.data.high_ram[(address - HIGH_RAM_START_ADDRESS) as usize] = value;
    }

    pub fn read_high_ram(&self, address: u16) -> u8 {
        self.data.high_ram[(address - HIGH_RAM_START_ADDRESS) as usize]
    }

    pub fn write_selected_work_ram_bank(&mut self, _value: u8) {
        // see: https://gbdev.io/pandocs/CGB_Registers.html#ff70--svbk-cgb-mode-only-wram-bank
        // TODO: Implement CGB variant where this should select the ram bank;
    }

    pub fn read_selected_work_ram_bank(&self) -> u8 {
        // see: https://gbdev.io/pandocs/CGB_Registers.html#ff70--svbk-cgb-mode-only-wram-bank
        // TODO: Implement CGB variant where this should return the selected ram bank ;
        0xFF
    }
}

impl Default for RamController {
    fn default() -> Self {
        Self::new()
    }
}

impl SaveState for RamController {
    fn save(&self) -> Result<String, StoreError> {
        store_to_string(&self.data)
    }

    fn load(&mut self, state: &str, _version: StorageVersion) -> Result<(), LoadError> {
        self.data = load_from_str(state)?;

        Ok(())
    }
}