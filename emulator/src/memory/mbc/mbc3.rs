use crate::memory::mbc::interface::{
    MemoryBankController, EXT_RAM_START_ADDRESS, RAM_BANK_SIZE, ROM_BANK_0_END_ADDRESS,
    ROM_BANK_1_N_END_ADDRESS, ROM_BANK_1_N_START_ADDRESS, ROM_BANK_SIZE,
};
use macros::BitAccessor;
use std::cmp::max;
use std::fs::File;
use std::io::{BufReader, Read, Seek};
use std::time::Duration;

pub struct MBC3BankController {
    rom: Vec<u8>,
    ram: Vec<u8>,
    rtc: RealTimeCounter,

    rom_index: u8,
    max_rom_index: u8,
    ram_rtc_enabled: bool,
    ram_rtc_index: u8,
    max_ram_index: u8,
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
            rtc: RealTimeCounter::new(),
            rom_index: 0,
            max_rom_index: num_rom_banks.saturating_sub(1) as u8,
            ram_rtc_index: 0,
            max_ram_index: num_ram_banks.saturating_sub(1) as u8,
            ram_rtc_enabled: false,
        }))
    }
}

impl MemoryBankController for MBC3BankController {
    fn write_rom(&mut self, address: u16, value: u8) {
        match address {
            // https://gbdev.io/pandocs/MBC3.html#0000-1fff---ram-and-timer-enable-write-only
            0x0000..=0x1FFF => {
                if value == 0x0A {
                    self.ram_rtc_enabled = true;
                }
                if value == 0x00 {
                    self.ram_rtc_enabled = false;
                }
            }
            // https://gbdev.io/pandocs/MBC3.html#2000-3fff---rom-bank-number-write-only
            0x2000..=0x3FFF => {
                if value <= self.max_rom_index {
                    self.rom_index = value;
                } else {
                    panic!("Unhandled rom index {}", value);
                }
            }
            // https://gbdev.io/pandocs/MBC3.html#4000-5fff---ram-bank-number---or---rtc-register-select-write-only
            0x4000..=0x5FFF => {
                self.ram_rtc_index = value;
            }
            // https://gbdev.io/pandocs/MBC3.html#6000-7fff---latch-clock-data-write-only
            0x6000..=0x7FFF => {
                self.rtc.latch(value);
            }
            address => panic!("Unhandled address {:X}", address),
        }
    }

    fn read_rom(&self, address: u16) -> u8 {
        match address {
            // https://gbdev.io/pandocs/MBC3.html#0000-3fff---rom-bank-00-read-only
            0..=ROM_BANK_0_END_ADDRESS => self.rom[address as usize],
            // https://gbdev.io/pandocs/MBC3.html#4000-7fff---rom-bank-01-7f-read-only
            ROM_BANK_1_N_START_ADDRESS..=ROM_BANK_1_N_END_ADDRESS => {
                let index = max(self.rom_index, 1) as usize;
                let relative_address = (address - ROM_BANK_1_N_START_ADDRESS) as usize;
                let final_address = relative_address + index * ROM_BANK_SIZE;
                self.rom[final_address]
            }
            _ => panic!("Rom address {:X} is invalid ", address),
        }
    }

    fn write_ext_ram(&mut self, address: u16, value: u8) {
        if !self.ram_rtc_enabled {
            return;
        }
        match self.ram_rtc_index {
            0..=3 if self.ram_rtc_index <= self.max_ram_index => {
                self.ram[get_ext_ram_relative_address(address, self.max_ram_index)] = value
            }
            8..=0xC => {
                self.rtc.write(self.ram_rtc_index, value);
            }
            _ => panic!("Trying to read invalid address {} is invalid ", address),
        }
    }

    /// Read external RAM or RTC
    /// Doc: https://gbdev.io/pandocs/MBC3.html#a000-bfff---rtc-register-08-0c-readwrite
    fn read_ext_ram(&self, address: u16) -> u8 {
        if !self.ram_rtc_enabled {
            return 0;
        }
        match self.ram_rtc_index {
            0..=3 if self.ram_rtc_index <= self.max_ram_index => {
                self.ram[get_ext_ram_relative_address(address, self.max_ram_index)]
            }
            8..=0xC => {
                self.rtc.read(self.ram_rtc_index)
            }
            _ => panic!("Trying to read invalid address {} is invalid ", address),
        }
    }

    fn update(&mut self, duration: Duration) -> () {
        self.rtc.update(duration);
    }
}

