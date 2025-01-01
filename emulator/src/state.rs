use crate::cartridge::Cartridge;
use crate::joypad::JoypadInput;
use crate::memory::cgb::CGBRegisters;
use crate::memory::gbmemory::GBMemory;
use crate::memory::init::{init_memory, init_registers};
use crate::memory::ram::RamController;
use crate::memory::registers::Registers;
use crate::serial::SerialTransfer;
use crate::sound::SoundController;
use crate::timer::Timer;
use crate::video::controller::VideoController;
use crate::video::renderer::CoreNonCgbRenderer;

pub struct EmulatorState {
    pub memory: GBMemory,
    pub registers: Registers,
    pub renderer: CoreNonCgbRenderer,
}

impl EmulatorState {
    pub fn new(cartridge: Cartridge) -> Self {
        let is_cgb = cartridge.cgb_flag.use_cgb();
        let mut memory = GBMemory::new(
            cartridge.memory_controller,
            VideoController::new(),
            RamController::new(),
            JoypadInput::new(),
            SerialTransfer::new(is_cgb),
            SoundController::default(),
            Timer::new(),
            CGBRegisters::new(is_cgb),
        );
        init_memory(cartridge.cgb_flag, &mut memory);
        let mut registers = Registers::new();
        init_registers(cartridge.cgb_flag, &mut registers);

        memory.init();
        Self {
            memory,
            registers,
            renderer: CoreNonCgbRenderer::new(),
        }
    }
}