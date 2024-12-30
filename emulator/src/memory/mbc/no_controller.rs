use crate::memory::mbc::interface::{
    MemoryBankController, EXT_RAM_START_ADDRESS, RAM_BANK_SIZE, ROM_BANK_SIZE,
};
use std::fs::File;
use std::io::{BufReader, Read, Seek};
use std::time::Duration;

pub struct NoMemoryBankController {
    rom: Vec<u8>,
    ram: Vec<u8>,
}

impl NoMemoryBankController {
    pub fn new(
        rom_reader: &mut BufReader<File>,
        num_ram_banks: usize,
    ) -> Result<Box<dyn MemoryBankController>, String> {
        rom_reader
            .rewind()
            .map_err(|e| format!("Could not rewind the reader {:?}", e))?;
        let mut rom = vec![0; 2 * ROM_BANK_SIZE];
        rom_reader
            .read_exact(&mut rom)
            .map_err(|e| format!("Could not get the rom data {:?}", e))?;

        Ok(Box::new(Self {
            rom,
            ram: vec![0; num_ram_banks * RAM_BANK_SIZE],
        }))
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
