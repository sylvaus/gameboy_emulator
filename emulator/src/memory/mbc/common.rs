use std::fs::File;
use std::io::{BufReader, Read, Seek};

pub const ROM_START_ADDRESS: u16 = 0x0000;
pub const ROM_BANK_0_END_ADDRESS: u16 = 0x3FFF;
pub const ROM_BANK_1_N_START_ADDRESS: u16 = 0x4000;
pub const ROM_BANK_1_N_END_ADDRESS: u16 = 0x7FFF;
pub const EXT_RAM_START_ADDRESS: u16 = 0xA000;
pub const EXT_RAM_END_ADDRESS: u16 = 0xBFFF;
pub const ROM_BANK_SIZE: usize = 0x4000;
pub const RAM_BANK_SIZE: usize = 0x2000;

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
