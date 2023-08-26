pub(crate) mod registers;

pub trait Memory {
    fn get(&self, address: u16) -> u8;
    fn get_signed(&self, address: u16) -> i8;
    fn get_16_bits(&self, address: u16) -> u16;

    fn set(&mut self, address: u16, value: u8);

    fn set_16_bits(&mut self, address: u16, value: u16) {
        self.set(address, (value & 0xFF) as u8);
        self.set(address + 1, (value >> 8) as u8);
    }
}
