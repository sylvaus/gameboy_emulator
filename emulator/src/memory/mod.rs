pub(crate) mod argument;
pub(crate) mod mbc;
pub(crate) mod ram;
pub(crate) mod registers;
mod memory;

pub trait Memory {
    fn read(&self, address: u16) -> u8;
    fn read_signed(&self, address: u16) -> i8;
    fn read_16_bits(&self, address: u16) -> u16 {
        (self.read(address) as u16) + ((self.read(address) as u16) << 8)
    }

    fn write(&mut self, address: u16, value: u8);

    fn write_16_bits(&mut self, address: u16, value: u16) {
        self.write(address, (value & 0xFF) as u8);
        self.write(address + 1, (value >> 8) as u8);
    }
}
