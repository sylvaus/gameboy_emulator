pub const ROM_START_ADDRESS: u16 = 0x0000;
pub const ROM_END_ADDRESS: u16 = 0x7FFF;
pub const EXT_RAM_START_ADDRESS: u16 = 0xA000;
pub const EXT_RAM_END_ADDRESS: u16 = 0xBFFF;

pub trait MemoryBankController {
    fn write_rom(&mut self, address: u16, value: u8);
    fn read_rom(&self, address: u16) -> u8;
    fn write_ext_ram(&mut self, address: u16, value: u8);
    fn read_ext_ram(&self, address: u16) -> u8;
}
