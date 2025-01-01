use crate::cartridge::Cartridge;
use crate::debugger::{Debugger, NoOpDebugger};
use crate::generated::instructions::{get_instruction, ImmediateArgumentType};
use crate::gui::Gui;
use crate::interrupts::Interrupt;
use crate::joypad::{InputProvider, JoypadState};
use crate::memory::argument::Argument;
use crate::memory::Memory;
use crate::state::EmulatorState;
use crate::statistics::StatisticsRecorder;
use crate::throttler::Throttler;
use crate::video::renderer::{Color, Screen};
use std::convert::Into;
use std::sync::mpsc;
use std::thread;
use std::thread::JoinHandle;

pub struct InstructionUpdate {
    pub nb_cycles: u64,
    pub update_frame: bool,
}

pub fn update_next_instruction(
    state: &mut EmulatorState,
    gui: &mut impl Gui,
    debugger: &mut impl Debugger,
) -> InstructionUpdate {
    let mut nb_cycles = 0u64;

    if let Some(interrupt) = state.memory.get_enabled_interrupt() {
        if state.registers.ime_flag {
            nb_cycles += handle_interrupt(state, interrupt);
        }
        state.registers.halted = false;
    }

    if !state.registers.halted {
        nb_cycles += fetch_and_execute(state, debugger);
    } else {
        // TODO: add handling when ime_flag is false and halted.
        nb_cycles += 4; // TODO: confirm the number of cycles to spend during halt
    }

    state.memory.update(nb_cycles);
    if state.memory.video.should_scanline() {
        state.renderer.scanline(&state.memory.video, |x, y, color| {
            gui.write_pixel(x, y, color)
        });
    }
    let mut update_frame = false;
    if state.memory.video.should_update_frame() {
        gui.update_frame();
        // Only update the inputs when a frame is completed to avoid polling too often.
        gui.update_inputs();
        update_frame = true;
    }
    if state.memory.joypad.write_state(&gui.get_inputs()) {
        state.memory.set_interrupt_flag(Interrupt::Joypad);
    }
    InstructionUpdate {
        nb_cycles,
        update_frame,
    }
}

fn handle_interrupt(state: &mut EmulatorState, interrupt: Interrupt) -> u64 {
    // Information from: https://gbdev.io/pandocs/Interrupts.html#interrupt-handling
    // step 1: Reset ime flag and interrupt flag
    state.registers.ime_flag = false;
    state.memory.reset_interrupt_flag(interrupt);

    // step 2: push program counter on the stack and jump to interrupt address
    state.memory.write(
        state.registers.sp - 1u16,
        ((state.registers.pc >> 8u16) & 0xFFu16) as u8,
    );
    state.memory.write(
        state.registers.sp - 2u16,
        (state.registers.pc & 0xFFu16) as u8,
    );
    state.registers.sp -= 2u16;
    state.registers.pc = interrupt.get_address();

    // 5 M-cycles
    20
}

fn fetch_and_execute(state: &mut EmulatorState, debugger: &mut impl Debugger) -> u64 {
    let mut opcode: u16 = state.memory.read(state.registers.pc).into();
    let mut argument_pc = state.registers.pc + 1;
    if opcode == 0xCB {
        opcode = 0x100u16 + state.memory.read(state.registers.pc + 1) as u16;
        argument_pc += 1;
    }

    debugger.handle_instruction(opcode, state);
    let (instruction, argument_type) = get_instruction(opcode);
    let argument = match argument_type {
        ImmediateArgumentType::None => Argument::new_empty(),
        ImmediateArgumentType::Unsigned8Bits => Argument::new_u8(state.memory.read(argument_pc)),
        ImmediateArgumentType::Signed8Bits => {
            Argument::new_i8(state.memory.read_signed(argument_pc))
        }
        ImmediateArgumentType::Unsigned16Bits => {
            Argument::new_u16(state.memory.read_16_bits(argument_pc))
        }
    };

    instruction(&mut state.registers, &mut state.memory, &argument)
}

pub struct ThreadedEmulator {
    handle: JoinHandle<()>,
    sender: mpsc::Sender<Action>,
}

impl Default for ThreadedEmulator {
    fn default() -> Self {
        Self::new()
    }
}

