const SERIAL_TRANSFER_START_ADDRESS: u16 = 0xFF01;
const SERIAL_TRANSFER_DATA_ADDRESS: u16 = 0xFF01;
const SERIAL_TRANSFER_CONTROL_ADDRESS: u16 = 0xFF02;
const SERIAL_TRANSFER_END_ADDRESS: u16 = 0xFF02;

struct SerialTransfer {}

impl SerialTransfer {
    pub fn read(&self) -> u8 {
        // TODO: Implement serial communication.
        0
    }

    pub fn write(&mut self, _value: u8) {
        // TODO: Implement serial communication.
    }
}
