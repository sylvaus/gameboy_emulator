/// TODO: Implement the CGB variant
/// Information from: https://gbdev.io/pandocs/Memory_Map.html#memory-map

const WORK_RAM_SIZE: usize = 0x1000;
const WORK_RAM_START_ADDRESS: u16 = 0xC000;
const ECHO_RAM_SIZE: usize = 0x1E00;
const ECHO_RAM_START_ADDRESS: u16 = 0xE000;

const HIGH_RAM_SIZE: usize = 0x7F;
const HIGH_RAM_START_ADDRESS: u16 = 0xFF80;
/// Information from: https://gbdev.io/pandocs/CGB_Registers.html#ff70--svbk-cgb-mode-only-wram-bank
const SELECT_WORK_RAM_BANK_ADDRESS: u16 = 0xFF70;

struct RamController {
    work_ram: Vec<u8>,
    high_ram: Vec<u8>,
}

impl RamController {
    pub fn new() -> Self {
        Self {
            work_ram: vec![0u8; WORK_RAM_SIZE * 2usize],
            high_ram: vec![0u8; HIGH_RAM_SIZE],
        }
    }

    pub fn write_work_ram(&mut self, address: u16, value: u8) {
        self.work_ram[(address - WORK_RAM_START_ADDRESS) as usize] = value;
    }

    pub fn read_work_ram(&mut self, address: u16) -> u8 {
        self.work_ram[(address - WORK_RAM_START_ADDRESS) as usize]
    }

    pub fn write_echo_ram(&mut self, address: u16, value: u8) {
        /// Just echo work ram
        self.write_work_ram(address - 0x2000, value)
    }

    pub fn read_echo_ram(&mut self, address: u16) -> u8 {
        self.read_work_ram(address - 0x2000)
    }

    pub fn write_high_ram(&mut self, address: u16, value: u8) {
        self.high_ram[(address - WORK_RAM_START_ADDRESS) as usize] = value;
    }

    pub fn read_high_ram(&mut self, address: u16) -> u8 {
        self.high_ram[(address - WORK_RAM_START_ADDRESS) as usize]
    }

    pub fn select_work_ram_bank() {
        /// see: https://gbdev.io/pandocs/CGB_Registers.html#ff70--svbk-cgb-mode-only-wram-bank
        todo!("Implement CGB variant where this should select the ram bank");
    }

    pub fn get_selected_work_ram_bank() -> u8 {
        /// see: https://gbdev.io/pandocs/CGB_Registers.html#ff70--svbk-cgb-mode-only-wram-bank
        todo!("Implement CGB variant where this should return the selected ram bank");
        return 0;
    }
}
