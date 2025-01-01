use macros::BitAccessor;

pub const SERIAL_TRANSFER_START_ADDRESS: u16 = 0xFF01;
pub const SERIAL_TRANSFER_DATA_ADDRESS: u16 = 0xFF01;
pub const SERIAL_TRANSFER_CONTROL_ADDRESS: u16 = 0xFF02;
pub const SERIAL_TRANSFER_END_ADDRESS: u16 = 0xFF02;


const CGB_CONTROL_UNUSED_BITS: u8 = 0b0111_1100;
const DMG_CONTROL_UNUSED_BITS: u8 = 0b0111_1110;

#[derive(BitAccessor, Debug)]
pub struct SerialTransfer {
    // https://gbdev.io/pandocs/Serial_Data_Transfer_(Link_Cable).html#ff01--sb-serial-transfer-data
    data: u8,

    // https://gbdev.io/pandocs/Serial_Data_Transfer_(Link_Cable).html#ff02--sc-serial-transfer-control
    #[bit_offset_size(transfer_enable, 1, 1)]
    #[bit_offset_size(control_unused, 2, 5)]
    #[bit_offset_size(clock_speed, 1, 1)]
    #[bit_offset_size(clock_select, 0, 1)]
    control: u8,

    is_cgb: bool,
}

impl SerialTransfer {
    pub fn new(is_cgb: bool) -> Self {
        Self {
            data: 0,
            control: 0,
            is_cgb,
        }
    }

    pub fn read(&self, address: u16) -> u8 {
        match address {
            0xFF01 => self.data,
            0xFF02 => {
                if self.is_cgb {
                    self.control | CGB_CONTROL_UNUSED_BITS
                } else {
                    self.control | DMG_CONTROL_UNUSED_BITS
                }
            }

            _ => panic!("This function should never be called with address outside range [0xFF10, 0xFF3F], called with {}", address),
        }
    }

    pub fn write(&mut self, address: u16, value: u8) {
        match address {
            0xFF01 => self.data = value,
            0xFF02 => self.control = value,

            _ => panic!("This function should never be called with address outside range [0xFF10, 0xFF3F], called with {}", address),
        }
    }
}
