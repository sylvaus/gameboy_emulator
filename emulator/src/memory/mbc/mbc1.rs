use std::fs::File;
use std::io::{BufReader, Read, Seek};
use crate::memory::mbc::interface::{MemoryBankController, EXT_RAM_START_ADDRESS, RAM_BANK_SIZE, ROM_BANK_SIZE};
use std::ops::Shl;
use std::time::Duration;

pub struct MBC1BankController {
    rom: Vec<u8>,
    ram: Vec<u8>,

    mask_rom_bank_number: u8,
    rom_bank_number: u8,
    ram_bank_number_or_rom_upper_bits: u8,

    is_ram_enabled: bool,
    is_advanced_banking_mode_enabled: bool,
}

impl MBC1BankController {
    pub fn new(
        rom_reader: &mut BufReader<File>,
        num_rom_banks: usize,
        num_ram_banks: usize,
    ) -> Result<Box<dyn MemoryBankController>, String> {
        rom_reader
            .rewind()
            .map_err(|e| format!("Could not rewind the reader {:?}", e))?;
        let mut rom = vec![0; num_rom_banks * ROM_BANK_SIZE];
        rom_reader
            .read_exact(&mut rom)
            .map_err(|e| format!("Could not get the rom data {:?}", e))?;

        let max_rom_bank_number = num_rom_banks.saturating_sub(1) as u8;

        Ok(Box::new(Self {
            rom,
            ram: vec![0; num_ram_banks * RAM_BANK_SIZE],
            // The num_rom_banks is always a power of 2.
            // This make the number of rom banks - 1, the mask for the rom bank number.
            mask_rom_bank_number: max_rom_bank_number,
            rom_bank_number: 1,
            ram_bank_number_or_rom_upper_bits: 0,

            is_ram_enabled: false,
            is_advanced_banking_mode_enabled: false,
        }))
    }
}

impl MemoryBankController for MBC1BankController {
    fn write_rom(&mut self, address: u16, value: u8) {
        match address {
            // https://gbdev.io/pandocs/MBC1.html#00001fff--ram-enable-write-only
            0x0000..=0x1FFF => {
                self.is_ram_enabled = value & 0x0F == 0x0A;
            }
            // https://gbdev.io/pandocs/MBC1.html#20003fff--rom-bank-number-write-only
            0x2000..=0x3FFF => {
                let mut raw_rom_bank_number = value & 0b1_1111;
                if raw_rom_bank_number == 0 {
                    raw_rom_bank_number = 1;
                }
                self.rom_bank_number = raw_rom_bank_number & self.mask_rom_bank_number;
            }
            // https://gbdev.io/pandocs/MBC1.html#40005fff--ram-bank-number--or--upper-bits-of-rom-bank-number-write-only
            0x4000..=0x5FFF => {
                self.ram_bank_number_or_rom_upper_bits = value & 0b11;
            }
            // https://gbdev.io/pandocs/MBC1.html#60007fff--banking-mode-select-write-only
            0x6000..=0x7FFF => {
                self.is_advanced_banking_mode_enabled = (value & 0b1) == 0b1;
            }
            _ => panic!("Invalid memory address 0x{:X}", address),
        }
    }

    fn read_rom(&self, address: u16) -> u8 {
        match address {
            0x0000..=0x3FFF => {
                // https://gbdev.io/pandocs/MBC1.html#00003fff
                let address = if self.is_advanced_banking_mode_enabled {
                    address + (self.ram_bank_number_or_rom_upper_bits as u16).shl(19)
                } else {
                    address
                };
                self.rom[address as usize]
            }
            0x4000..=0x7FFF => {
                let address = address + (self.rom_bank_number as u16).shl(14);
                let address = if self.is_advanced_banking_mode_enabled {
                    address + (self.ram_bank_number_or_rom_upper_bits as u16).shl(19)
                } else {
                    address
                };
                self.rom[address as usize]
            }
            _ => 0,
        }
    }

    fn write_ext_ram(&mut self, address: u16, value: u8) {
        if !self.is_ram_enabled {
            return;
        }
        self.ram[get_ext_ram_relative_address(
            address,
            self.ram_bank_number_or_rom_upper_bits,
            self.is_advanced_banking_mode_enabled,
        )] = value;
    }

    fn read_ext_ram(&self, address: u16) -> u8 {
        if !self.is_ram_enabled {
            return 0xFF;
        }
        self.ram[get_ext_ram_relative_address(
            address,
            self.ram_bank_number_or_rom_upper_bits,
            self.is_advanced_banking_mode_enabled,
        )]
    }

    fn update(&mut self, _duration: Duration) {
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
