use std::path::Path;
use serde::{Deserialize, Serialize};
use crate::cartridge::Cartridge;
use crate::joypad::JoypadInput;
use crate::memory::cgb::CGBRegisters;
use crate::memory::gbmemory::GBMemory;
use crate::memory::init::{init_memory, init_registers};
use crate::memory::ram::RamController;
use crate::memory::registers::Registers;
use crate::serial::SerialTransfer;
use crate::sound::SoundController;
use crate::storage::helpers::{load_from_file, store_to_file};
use crate::storage::interface::{LoadError, SaveState, StorageVersion, StoreError};
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

const CURRENT_STORAGE_VERSION: StorageVersion = StorageVersion::Version1;

#[derive(Serialize, Deserialize)]
struct EmulatorSaveState {
    version: StorageVersion,
    memory: String,
    registers: String,
    renderer: String,
}

impl EmulatorState {
    pub fn save(&self, path: &Path) -> Result<(), StoreError> {
        let value = EmulatorSaveState {
            version: CURRENT_STORAGE_VERSION,
            memory: self.memory.save()?,
            registers: self.registers.save()?,
            renderer: self.renderer.save()?,
        };
        store_to_file(&value, path)
    }

    pub fn load(&mut self, path: &Path) -> Result<(), LoadError> {
        let raw_data: EmulatorSaveState = load_from_file(path)?;

        self.memory.load(&raw_data.memory, raw_data.version)?;
        self.registers.load(&raw_data.registers, raw_data.version)?;
        self.renderer.load(&raw_data.renderer, raw_data.version)?;

        Ok(())
    }
}
