use crate::memory::mbc::interface::MemoryBankController;
use std::fs::File;
use std::io::{BufReader, Read, Seek};

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
        let mut rom = vec![0; 0x8000];
        rom_reader
            .read_exact(&mut rom)
            .map_err(|e| format!("Could not get the rom data {:?}", e))?;

        Ok(Box::new(Self {
            rom,
            ram: vec![0; num_ram_banks * 0x8000],
        }))
    }
}

impl MemoryBankController for NoMemoryBankController {
    fn write_rom(&mut self, address: u16, value: u8) {
        todo!()
    }

    fn read_rom(&self, address: u16) {
        todo!()
    }

    fn write_ext_ram(&mut self, address: u16, value: u8) {
        todo!()
    }

    fn read_ext_ram(&self, address: u16) {
        todo!()
    }
}
