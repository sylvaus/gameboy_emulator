use crate::memory::mbc::common::{get_rom_ram_banks, RomRamBanks};
use crate::memory::mbc::interface::{MemoryBankController, EXT_RAM_START_ADDRESS, RAM_BANK_SIZE};
use std::fs::File;
use std::io::BufReader;
use std::ops::Shl;
use std::time::Duration;

pub struct MBC5BankController {
    rom: Vec<u8>,
    ram: Vec<u8>,

    rom_bank_number: u8,
    rom_upper_bits: u8,

    ram_bank_number: u8,
    is_ram_enabled: bool,
}

impl MBC5BankController {
    pub fn create(
        rom_reader: &mut BufReader<File>,
        num_rom_banks: usize,
        num_ram_banks: usize,
    ) -> Result<Box<dyn MemoryBankController>, String> {
        let RomRamBanks { rom, ram } = get_rom_ram_banks(rom_reader, num_rom_banks, num_ram_banks)?;

        Ok(Box::new(Self {
            rom,
            ram,
            rom_bank_number: 1,
            rom_upper_bits: 0,
            ram_bank_number: 0,
            is_ram_enabled: false,
        }))
    }

    fn get_ext_ram_relative_address(&self, address: u16) -> usize {
        (address - EXT_RAM_START_ADDRESS) as usize + (self.ram_bank_number as usize * RAM_BANK_SIZE)
    }
}

impl MemoryBankController for MBC5BankController {
    fn write_rom(&mut self, address: u16, value: u8) {
        match address {
            // https://gbdev.io/pandocs/MBC5.html#0000-1fff---ram-enable-write-only
            0x0000..=0x1FFF => {
                self.is_ram_enabled = value & 0x0F == 0x0A;
            }
            // https://gbdev.io/pandocs/MBC5.html#2000-2fff---8-least-significant-bits-of-rom-bank-number-write-only
            0x2000..=0x2FFF => {
                self.rom_bank_number = value;
            }
            // https://gbdev.io/pandocs/MBC5.html#3000-3fff---9th-bit-of-rom-bank-number-write-only
            0x3000..=0x3FFF => {
                self.rom_upper_bits = value & 0b1;
            }
            // https://gbdev.io/pandocs/MBC1.html#40005fff--ram-bank-number--or--upper-bits-of-rom-bank-number-write-only
            0x4000..=0x5FFF => {
                self.ram_bank_number = value;
            }
            0x6000..=0x7FFF => { /* Nothing to do */ }
            _ => panic!("Invalid memory address 0x{:X}", address),
        }
    }

    fn read_rom(&self, address: u16) -> u8 {
        match address {
            0x0000..=0x3FFF => {
                // https://gbdev.io/pandocs/MBC5.html#0000-3fff---rom-bank-00-read-only
                self.rom[address as usize]
            }
            0x4000..=0x7FFF => {
                // https://gbdev.io/pandocs/MBC1.html#40007fff
                let rom_bank_number: usize =
                    self.rom_bank_number as usize + (self.rom_upper_bits as usize).shl(8);
                let address = (address as usize & 0x3FFF) + rom_bank_number.shl(14);
                self.rom[address]
            }
            _ => 0,
        }
    }

    fn write_ext_ram(&mut self, address: u16, value: u8) {
        // Some blargg test roms write to the ext ram even if the ram information was configured
        // with 0 external ram banks.
        if !self.is_ram_enabled || self.ram.is_empty() {
            return;
        }
        let relative_address = self.get_ext_ram_relative_address(address);
        self.ram[relative_address] = value;
    }

    fn read_ext_ram(&self, address: u16) -> u8 {
        if !self.is_ram_enabled {
            return 0xFF;
        }
        self.ram[self.get_ext_ram_relative_address(address)]
    }

    fn update(&mut self, _duration: Duration) {
        // Nothing to do.
    }
}
