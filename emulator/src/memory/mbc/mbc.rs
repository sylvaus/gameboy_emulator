use std::time::Duration;
use crate::memory::mbc::mbc1::MBC1BankController;
use crate::memory::mbc::mbc3::MBC3BankController;
use crate::memory::mbc::mbc5::MBC5BankController;
use crate::memory::mbc::no_controller::NoMemoryBankController;
use crate::storage::interface::{LoadError, SaveState, StorageVersion, StoreError};

#[derive(Debug)]
pub enum MemoryBankController {
    NoController(NoMemoryBankController),
    Mbc1(MBC1BankController),
    Mbc3(MBC3BankController),
    Mbc5(MBC5BankController),
}

impl MemoryBankController {
    /// This method handles the writes for addresses in range [0x0000, 0x7FFF]
    pub fn write_rom(&mut self, address: u16, value: u8) {
        match self {
            MemoryBankController::NoController(controller) => controller.write_rom(address, value),
            MemoryBankController::Mbc1(controller) => controller.write_rom(address, value),
            MemoryBankController::Mbc3(controller) => controller.write_rom(address, value),
            MemoryBankController::Mbc5(controller) => controller.write_rom(address, value),
        }
    }

    /// This method handles the reads for addresses in range [0x0000, 0x7FFF]
    pub fn read_rom(&self, address: u16) -> u8 {
        match self {
            MemoryBankController::NoController(controller) => controller.read_rom(address),
            MemoryBankController::Mbc1(controller) => controller.read_rom(address),
            MemoryBankController::Mbc3(controller) => controller.read_rom(address),
            MemoryBankController::Mbc5(controller) => controller.read_rom(address),
        }
    }

    /// This method handles the writes for addresses in range [0xA000, 0xBFFF]
    pub fn write_ext_ram(&mut self, address: u16, value: u8) {
        match self {
            MemoryBankController::NoController(controller) => controller.write_ext_ram(address, value),
            MemoryBankController::Mbc1(controller) => controller.write_ext_ram(address, value),
            MemoryBankController::Mbc3(controller) => controller.write_ext_ram(address, value),
            MemoryBankController::Mbc5(controller) => controller.write_ext_ram(address, value),
        }
    }

    /// This method handles the reads for addresses in range [0xA000, 0xBFFF]
    pub fn read_ext_ram(&self, address: u16) -> u8 {
        match self {
            MemoryBankController::NoController(controller) => controller.read_ext_ram(address),
            MemoryBankController::Mbc1(controller) => controller.read_ext_ram(address),
            MemoryBankController::Mbc3(controller) => controller.read_ext_ram(address),
            MemoryBankController::Mbc5(controller) => controller.read_ext_ram(address),
        }
    }

    /// This method should be called after each instruction to ensure that the RTC registers are updated appropriately.
    pub fn update(&mut self, duration: Duration) {
        match self {
            MemoryBankController::NoController(controller) => controller.update(duration),
            MemoryBankController::Mbc1(controller) => controller.update(duration),
            MemoryBankController::Mbc3(controller) => controller.update(duration),
            MemoryBankController::Mbc5(controller) => controller.update(duration),
        }
    }
}

impl SaveState for MemoryBankController {
    fn save(&self) -> Result<String, StoreError> {
        match self {
            MemoryBankController::NoController(controller) => controller.save(),
            MemoryBankController::Mbc1(controller) => controller.save(),
            MemoryBankController::Mbc3(controller) => controller.save(),
            MemoryBankController::Mbc5(controller) => controller.save(),
        }
    }

    fn load(&mut self, state: &str, version: StorageVersion) -> Result<(), LoadError> {
        match self {
            MemoryBankController::NoController(controller) => controller.load(state, version),
            MemoryBankController::Mbc1(controller) => controller.load(state, version),
            MemoryBankController::Mbc3(controller) => controller.load(state, version),
            MemoryBankController::Mbc5(controller) => controller.load(state, version),
        }
    }
}
