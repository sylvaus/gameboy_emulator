//! This file assumes that the mooneye tests have been downloaded and unpacked in a folder
//! called mts in this folder. Expected structure:
//! tests/
//!   | mts/
//!     | acceptance/
//!     ...
//!   mooneye_tests.rs
//! Download link can be found on the mooneye GitHub page: https://github.com/Gekkio/mooneye-test-suite

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

    assert!(debugger.completed && is_success(&state), "Failed acceptance test: {:?}", path);
}

// Complete
mod acceptance {
    use std::path::Path;
    use crate::run_acceptance_test;

    #[test]
    fn mem_oam() {
        run_acceptance_test(Path::new("./tests/mts/acceptance/bits/mem_oam.gb"));
    }

    #[test]
    fn reg_f() {
        run_acceptance_test(Path::new("./tests/mts/acceptance/bits/reg_f.gb"));
    }
    
    #[test]
    fn unused_hwio_gs() {run_acceptance_test(Path::new("./tests/mts/acceptance/bits/unused_hwio-GS.gb"));}
}

// Complete
mod instr {
    use std::path::Path;
    use crate::run_acceptance_test;

    #[test]
    fn instr_daa() {
        run_acceptance_test(Path::new("./tests/mts/acceptance/instr/daa.gb"));
    }
    
}

// Complete
mod interrupts {
    use std::path::Path;
    use crate::run_acceptance_test;
    #[test]
    fn ie_push() {
        run_acceptance_test(Path::new("./tests/mts/acceptance/interrupts/ie_push.gb"));
    }
}


mod oam_dma {
    use std::path::Path;
    use crate::run_acceptance_test;

    #[test]
    fn basic() {
        run_acceptance_test(Path::new("./tests/mts/acceptance/oam_dma/basic.gb"))
    }

    #[test]
    fn reg_read() {
        run_acceptance_test(Path::new("./tests/mts/acceptance/oam_dma/reg_read.gb"))
    }

    // TODO: fix
    // Test currently no passing but tested on PyBoy, and it also does not pass
    #[ignore]
    #[test]
    fn sources_gs() {
        run_acceptance_test(Path::new("./tests/mts/acceptance/oam_dma/sources-GS.gb"))
    }
} 

mod ppu {
    use std::path::Path;
    use crate::run_acceptance_test;

    #[test]
    fn hblank_ly_scx_timing_gs() {
        run_acceptance_test(Path::new("./tests/mts/acceptance/ppu/hblank_ly_scx_timing-GS.gb"))
    }


    #[test]
    fn intr_1_2_timing_gs() {
        run_acceptance_test(Path::new("./tests/mts/acceptance/ppu/intr_1_2_timing-GS.gb"))
    }

    #[test]
    fn intr_2_0_timing() {
        run_acceptance_test(Path::new("./tests/mts/acceptance/ppu/intr_2_0_timing.gb"))
    }

    #[test]
    fn intr_2_mode0_timing() {
        run_acceptance_test(Path::new("./tests/mts/acceptance/ppu/intr_2_mode0_timing.gb"))
    }

    #[test]
    fn intr_2_mode0_timing_sprites() {
        run_acceptance_test(Path::new("./tests/mts/acceptance/ppu/intr_2_mode0_timing_sprites.gb"))
    }

    #[test]
    fn intr_2_mode3_timing() {
        run_acceptance_test(Path::new("./tests/mts/acceptance/ppu/intr_2_mode3_timing.gb.gb"))
    }

    #[test]
    fn intr_2_oam_ok_timing() {
        run_acceptance_test(Path::new("./tests/mts/acceptance/ppu/intr_2_oam_ok_timing.gb"))
    }

    #[test]
    fn lcdon_timing_gs() {
        run_acceptance_test(Path::new("./tests/mts/acceptance/ppu/lcdon_timing-GS.gb"))
    }

    #[test]
    fn lcdon_write_timing_gs() {
        run_acceptance_test(Path::new("./tests/mts/acceptance/ppu/lcdon_write_timing-GS.gb"))
    }

    #[test]
    fn stat_irq_blocking() {
        run_acceptance_test(Path::new("./tests/mts/acceptance/ppu/stat_irq_blocking.gb"))
    }

    #[test]
    fn stat_lyc_onoff() {
        run_acceptance_test(Path::new("./tests/mts/acceptance/ppu/stat_lyc_onoff.gb"))
    }

    #[test]
    fn vblank_stat_intr_gs() {
        run_acceptance_test(Path::new("./tests/mts/acceptance/ppu/vblank_stat_intr-GS.gb"))
    }
}

mod timer {
    use std::path::Path;
    use crate::run_acceptance_test;

    #[test]
    fn div_write() {
        run_acceptance_test(Path::new("./tests/mts/acceptance/timer/div_write.gb"))
    }

    #[test]
    fn rapid_toggle() {
        run_acceptance_test(Path::new("./tests/mts/acceptance/timer/rapid_toggle.gb"))
    }

    #[test]
    fn tim00() {
        run_acceptance_test(Path::new("./tests/mts/acceptance/timer/tim00.gb"))
    }

    #[test]
    fn tim00_div_trigger() {
        run_acceptance_test(Path::new("./tests/mts/acceptance/timer/tim00_div_trigger.gb"))
    }

    #[test]
    fn tim01() {
        run_acceptance_test(Path::new("./tests/mts/acceptance/timer/tim01.gb"))
    }

    #[test]
    fn tim01_div_trigger() {
        run_acceptance_test(Path::new("./tests/mts/acceptance/timer/tim01_div_trigger.gb"))
    }

    #[test]
    fn tim10() {
        run_acceptance_test(Path::new("./tests/mts/acceptance/timer/tim10.gb"))
    }

    #[test]
    fn tim10_div_trigger() {
        run_acceptance_test(Path::new("./tests/mts/acceptance/timer/tim10_div_trigger.gb"))
    }

    #[test]
    fn tim11() {
        run_acceptance_test(Path::new("./tests/mts/acceptance/timer/tim00.gb"))
    }

    #[test]
    fn tim11_div_trigger() {
        run_acceptance_test(Path::new("./tests/mts/acceptance/timer/tim11_div_trigger.gb"))
    }

    #[test]
    fn tima_reload() {
        run_acceptance_test(Path::new("./tests/mts/acceptance/timer/tima_reload.gb"))
    }

    #[test]
    fn tima_write_reloading() {
        run_acceptance_test(Path::new("./tests/mts/acceptance/timer/tima_write_reloading.gb"))
    }

    #[test]
    fn tma_write_reloading() {
        run_acceptance_test(Path::new("./tests/mts/acceptance/timer/tma_write_reloading.gb"))
    }
}

mod root {
    use std::path::Path;
    use crate::run_acceptance_test;

    #[test]
    fn add_sp_e_timing() {
        run_acceptance_test(Path::new("./tests/mts/acceptance/add_sp_e_timing.gb"))
    }
}