use crate::memory::mbc::common::{get_rom_ram_banks, RomRamBanks, EXT_RAM_START_ADDRESS};
use std::fs::File;
use std::io::BufReader;
use std::time::Duration;
use serde::{Deserialize, Serialize};
use crate::storage::helpers::{load_from_str, store_to_string};
use crate::storage::interface::{LoadError, SaveState, StorageVersion, StoreError};

#[derive(Debug)]
pub struct NoMemoryBankController {
    rom: Vec<u8>,
    data: NoMemoryBankControllerData,
}

#[derive(Debug, Serialize, Deserialize)]
struct NoMemoryBankControllerData {
    ram: Vec<u8>,
}

impl NoMemoryBankController {
    pub fn create(
        rom_reader: &mut BufReader<File>,
        num_ram_banks: usize,
    ) -> Result<Self, String> {
        let RomRamBanks { rom, ram } = get_rom_ram_banks(rom_reader, 2, num_ram_banks)?;

        Ok(Self { rom, data: NoMemoryBankControllerData { ram } })
    }

    pub fn new(rom: Vec<u8>, ram: Vec<u8>) -> Self {
        Self { rom, data: NoMemoryBankControllerData { ram } }
    }

    pub fn write_rom(&mut self, _address: u16, _value: u8) {
        // Rom is read only.
    }

    pub fn read_rom(&self, address: u16) -> u8 {
        self.rom[address as usize]
    }

    pub fn write_ext_ram(&mut self, address: u16, value: u8) {
        self.data.ram[(address - EXT_RAM_START_ADDRESS) as usize] = value;
    }

    pub fn read_ext_ram(&self, address: u16) -> u8 {
        self.data.ram[(address - EXT_RAM_START_ADDRESS) as usize]
    }

    pub fn update(&mut self, _duration: Duration) {
        // Nothing to do.
    }
}

impl SaveState for NoMemoryBankController {
    fn save(&self) -> Result<String, StoreError> {
        store_to_string(&self.data)
    }

    fn load(&mut self, state: &str, _version: StorageVersion) -> Result<(), LoadError> {
        self.data = load_from_str(state)?;
        Ok(())
    }
}
