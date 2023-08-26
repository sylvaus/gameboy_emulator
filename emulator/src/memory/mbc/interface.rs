pub trait MemoryBankController {
    fn write_rom(&mut self, address: u16, value: u8);
    fn read_rom(&self, address: u16);
    fn write_ext_ram(&mut self, address: u16, value: u8);
    fn read_ext_ram(&self, address: u16);
}
