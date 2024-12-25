use crate::memory::mbc::interface::{
    MemoryBankController, EXT_RAM_START_ADDRESS, RAM_BANK_SIZE, ROM_BANK_0_END_ADDRESS,
    ROM_BANK_1_N_END_ADDRESS, ROM_BANK_1_N_START_ADDRESS, ROM_BANK_SIZE,
};
use chrono::{DateTime, Utc};
use std::cmp::max;
use std::fs::File;
use std::io::{BufReader, Read, Seek};

pub struct MBC3BankController {
    rom: Vec<u8>,
    ram: Vec<u8>,

    rom_index: usize,
    ram_rtc_index: usize,
    max_ram_index: usize,
    ram_rtc_enabled: bool,
}

impl MBC3BankController {
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

        Ok(Box::new(Self {
            rom,
            ram: vec![0; num_ram_banks * RAM_BANK_SIZE],
            rom_index: 0,
            ram_rtc_index: 0,
            max_ram_index: num_ram_banks.saturating_sub(1),
            ram_rtc_enabled: false,
        }))
    }
}

impl MemoryBankController for MBC3BankController {
    fn write_rom(&mut self, _address: u16, _value: u8) {
        // Rom is read only.
    }

    fn read_rom(&self, address: u16) -> u8 {
        match address {
            0..=ROM_BANK_0_END_ADDRESS => self.rom[address as usize],
            ROM_BANK_1_N_START_ADDRESS..=ROM_BANK_1_N_END_ADDRESS => {
                self.rom[address as usize + max(self.rom_index, 1) * ROM_BANK_SIZE]
            }
            _ => panic!("Rom address {} is invalid ", address),
        }
    }

    fn write_ext_ram(&mut self, address: u16, value: u8) {
        if !self.ram_rtc_enabled {
            return;
        }
        self.ram[(address - EXT_RAM_START_ADDRESS) as usize] = value;
    }

    fn read_ext_ram(&self, address: u16) -> u8 {
        if !self.ram_rtc_enabled {
            return 0;
        }
        match self.ram_rtc_index {
            0..=3 if self.ram_rtc_index <= self.max_ram_index => {
                self.ram[(address - EXT_RAM_START_ADDRESS) as usize
                    + self.ram_rtc_index * RAM_BANK_SIZE]
            }
            // 8..=0xC => {
            //
            // }
            _ => panic!("Trying to read invalid address {} is invalid ", address),
        }
    }
}

struct RealTimeCounter {
    // When the rtc was started
    start: DateTime<Utc>,
    // When the rtc was stopped
    stop: DateTime<Utc>,
    running: bool,
    // Overflow was set
    overflow: bool,
}

impl RealTimeCounter {
    pub fn new() -> Self {
        Self {
            start: Utc::now(),
            stop: Utc::now(),
            running: false,
            overflow: false,
        }
    }

    // pub fn write_second(&mut self, value: u8) {
    //     let current = self.get_duration();
    //     current.
    // }
    //
    // pub fn update_time(&mut self, value :)
    //
    // fn get_duration(&self) -> Duration {
    //     if !self.running {
    //         self.stop - self.start
    //     } else {
    //         Instant::now().saturating_duration_since(self.start)
    //     }
    // }
}
