use crate::memory::mbc::common::{get_rom_ram_banks, RomRamBanks, EXT_RAM_START_ADDRESS, RAM_BANK_SIZE};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::BufReader;
use std::ops::Shl;
use std::time::Duration;
use crate::storage::helpers::{load_from_str, store_to_string};
use crate::storage::interface::{LoadError, SaveState, StorageVersion, StoreError};

#[derive(Debug, Serialize, Deserialize)]
pub struct MBC1BankController {
    rom: Vec<u8>,
    data: MBC1BankControllerSaveData,
}

    #[derive(Debug, Serialize, Deserialize)]
    pub struct MBC1BankControllerSaveData {
    ram: Vec<u8>,

    mask_rom_bank_number: u8,
    rom_bank_number: u8,
    ram_bank_number_or_rom_upper_bits: u8,

    is_ram_enabled: bool,
    is_advanced_banking_mode_enabled: bool,
}

impl MBC1BankController {
    pub fn create(
        rom_reader: &mut BufReader<File>,
        num_rom_banks: usize,
        num_ram_banks: usize,
    ) -> Result<Self, String> {
        let RomRamBanks { rom, ram } = get_rom_ram_banks(rom_reader, num_rom_banks, num_ram_banks)?;

        let max_rom_bank_number = num_rom_banks.saturating_sub(1) as u8;

        Ok(Self {
            rom,
            data: MBC1BankControllerSaveData {
                ram,
                // The num_rom_banks is always a power of 2.
                // This makes the number of rom banks - 1, the mask for the rom bank number.
                mask_rom_bank_number: max_rom_bank_number,
                rom_bank_number: 1,
                ram_bank_number_or_rom_upper_bits: 0,

                is_ram_enabled: false,
                is_advanced_banking_mode_enabled: false,
            }
        })
    }

    pub fn write_rom(&mut self, address: u16, value: u8) {
        match address {
            // https://gbdev.io/pandocs/MBC1.html#00001fff--ram-enable-write-only
            0x0000..=0x1FFF => {
                self.data.is_ram_enabled = value & 0x0F == 0x0A;
            }
            // https://gbdev.io/pandocs/MBC1.html#20003fff--rom-bank-number-write-only
            0x2000..=0x3FFF => {
                let mut raw_rom_bank_number = value & 0b1_1111;
                if raw_rom_bank_number == 0 {
                    raw_rom_bank_number = 1;
                }
                self.data.rom_bank_number = raw_rom_bank_number & self.data.mask_rom_bank_number;
            }
            // https://gbdev.io/pandocs/MBC1.html#40005fff--ram-bank-number--or--upper-bits-of-rom-bank-number-write-only
            0x4000..=0x5FFF => {
                self.data.ram_bank_number_or_rom_upper_bits = value & 0b11;
            }
            // https://gbdev.io/pandocs/MBC1.html#60007fff--banking-mode-select-write-only
            0x6000..=0x7FFF => {
                self.data.is_advanced_banking_mode_enabled = (value & 0b1) == 0b1;
            }
            _ => panic!("Invalid memory address 0x{:X}", address),
        }
    }

    pub fn read_rom(&self, address: u16) -> u8 {
        match address {
            0x0000..=0x3FFF => {
                // https://gbdev.io/pandocs/MBC1.html#00003fff
                let address = if self.data.is_advanced_banking_mode_enabled {
                    address as usize + (self.data.ram_bank_number_or_rom_upper_bits as usize).shl(19)
                } else {
                    address as usize
                };
                self.rom[address]
            }
            0x4000..=0x7FFF => {
                // https://gbdev.io/pandocs/MBC1.html#40007fff
                let address = (address as usize & 0x3FFF) + (self.data.rom_bank_number as usize).shl(14);
                let address = if self.data.is_advanced_banking_mode_enabled {
                    address + (self.data.ram_bank_number_or_rom_upper_bits as usize).shl(19)
                } else {
                    address
                };
                self.rom[address]
            }
            _ => 0,
        }
    }

    pub fn write_ext_ram(&mut self, address: u16, value: u8) {
        // Some blargg test roms write to the ext ram even if the ram information was configured
        // with 0 external ram banks.
        if !self.data.is_ram_enabled || self.data.ram.is_empty() {
            return;
        }
        self.data.ram[get_ext_ram_relative_address(
            address,
            self.data.ram_bank_number_or_rom_upper_bits,
            self.data.is_advanced_banking_mode_enabled,
        )] = value;
    }

    pub fn read_ext_ram(&self, address: u16) -> u8 {
        if !self.data.is_ram_enabled {
            return 0xFF;
        }
        self.data.ram[get_ext_ram_relative_address(
            address,
            self.data.ram_bank_number_or_rom_upper_bits,
            self.data.is_advanced_banking_mode_enabled,
        )]
    }

    pub fn update(&mut self, _duration: Duration) {
        // Nothing to do.
    }
}

fn get_ext_ram_relative_address(
    absolute_address: u16,
    ram_index: u8,
    is_advanced_banking_mode_enabled: bool,
) -> usize {
    if is_advanced_banking_mode_enabled {
        (absolute_address - EXT_RAM_START_ADDRESS) as usize + (ram_index as usize * RAM_BANK_SIZE)
    } else {
        (absolute_address - EXT_RAM_START_ADDRESS) as usize
    }
}

impl SaveState for MBC1BankController {
    fn save(&self) -> Result<String, StoreError> {
        store_to_string(&self.data)
    }

    fn load(&mut self, state: &str, _version: StorageVersion) -> Result<(), LoadError> {
        self.data = load_from_str(state)?;

        Ok(())
    }
}