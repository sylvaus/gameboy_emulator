use crate::cartridge::Cartridge;
use crate::generated::instructions::{get_instruction, ImmediateArgumentType};
use crate::interrupts::Interrupt;
use crate::joypad::{InputProvider, JoypadInput};
use crate::memory::argument::Argument;
use crate::memory::cgb::CGBRegisters;
use crate::memory::gbmemory::GBMemory;
use crate::memory::init::{init_memory, init_registers};
use crate::memory::ram::RamController;
use crate::memory::registers::Registers;
use crate::memory::Memory;
use crate::serial::SerialTransfer;
use crate::sound::SoundController;
use crate::timer::Timer;
use crate::video::controller::VideoController;
use crate::video::renderer::VideoRenderer;
use std::convert::Into;
use std::ops::Div;
use std::thread::sleep;
use std::time::Duration;
use std::time::Instant;

const CPU_FREQUENCY: u32 = 1 << 22;

pub struct Emulator<Renderer, Input> {
    memory: GBMemory,
    registers: Registers,
    renderer: Renderer,
    input_provider: Input,
}

impl<Renderer, Input> Emulator<Renderer, Input>
where
    Renderer: VideoRenderer,
    Input: InputProvider,
{
    pub fn new(cartridge: Cartridge, renderer: Renderer, input_provider: Input) -> Self {
        let mut memory = GBMemory::new(
            cartridge.memory_controller,
            VideoController::new(),
            RamController::new(),
            JoypadInput::new(),
            SerialTransfer {},
            SoundController {},
            Timer::new(),
            CGBRegisters::default(),
        );
        init_memory(cartridge.cgb_flag.clone(), &mut memory);
        let mut registers = Registers::new();
        init_registers(cartridge.cgb_flag, &mut registers);

        memory.init();
        Self {
            memory,
            registers,
            renderer,
            input_provider,
        }
    }

    pub fn run(&mut self) {
        while !self.input_provider.is_quit_pressed() {
            let start_time = Instant::now();

            let nb_cycles = self.update();

            let expected_time = Duration::from_secs(nb_cycles).div(CPU_FREQUENCY);
            let time_left = expected_time.saturating_sub(Instant::now().duration_since(start_time));
            if time_left > Duration::ZERO {
                sleep(time_left);
            }
        }
    }

    pub fn update(&mut self) -> u64 {
        let mut nb_cycles = 0u64;
        if self.registers.ime_flag {
            if let Some(interrupt) = self.memory.get_enabled_interrupt() {
                nb_cycles += self.handle_interrupt(interrupt);
            }
        }

        if !(self.registers.halted && self.registers.ime_flag) {
            nb_cycles += self.fetch_and_execute();
        } else {
            // TODO: add handling when ime_flag is false and halted.
            nb_cycles += 4; // TODO: confirm the number of cycles to spend during halt
        }

        self.memory.update(nb_cycles);
        if self.memory.video.start_generating_line() {
            self.renderer.render(&self.memory.video);
            /// Only update the inputs when a frame is completed to avoid polling too often.
            self.input_provider.update();
            if self.input_provider.set_inputs(&mut self.memory.joypad) {
                self.memory.set_interrupt_flag(Interrupt::Joypad)
            }
        }
        nb_cycles
    }

    pub fn handle_interrupt(&mut self, interrupt: Interrupt) -> u64 {
        // Information from: https://gbdev.io/pandocs/Interrupts.html#interrupt-handling
        // step 1: Reset ime flag and interrupt flag
        self.registers.ime_flag = false;
        self.memory.reset_interrupt_flag(interrupt);

        // step 2: push program counter on the stack and jump to interrupt address
        self.memory.write(
            self.registers.sp - 1u16,
            ((self.registers.pc >> 8u16) & 0xFFu16) as u8,
        );
        self.memory.write(
            self.registers.sp - 2u16,
            (self.registers.pc & 0xFFu16) as u8,
        );
        self.registers.sp = self.registers.sp - 2u16;
        self.registers.pc = interrupt.get_address();

        // 5 M-cycles
        20
    }

    pub fn fetch_and_execute(&mut self) -> u64 {
        let mut opcode: u16 = self.memory.read(self.registers.pc).into();
        let mut argument_pc = self.registers.pc + 1;
        if opcode == 0xCB {
            opcode = 0x100u16 + self.memory.read(self.registers.pc + 1) as u16;
            argument_pc += 1;
        }

        let (instruction, argument_type) = get_instruction(opcode);
        let argument = match argument_type {
            ImmediateArgumentType::None => Argument::new_empty(),
            ImmediateArgumentType::Unsigned8Bits => Argument::new_u8(self.memory.read(argument_pc)),
            ImmediateArgumentType::Signed8Bits => {
                Argument::new_i8(self.memory.read_signed(argument_pc))
            }
            ImmediateArgumentType::Unsigned16Bits => {
                Argument::new_u16(self.memory.read_16_bits(argument_pc))
            }
        };

        instruction(&mut self.registers, &mut self.memory, &argument)
    }
}
