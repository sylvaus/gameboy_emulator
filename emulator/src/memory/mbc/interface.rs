use std::time::Duration;

pub const ROM_START_ADDRESS: u16 = 0x0000;
pub const ROM_BANK_0_END_ADDRESS: u16 = 0x3FFF;
pub const ROM_BANK_1_N_START_ADDRESS: u16 = 0x4000;
pub const ROM_BANK_1_N_END_ADDRESS: u16 = 0x7FFF;
pub const EXT_RAM_START_ADDRESS: u16 = 0xA000;
pub const EXT_RAM_END_ADDRESS: u16 = 0xBFFF;
pub const ROM_BANK_SIZE: usize = 0x4000;
pub const RAM_BANK_SIZE: usize = 0x2000;

pub trait MemoryBankController: Send {
    /// This method handles the writes for addresses in range [0x0000, 0x7FFFF]
    fn write_rom(&mut self, address: u16, value: u8);

    /// This method handles the reads for addresses in range [0x0000, 0x7FFFF]
    fn read_rom(&self, address: u16) -> u8;

    /// This method handles the writes for addresses in range [0xA000, 0xBFFF]
    fn write_ext_ram(&mut self, address: u16, value: u8);

    /// This method handles the reads for addresses in range [0xA000, 0xBFFF]
    fn read_ext_ram(&self, address: u16) -> u8;

    /// This method should be called after each instruction to ensure that the RTC registers are updated appropriately.
    fn update(&mut self, duration: Duration);
}