#[allow(dead_code)]
impl ThreadedEmulator {
    pub fn new() -> Self {
        let (sender, receiver) = mpsc::channel();
        let handle = thread::spawn(move || {
            thread_loop(receiver);
        });
        Self { handle, sender }
    }
    pub fn start(&mut self, cartridge: Cartridge, screen: Box<dyn Screen>) {
        if !self.handle.is_finished() {
            self.sender
                .send(Action::Start((cartridge, screen)))
                .expect("Channel is invalid");
        }
    }

    pub fn pause(&mut self) {
        if !self.handle.is_finished() {
            self.sender
                .send(Action::Pause())
                .expect("Channel is invalid");
        }
    }

    pub fn resume(&mut self) {
        if !self.handle.is_finished() {
            self.sender
                .send(Action::Resume())
                .expect("Channel is invalid");
        }
    }

    pub fn stop(&mut self) -> bool {
        if !self.handle.is_finished() {
            self.sender
                .send(Action::Stop())
                .expect("Channel is invalid");
            true
        } else {
            false
        }
    }

    pub fn update_inputs(&mut self, input: JoypadState) {
        if !self.handle.is_finished() {
            self.sender
                .send(Action::Inputs(input))
                .expect("Channel is invalid");
        }
    }
}

fn thread_loop(receiver: mpsc::Receiver<Action>) {
    let mut debugger = NoOpDebugger::new();
    let mut state = State::default();
    let mut throttler = Throttler::new();
    let mut stats_recorder = StatisticsRecorder::new();
    'main: loop {
        if let Ok(action) = receiver.recv() {
            update_state(&mut state, action)
        } else {
            break;
        }

        let mut nb_cycles = 0;
        'running: loop {
            for action in receiver.try_iter() {
                update_state(&mut state, action)
            }
            if state.input.should_quit {
                break 'main;
            }
            if state.input.is_paused || state.emulator.is_none() {
                break 'running;
            }
            let (emulator_state, screen) = &mut state.emulator.as_mut().unwrap();
            let mut gui = GuiMiddleware::new(screen, &state.input);
            let update = update_next_instruction(emulator_state, &mut gui, &mut debugger);

            nb_cycles += update.nb_cycles;
            if update.update_frame {
                throttler.throttle_for_cycles(nb_cycles);
                stats_recorder.record_frame();
                nb_cycles = 0;
            }
        }
    }
}

fn update_state(state: &mut State, action: Action) {
    match action {
        Action::Start((cartridge, screen)) => {
            state.input.is_paused = false;
            state.input.should_quit = false;
            state.input.joypad = Default::default();
            state.emulator = Some((EmulatorState::new(cartridge), screen))
        }
        Action::Pause() => {
            state.input.is_paused = true;
        }
        Action::Resume() => {
            state.input.is_paused = false;
        }
        Action::Stop() => {
            state.input.should_quit = false;
        }
        Action::Inputs(inputs) => {
            state.input.joypad = inputs;
        }
    }
}

#[derive(Default, Debug)]
struct InputState {
    pub is_paused: bool,
    pub should_quit: bool,
    pub joypad: JoypadState,
}

#[derive(Default)]
struct State {
    pub input: InputState,
    pub emulator: Option<(EmulatorState, Box<dyn Screen>)>,
}

enum Action {
    Start((Cartridge, Box<dyn Screen>)),
    Pause(),
    Resume(),
    Stop(),
    Inputs(JoypadState),
}

struct GuiMiddleware<'a> {
    screen: &'a mut Box<dyn Screen>,
    state: &'a InputState,
}

impl<'a> GuiMiddleware<'a> {
    pub fn new(screen: &'a mut Box<dyn Screen>, state: &'a InputState) -> Self {
        Self { screen, state }
    }
}

impl Screen for GuiMiddleware<'_> {
    fn write_pixel(&mut self, x: usize, y: usize, color: &Color) {
        self.screen.write_pixel(x, y, color);
    }

    fn update_frame(&mut self) {
        self.screen.update_frame();
    }
}

impl InputProvider for GuiMiddleware<'_> {
    fn update_inputs(&mut self) {}

    fn get_inputs(&self) -> JoypadState {
        self.state.joypad.clone()
    }
}

impl Gui for GuiMiddleware<'_> {}
