use crate::cartridge::Cartridge;
use crate::interrupts::Interrupt;
use crate::joypad::JoypadInput;
use crate::memory::gbmemory::GBMemory;
use crate::memory::init::{init_memory, init_registers};
use crate::memory::ram::RamController;
use crate::memory::registers::Registers;
use crate::serial::SerialTransfer;
use crate::sound::SoundController;
use crate::timer::Timer;
use crate::video::controller::VideoController;

struct Emulator {
    memory: GBMemory,
    registers: Registers,
}

impl Emulator {
    pub fn new(cartridge: Cartridge) -> Self {
        let mut memory = GBMemory::new(
            cartridge.memory_controller,
            VideoController::new(),
            RamController::new(),
            JoypadInput::new(),
            SerialTransfer {},
            SoundController {},
            Timer::new(),
        );
        init_memory(cartridge.cgb_flag.clone(), &mut memory);
        let mut registers = Registers::new();
        init_registers(cartridge.cgb_flag, &mut registers);

        Self { memory, registers }
    }

    pub fn update(&mut self) {
        let mut nb_cycles = 0u64 ;
        if self.registers.ime_flag {
            if let Some(interrupt) = &self.memory.get_enabled_interrupt() {
                nb_cycles += self.handle_interrupt(interrupt);
            }
        }

        if !(self.registers.halted && self.registers.ime_flag) {
            nb_cycles += self.fetch_and_execute();
        } else {
            nb_cycles += 4; // TODO: confirm the number of cycles to spend during halt
        }

        self.memory.update(nb_cycles);
    }

    pub fn handle_interrupt(&mut self, interrupt: &Interrupt) -> u64 {
        todo!()
    }

    pub fn fetch_and_execute(&mut self) -> u64 {
        todo!()
    }
}
