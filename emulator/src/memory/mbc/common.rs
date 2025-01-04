use crate::memory::mbc::interface::{RAM_BANK_SIZE, ROM_BANK_SIZE};
use std::fs::File;
use std::io::{BufReader, Read, Seek};

pub struct RomRamBanks {
    pub rom: Vec<u8>,
    pub ram: Vec<u8>,
}

pub fn get_rom_ram_banks(
    rom_reader: &mut BufReader<File>,
    num_rom_banks: usize,
    num_ram_banks: usize,
) -> Result<RomRamBanks, String> {
    rom_reader
        .rewind()
        .map_err(|e| format!("Could not rewind the reader {:?}", e))?;
    let mut rom = vec![0; num_rom_banks * ROM_BANK_SIZE];
    rom_reader
        .read_exact(&mut rom)
        .map_err(|e| format!("Could not get the rom data {:?}", e))?;

    Ok(RomRamBanks {
        rom,
        ram: vec![0; num_ram_banks * RAM_BANK_SIZE],
    })
}
