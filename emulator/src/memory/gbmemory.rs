use crate::joypad::{JoypadInput, JOYPAD_INPUT_ADDRESS};
use crate::memory::mbc::interface::{
    MemoryBankController, EXT_RAM_END_ADDRESS, EXT_RAM_START_ADDRESS, ROM_END_ADDRESS,
    ROM_START_ADDRESS,
};
use crate::memory::ram::{
    RamController, ECHO_RAM_END_ADDRESS, ECHO_RAM_START_ADDRESS, HIGH_RAM_END_ADDRESS,
    HIGH_RAM_START_ADDRESS, SELECT_WORK_RAM_BANK_ADDRESS, WORK_RAM_END_ADDRESS,
    WORK_RAM_START_ADDRESS,
};
use crate::memory::Memory;
use crate::serial::{SerialTransfer, SERIAL_TRANSFER_END_ADDRESS, SERIAL_TRANSFER_START_ADDRESS};
use crate::sound::SoundController;
use crate::timer::{Timer, TIMER_END_ADDRESS, TIMER_START_ADDRESS};
use crate::video::controller::{
    VideoController, BG_OBJ_PALETTES_START_ADDRESS, BG_OBJ_PALETTES__END_ADDRESS,
    IO_LCD_END_ADDRESS, IO_LCD_START_ADDRESS, OAM_DMA_ADDRESS, OAM_END_ADDRESS, OAM_SIZE,
    OAM_START_ADDRESS, VRAM_BANK_SELECT, VRAM_END_ADDRESS, VRAM_START_ADDRESS,
};

/// Information from: https://gbdev.io/pandocs/Memory_Map.html#memory-map
pub const NOT_USABLE_START_ADDRESS: u16 = 0xFEA0;
pub const NOT_USABLE_END_ADDRESS: u16 = 0xFEFF;
pub const BEFORE_OAM_DMA_ADDRESS: u16 = OAM_DMA_ADDRESS - 1;
pub const AFTER_OAM_DMA_ADDRESS: u16 = OAM_DMA_ADDRESS + 1;
pub const DISABLE_BOOT_ROM_ADDRESS: u16 = 0xFF50;
pub const VRAM_DMA_START_ADDRESS: u16 = 0xFF51;
pub const VRAM_DMA_END_ADDRESS: u16 = 0xFF55;
pub const INTERRUPT_ENABLE_REGISTER_ADDRESS: u16 = 0xFFFF;

pub struct GBMemory {
    mbc: Box<dyn MemoryBankController>,
    video: VideoController,
    ram: RamController,
    joypad: JoypadInput,
    serial: SerialTransfer,
    sound: SoundController,
    timer: Timer,

    oam_dma_high_bits: u8,
    interrupt_enable: u8,
    boot_rom_disabled: u8,
}

impl GBMemory {
    pub fn new(
        mbc: Box<dyn MemoryBankController>,
        video: VideoController,
        ram: RamController,
        joypad: JoypadInput,
        serial: SerialTransfer,
        sound: SoundController,
        timer: Timer,
    ) -> Self {
        Self {
            mbc,
            video,
            ram,
            joypad,
            serial,
            sound,
            timer,
            oam_dma_high_bits: 0,
            interrupt_enable: 0,
            boot_rom_disabled: 0,
        }
    }

    fn write_oam_dma(&mut self, value: u8) {
        // This should add some machine cycles somewhere
        // Source: https://gbdev.io/pandocs/OAM_DMA_Transfer.html
        self.oam_dma_high_bits = value;
        let high_bits: u16 = (value as u16) << 8;
        for i in 0u16..(OAM_SIZE as u16) {
            self.write(OAM_START_ADDRESS + i, self.read(high_bits + i));
        }
    }

    fn read_vram_dma(&self, _address: u16) -> u8 {
        // TODO implement vram dma for CGB: https://gbdev.io/pandocs/CGB_Registers.html#lcd-vram-dma-transfers
        0
    }

    fn write_vram_dma(&mut self, _address: u16, _value: u8) {
        // TODO implement vram dma for CGB: https://gbdev.io/pandocs/CGB_Registers.html#lcd-vram-dma-transfers
    }
}

