pub const SERIAL_TRANSFER_START_ADDRESS: u16 = 0xFF01;
pub const SERIAL_TRANSFER_DATA_ADDRESS: u16 = 0xFF01;
pub const SERIAL_TRANSFER_CONTROL_ADDRESS: u16 = 0xFF02;
pub const SERIAL_TRANSFER_END_ADDRESS: u16 = 0xFF02;

pub struct SerialTransfer {}

impl SerialTransfer {
    pub fn read(&self, _address: u16) -> u8 {
        // TODO: Implement serial communication.
        0
    }

    pub fn write(&mut self, _address: u16, _value: u8) {
        // TODO: Implement serial communication.
    }
}