fn get_ext_ram_relative_address(absolute_address: u16, ram_index: u8) -> usize {
    (absolute_address - EXT_RAM_START_ADDRESS) as usize
        + (ram_index as usize * RAM_BANK_SIZE)
}

#[derive(BitAccessor, Debug, Copy, Clone, Default)]
struct DayHighFlag {
    #[bit_offset_size(day_carry, 7, 1)]
    #[bit_offset_size(halt, 6, 1)]
    #[bit_offset_size(day_high, 0, 1)]
    pub value: u8,
}
#[derive(Debug, Clone, Default)]
struct RealTimeCounterRegister {
    microseconds: u64,
    seconds: u8,
    minutes: u8,
    hours: u8,
    day_low: u8,
    day_high_flags: DayHighFlag,
}

impl RealTimeCounterRegister {
    pub fn add_time(&mut self, duration: Duration) {
        // https://gbdev.io/pandocs/MBC3.html#the-clock-counter-registers
        let microseconds = duration.as_micros() + self.microseconds as u128;
        self.microseconds = (microseconds % 1000_000) as u64;
        let seconds = (microseconds / 1000_000) as u64 + self.seconds as u64;
        self.seconds = (seconds % 60) as u8;
        let minutes = (seconds / 60) + self.minutes as u64;
        self.minutes = (minutes % 60) as u8;
        let hours = (seconds / 60) + self.hours as u64;
        self.hours = (hours % 24) as u8;
        let days_low = (hours / 24) + self.day_low as u64;
        self.day_low = (days_low % (u8::MAX as u64)) as u8;
        let days_high = (days_low / (u8::MAX as u64)) + self.day_high_flags.value as u64;
        self.day_high_flags.write_day_high((days_high & 0b1) as u8);
        let flag = (self.day_high_flags.read_day_carry() > 0) || days_high > 1;
        self.day_high_flags.write_day_carry(flag as u8);
    }
}

// https://gbdev.io/pandocs/MBC3.html#the-clock-counter-registers
const RTC_SECONDS_SELECT_VALUE: u8 = 0x08;
const RTC_MINUTES_SELECT_VALUE: u8 = 0x09;
const RTC_HOURS_SELECT_VALUE: u8 = 0x0A;
const RTC_DAY_LOW_SELECT_VALUE: u8 = 0x0B;
const RTC_DAY_HIGH_SELECT_VALUE: u8 = 0x0C;

struct RealTimeCounter {
    register: RealTimeCounterRegister,
    latched: RealTimeCounterRegister,
    latch_control: u8,
    is_latched: bool,
}

impl RealTimeCounter {
    pub fn new() -> Self {
        Self {
            register: RealTimeCounterRegister::default(),
            latched: RealTimeCounterRegister::default(),
            // Set value different from 0 0r 1 to ensure the latch is not enabled by simply writing a 1.
            // https://gbdev.io/pandocs/MBC3.html#6000-7fff---latch-clock-data-write-only
            latch_control: 2,
            is_latched: false,
        }
    }

    pub fn latch(&mut self, value: u8) {
        if self.latch_control == 0 && value == 1 {
            self.is_latched = !self.is_latched;
            if self.is_latched {
                self.latched = self.register.clone();
            }
        }
        self.latch_control = value;
    }

    pub fn read(&self, selected_value: u8) -> u8 {
        let register = if self.is_latched {
            &self.latched
        } else {
            &self.register
        };

        match selected_value {
            RTC_SECONDS_SELECT_VALUE => register.seconds,
            RTC_MINUTES_SELECT_VALUE => register.minutes,
            RTC_HOURS_SELECT_VALUE => register.hours,
            RTC_DAY_LOW_SELECT_VALUE => register.day_low,
            RTC_DAY_HIGH_SELECT_VALUE => register.day_high_flags.value,
            _ => 0,
        }
    }

    pub fn write(&mut self, selected_value: u8, value: u8) {
        let register = if self.is_latched {
            &mut self.latched
        } else {
            &mut self.register
        };

        match selected_value {
            RTC_SECONDS_SELECT_VALUE => {
                register.seconds = value;
            }
            RTC_MINUTES_SELECT_VALUE => {
                register.minutes = value;
            }
            RTC_HOURS_SELECT_VALUE => {
                register.hours = value;
            }
            RTC_DAY_LOW_SELECT_VALUE => {
                register.day_low = value;
            }
            RTC_DAY_HIGH_SELECT_VALUE => {
                register.day_high_flags.value = value;
            }
            _ => {}
        }
    }

    pub fn update(&mut self, duration: Duration) {
        self.register.add_time(duration);
    }
}
