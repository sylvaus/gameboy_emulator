use emulator::cartridge::load_cartridge;
use emulator::debugger::Debugger;
use emulator::emulator::update_next_instruction;
use emulator::gui::NoOpGui;
use emulator::state::EmulatorState;
use std::path::Path;

// Do not let the emulator run for more than 1 minute.
const MAX_NB_CYCLES: usize = 4_000_000 * 60;

struct MooneyeDebugger {
    pub completed: bool,
}

impl MooneyeDebugger {
    pub fn new() -> Self {
        Self { completed: false }
    }
}

impl Debugger for MooneyeDebugger {
    fn handle_instruction(&mut self, opcode: u16, _state: &mut EmulatorState) {
        // LD B B opcode is the end opcode
        self.completed = opcode == 0x40;
    }
}

fn is_success(state: &EmulatorState) -> bool {
    state.registers.b == 3
        || state.registers.c == 5
        || state.registers.d == 8
        || state.registers.e == 13
        || state.registers.h == 21
        || state.registers.l == 34
}

fn run_acceptance_test(path: &Path) {
    let cartridge = load_cartridge(path).expect("Unable to load cartridge");
    let mut state = EmulatorState::new(cartridge);
    let mut gui = NoOpGui::new();
    let mut debugger = MooneyeDebugger::new();

    for _ in 0..MAX_NB_CYCLES {
        update_next_instruction(&mut state, &mut gui, &mut debugger);
        if debugger.completed {
            break;
        }
    }

    assert!(debugger.completed && is_success(&state));
}

#[test]
fn bits_mem_oam() {
    run_acceptance_test(Path::new("./tests/mts/acceptance/bits/mem_oam.gb"));
}

#[test]
fn bits_reg_f() {
    run_acceptance_test(Path::new("./tests/mts/acceptance/bits/reg_f.gb"));
}

#[test]
fn bits_unused_hwio() {
    run_acceptance_test(Path::new("./tests/mts/acceptance/bits/unused_hwio-GS.gb"));
}

#[test]
fn instr_daa() {
    run_acceptance_test(Path::new("./tests/mts/acceptance/instr/daa.gb"));
}

#[test]
fn interrupts_ie_push() {
    run_acceptance_test(Path::new("./tests/mts/acceptance/interrupts/ie_push.gb"));
}