impl Memory for GBMemory {
    fn read(&self, address: u16) -> u8 {
        match address {
            ROM_START_ADDRESS..=ROM_END_ADDRESS => self.mbc.read_rom(address),
            VRAM_START_ADDRESS..=VRAM_END_ADDRESS => self.video.read_vram(address),
            EXT_RAM_START_ADDRESS..=EXT_RAM_END_ADDRESS => self.mbc.read_ext_ram(address),
            WORK_RAM_START_ADDRESS..=WORK_RAM_END_ADDRESS => self.ram.read_work_ram(address),
            ECHO_RAM_START_ADDRESS..=ECHO_RAM_END_ADDRESS => self.ram.read_echo_ram(address),
            OAM_START_ADDRESS..=OAM_END_ADDRESS => self.video.read_oam(address),
            NOT_USABLE_START_ADDRESS..=NOT_USABLE_END_ADDRESS => {
                panic!("Trying to read unusable address {:?}", address)
            }
            JOYPAD_INPUT_ADDRESS => self.joypad.value,
            SERIAL_TRANSFER_START_ADDRESS..=SERIAL_TRANSFER_END_ADDRESS => {
                self.serial.read(address)
            }
            TIMER_START_ADDRESS..=TIMER_END_ADDRESS => self.timer.read(address),
            IO_LCD_START_ADDRESS..=BEFORE_OAM_DMA_ADDRESS => self.video.read_lcd(address),
            OAM_DMA_ADDRESS => self.oam_dma_high_bits,
            AFTER_OAM_DMA_ADDRESS..=IO_LCD_END_ADDRESS => self.video.read_lcd(address),
            VRAM_BANK_SELECT => self.video.read_vram_bank(),
            DISABLE_BOOT_ROM_ADDRESS => self.boot_rom_disabled,
            VRAM_DMA_START_ADDRESS..=VRAM_DMA_END_ADDRESS => self.read_vram_dma(address),
            BG_OBJ_PALETTES_START_ADDRESS..=BG_OBJ_PALETTES__END_ADDRESS => {
                self.video.read_cgb_lcd_color_palette(address)
            }
            SELECT_WORK_RAM_BANK_ADDRESS => self.ram.read_selected_work_ram_bank(),
            HIGH_RAM_START_ADDRESS..=HIGH_RAM_END_ADDRESS => self.ram.read_high_ram(address),
            INTERRUPT_ENABLE_REGISTER_ADDRESS => self.interrupt_enable,

            address => panic!("Trying to read unknown address {:?}", address),
        }
    }

    fn read_signed(&self, address: u16) -> i8 {
        self.read(address) as i8
    }

    fn write(&mut self, address: u16, value: u8) {
        match address {
            ROM_START_ADDRESS..=ROM_END_ADDRESS => self.mbc.write_rom(address, value),
            VRAM_START_ADDRESS..=VRAM_END_ADDRESS => self.video.write_vram(address, value),
            EXT_RAM_START_ADDRESS..=EXT_RAM_END_ADDRESS => self.mbc.write_ext_ram(address, value),
            WORK_RAM_START_ADDRESS..=WORK_RAM_END_ADDRESS => {
                self.ram.write_work_ram(address, value)
            }
            ECHO_RAM_START_ADDRESS..=ECHO_RAM_END_ADDRESS => {
                self.ram.write_echo_ram(address, value)
            }
            OAM_START_ADDRESS..=OAM_END_ADDRESS => self.video.write_oam(address, value),
            NOT_USABLE_START_ADDRESS..=NOT_USABLE_END_ADDRESS => {
                panic!("Trying to write to unusable address {:?}", address)
            }
            JOYPAD_INPUT_ADDRESS => self.joypad.value = value,
            SERIAL_TRANSFER_START_ADDRESS..=SERIAL_TRANSFER_END_ADDRESS => {
                self.serial.write(address, value)
            }
            TIMER_START_ADDRESS..=TIMER_END_ADDRESS => self.timer.write(address, value),
            IO_LCD_START_ADDRESS..=BEFORE_OAM_DMA_ADDRESS => self.video.write_lcd(address, value),
            OAM_DMA_ADDRESS => self.write_oam_dma(value),
            AFTER_OAM_DMA_ADDRESS..=IO_LCD_END_ADDRESS => self.video.write_lcd(address, value),
            VRAM_BANK_SELECT => self.video.write_vram_bank(value),
            DISABLE_BOOT_ROM_ADDRESS => self.boot_rom_disabled = value,
            VRAM_DMA_START_ADDRESS..=VRAM_DMA_END_ADDRESS => self.write_vram_dma(address, value),
            BG_OBJ_PALETTES_START_ADDRESS..=BG_OBJ_PALETTES__END_ADDRESS => {
                self.video.write_cgb_lcd_color_palette(address, value)
            }
            SELECT_WORK_RAM_BANK_ADDRESS => self.ram.write_selected_work_ram_bank(value),
            HIGH_RAM_START_ADDRESS..=HIGH_RAM_END_ADDRESS => {
                self.ram.write_high_ram(address, value)
            }
            INTERRUPT_ENABLE_REGISTER_ADDRESS => self.interrupt_enable = value,

            address => panic!("Trying to write unknown address {:?}", address),
        }
    }
}
