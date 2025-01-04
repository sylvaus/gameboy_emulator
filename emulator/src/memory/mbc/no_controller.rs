use crate::memory::mbc::common::{get_rom_ram_banks, RomRamBanks};
use crate::memory::mbc::interface::{MemoryBankController, EXT_RAM_START_ADDRESS};
use std::fs::File;
use std::io::BufReader;
use std::time::Duration;

pub struct NoMemoryBankController {
    rom: Vec<u8>,
    ram: Vec<u8>,
}

impl NoMemoryBankController {
    pub fn create(
        rom_reader: &mut BufReader<File>,
        num_ram_banks: usize,
    ) -> Result<Box<dyn MemoryBankController>, String> {
        let RomRamBanks { rom, ram } = get_rom_ram_banks(rom_reader, 2, num_ram_banks)?;

        Ok(Box::new(Self { rom, ram }))
    }
}

impl MemoryBankController for NoMemoryBankController {
    fn write_rom(&mut self, _address: u16, _value: u8) {
        // Rom is read only.
    }

    fn read_rom(&self, address: u16) -> u8 {
        self.rom[address as usize]
    }

    fn write_ext_ram(&mut self, address: u16, value: u8) {
        self.ram[(address - EXT_RAM_START_ADDRESS) as usize] = value;
    }

    fn read_ext_ram(&self, address: u16) -> u8 {
        self.ram[(address - EXT_RAM_START_ADDRESS) as usize]
    }

    fn update(&mut self, _duration: Duration) {
        // Nothing to do.
    }
}
