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

    assert!(
        debugger.completed && is_success(&state),
        "Failed acceptance test: {:?}",
        path
    );
}

// Complete
mod acceptance {
    use crate::run_acceptance_test;
    use std::path::Path;

    #[test]
    fn mem_oam() {
        run_acceptance_test(Path::new("./tests/mts/acceptance/bits/mem_oam.gb"));
    }

    #[test]
    fn reg_f() {
        run_acceptance_test(Path::new("./tests/mts/acceptance/bits/reg_f.gb"));
    }

    #[test]
    fn unused_hwio_gs() {
        run_acceptance_test(Path::new("./tests/mts/acceptance/bits/unused_hwio-GS.gb"));
    }
}

// Complete
mod instr {
    use crate::run_acceptance_test;
    use std::path::Path;

    #[test]
    fn instr_daa() {
        run_acceptance_test(Path::new("./tests/mts/acceptance/instr/daa.gb"));
    }
}

// Complete
mod interrupts {
    use crate::run_acceptance_test;
    use std::path::Path;
    #[test]
    fn ie_push() {
        run_acceptance_test(Path::new("./tests/mts/acceptance/interrupts/ie_push.gb"));
    }
}

mod oam_dma {
    use crate::run_acceptance_test;
    use std::path::Path;

    #[test]
    fn basic() {
        run_acceptance_test(Path::new("./tests/mts/acceptance/oam_dma/basic.gb"))
    }

    #[test]
    fn reg_read() {
        run_acceptance_test(Path::new("./tests/mts/acceptance/oam_dma/reg_read.gb"))
    }

    // TODO: fix
    // Test currently not passing but tested on PyBoy, and it also does not pass.
    #[ignore]
    #[test]
    fn sources_gs() {
        run_acceptance_test(Path::new("./tests/mts/acceptance/oam_dma/sources-GS.gb"))
    }
}

mod ppu {
    use crate::run_acceptance_test;
    use std::path::Path;

    #[ignore] // TODO: fix
    #[test]
    fn hblank_ly_scx_timing_gs() {
        run_acceptance_test(Path::new(
            "./tests/mts/acceptance/ppu/hblank_ly_scx_timing-GS.gb",
        ))
    }

    #[test]
    fn intr_1_2_timing_gs() {
        run_acceptance_test(Path::new(
            "./tests/mts/acceptance/ppu/intr_1_2_timing-GS.gb",
        ))
    }

    #[ignore] // TODO: fix
    #[test]
    fn intr_2_0_timing() {
        run_acceptance_test(Path::new("./tests/mts/acceptance/ppu/intr_2_0_timing.gb"))
    }

    #[ignore] // TODO: fix
    #[test]
    fn intr_2_mode0_timing() {
        run_acceptance_test(Path::new(
            "./tests/mts/acceptance/ppu/intr_2_mode0_timing.gb",
        ))
    }

    #[ignore] // TODO: fix
    #[test]
    fn intr_2_mode0_timing_sprites() {
        run_acceptance_test(Path::new(
            "./tests/mts/acceptance/ppu/intr_2_mode0_timing_sprites.gb",
        ))
    }

    #[ignore] // TODO: fix
    #[test]
    fn intr_2_mode3_timing() {
        run_acceptance_test(Path::new(
            "./tests/mts/acceptance/ppu/intr_2_mode3_timing.gb.gb",
        ))
    }

    #[ignore] // TODO: fix
    #[test]
    fn intr_2_oam_ok_timing() {
        run_acceptance_test(Path::new(
            "./tests/mts/acceptance/ppu/intr_2_oam_ok_timing.gb",
        ))
    }

    #[ignore] // TODO: fix
    #[test]
    fn lcdon_timing_gs() {
        run_acceptance_test(Path::new("./tests/mts/acceptance/ppu/lcdon_timing-GS.gb"))
    }

    #[ignore] // TODO: fix
    #[test]
    fn lcdon_write_timing_gs() {
        run_acceptance_test(Path::new(
            "./tests/mts/acceptance/ppu/lcdon_write_timing-GS.gb",
        ))
    }

    #[ignore] // TODO: fix
    #[test]
    fn stat_irq_blocking() {
        run_acceptance_test(Path::new("./tests/mts/acceptance/ppu/stat_irq_blocking.gb"))
    }

    #[ignore] // TODO: fix
    #[test]
    fn stat_lyc_onoff() {
        run_acceptance_test(Path::new("./tests/mts/acceptance/ppu/stat_lyc_onoff.gb"))
    }

    #[ignore] // TODO: fix
    #[test]
    fn vblank_stat_intr_gs() {
        run_acceptance_test(Path::new(
            "./tests/mts/acceptance/ppu/vblank_stat_intr-GS.gb",
        ))
    }
}

mod timer {
    use crate::run_acceptance_test;
    use std::path::Path;

    #[test]
    fn div_write() {
        run_acceptance_test(Path::new("./tests/mts/acceptance/timer/div_write.gb"))
    }

    #[ignore] // TODO: fix
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
        run_acceptance_test(Path::new(
            "./tests/mts/acceptance/timer/tim00_div_trigger.gb",
        ))
    }

    #[test]
    fn tim01() {
        run_acceptance_test(Path::new("./tests/mts/acceptance/timer/tim01.gb"))
    }

    #[test]
    fn tim01_div_trigger() {
        run_acceptance_test(Path::new(
            "./tests/mts/acceptance/timer/tim01_div_trigger.gb",
        ))
    }

    #[test]
    fn tim10() {
        run_acceptance_test(Path::new("./tests/mts/acceptance/timer/tim10.gb"))
    }

    #[test]
    fn tim10_div_trigger() {
        run_acceptance_test(Path::new(
            "./tests/mts/acceptance/timer/tim10_div_trigger.gb",
        ))
    }

    #[test]
    fn tim11() {
        run_acceptance_test(Path::new("./tests/mts/acceptance/timer/tim11.gb"))
    }

    #[test]
    fn tim11_div_trigger() {
        run_acceptance_test(Path::new(
            "./tests/mts/acceptance/timer/tim11_div_trigger.gb",
        ))
    }

    #[ignore] // TODO: fix
    #[test]
    fn tima_reload() {
        run_acceptance_test(Path::new("./tests/mts/acceptance/timer/tima_reload.gb"))
    }

    #[ignore] // TODO: fix
    #[test]
    fn tima_write_reloading() {
        run_acceptance_test(Path::new(
            "./tests/mts/acceptance/timer/tima_write_reloading.gb",
        ))
    }

    #[ignore] // TODO: fix
    #[test]
    fn tma_write_reloading() {
        run_acceptance_test(Path::new(
            "./tests/mts/acceptance/timer/tma_write_reloading.gb",
        ))
    }
}

mod serial {
    use crate::run_acceptance_test;
    use std::path::Path;

    #[ignore] // The serial part is not implemented.
    #[test]
    fn boot_sclk_align_dmgabcmgb() {
        run_acceptance_test(Path::new(
            "./tests/mts/acceptance/serial/boot_sclk_align-dmgABCmgb.gb",
        ))
    }
}

mod root {
    use crate::run_acceptance_test;
    use std::path::Path;

    #[ignore] // TODO: fix
    #[test]
    fn add_sp_e_timing() {
        run_acceptance_test(Path::new("./tests/mts/acceptance/add_sp_e_timing.gb"))
    }

    #[ignore] // TODO: fix
    #[test]
    fn boot_div2_s() {
        run_acceptance_test(Path::new("./tests/mts/acceptance/boot_div2-S.gb"))
    }

    #[ignore] // TODO: fix
    #[test]
    fn boot_div_dmg0() {
        run_acceptance_test(Path::new("./tests/mts/acceptance/boot_div-dmg0.gb"))
    }

    #[ignore] // TODO: fix
    #[test]
    fn boot_div_dmgabcmgb() {
        run_acceptance_test(Path::new("./tests/mts/acceptance/boot_div-dmgABCmgb.gb"))
    }

    #[ignore] // TODO: fix
    #[test]
    fn boot_div_s() {
        run_acceptance_test(Path::new("./tests/mts/acceptance/boot_div-S.gb"))
    }

    #[ignore] // TODO: fix
    #[test]
    fn boot_hwio_dmg0() {
        run_acceptance_test(Path::new("./tests/mts/acceptance/boot_hwio-dmg0.gb"))
    }

    #[ignore] // TODO: fix
    #[test]
    fn boot_hwio_dmgabcmgb() {
        run_acceptance_test(Path::new("./tests/mts/acceptance/boot_hwio-dmgABCmgb.gb"))
    }

    #[ignore] // TODO: fix
    #[test]
    fn boot_hwio_s() {
        run_acceptance_test(Path::new("./tests/mts/acceptance/boot_hwio-S.gb"))
    }

    #[ignore] // TODO: fix
    #[test]
    fn boot_regs_dmg0() {
        run_acceptance_test(Path::new("./tests/mts/acceptance/boot_regs-dmg0.gb"))
    }

    #[ignore] // TODO: fix
    #[test]
    fn boot_regs_dmgabc() {
        run_acceptance_test(Path::new("./tests/mts/acceptance/boot_regs-dmgABC.gb"))
    }

    #[ignore] // TODO: fix
    #[test]
    fn boot_regs_mgb() {
        run_acceptance_test(Path::new("./tests/mts/acceptance/boot_regs-mgb.gb"))
    }

    #[ignore] // TODO: fix
    #[test]
    fn boot_regs_sgb() {
        run_acceptance_test(Path::new("./tests/mts/acceptance/boot_regs-sgb.gb"))
    }

    #[ignore] // TODO: fix
    #[test]
    fn boot_regs_sgb2() {
        run_acceptance_test(Path::new("./tests/mts/acceptance/boot_regs-sgb2.gb"))
    }

    #[ignore] // TODO: fix
    #[test]
    fn call_cc_timing() {
        run_acceptance_test(Path::new("./tests/mts/acceptance/call_cc_timing.gb"))
    }

    #[ignore] // TODO: fix
    #[test]
    fn call_cc_timing2() {
        run_acceptance_test(Path::new("./tests/mts/acceptance/call_cc_timing2.gb"))
    }

    #[ignore] // TODO: fix
    #[test]
    fn call_timing() {
        run_acceptance_test(Path::new("./tests/mts/acceptance/call_timing.gb"))
    }

    #[ignore] // TODO: fix
    #[test]
    fn call_timing2() {
        run_acceptance_test(Path::new("./tests/mts/acceptance/call_timing2.gb"))
    }

    #[test]
    fn di_timing_gs() {
        run_acceptance_test(Path::new("./tests/mts/acceptance/di_timing-GS.gb"))
    }

    #[test]
    fn div_timing() {
        run_acceptance_test(Path::new("./tests/mts/acceptance/div_timing.gb"))
    }

    #[ignore] // TODO: fix
    #[test]
    fn ei_sequence() {
        run_acceptance_test(Path::new("./tests/mts/acceptance/ei_sequence.gb"))
    }

    #[ignore] // TODO: fix
    #[test]
    fn ei_timing() {
        run_acceptance_test(Path::new("./tests/mts/acceptance/ei_timing.gb"))
    }

    #[test]
    fn halt_ime0_ei() {
        run_acceptance_test(Path::new("./tests/mts/acceptance/halt_ime0_ei.gb"))
    }

    #[test]
    fn halt_ime0_nointr_timing() {
        run_acceptance_test(Path::new(
            "./tests/mts/acceptance/halt_ime0_nointr_timing.gb",
        ))
    }

    #[test]
    fn halt_ime1_timing() {
        run_acceptance_test(Path::new("./tests/mts/acceptance/halt_ime1_timing.gb"))
    }

    #[test]
    fn halt_ime1_timing2_gs() {
        run_acceptance_test(Path::new("./tests/mts/acceptance/halt_ime1_timing2-GS.gb"))
    }

    #[test]
    fn if_ie_registers() {
        run_acceptance_test(Path::new("./tests/mts/acceptance/if_ie_registers.gb"))
    }

    #[test]
    fn intr_timing() {
        run_acceptance_test(Path::new("./tests/mts/acceptance/intr_timing.gb"))
    }

    #[ignore] // TODO: fix
    #[test]
    fn jp_cc_timing() {
        run_acceptance_test(Path::new("./tests/mts/acceptance/jp_cc_timing.gb"))
    }

    #[ignore] // TODO: fix
    #[test]
    fn jp_timing() {
        run_acceptance_test(Path::new("./tests/mts/acceptance/jp_timing.gb"))
    }

    #[ignore] // TODO: fix
    #[test]
    fn ld_hl_sp_e_timing() {
        run_acceptance_test(Path::new("./tests/mts/acceptance/ld_hl_sp_e_timing.gb"))
    }

    #[ignore] // TODO: fix
    #[test]
    fn oam_dma_restart() {
        run_acceptance_test(Path::new("./tests/mts/acceptance/oam_dma_restart.gb"))
    }

    #[ignore] // TODO: fix
    #[test]
    fn oam_dma_start() {
        run_acceptance_test(Path::new("./tests/mts/acceptance/oam_dma_start.gb"))
    }

    #[ignore] // TODO: fix
    #[test]
    fn oam_dma_timing() {
        run_acceptance_test(Path::new("./tests/mts/acceptance/oam_dma_timing.gb"))
    }

    #[ignore] // TODO: fix
    #[test]
    fn pop_timing() {
        run_acceptance_test(Path::new("./tests/mts/acceptance/pop_timing.gb"))
    }

    #[ignore] // TODO: fix
    #[test]
    fn push_timing() {
        run_acceptance_test(Path::new("./tests/mts/acceptance/push_timing.gb"))
    }

    #[ignore] // TODO: fix
    #[test]
    fn rapid_di_ei() {
        run_acceptance_test(Path::new("./tests/mts/acceptance/rapid_di_ei.gb"))
    }

    #[ignore] // TODO: fix
    #[test]
    fn reti_intr_timing() {
        run_acceptance_test(Path::new("./tests/mts/acceptance/reti_intr_timing.gb"))
    }

    #[ignore] // TODO: fix
    #[test]
    fn reti_timing() {
        run_acceptance_test(Path::new("./tests/mts/acceptance/reti_timing.gb"))
    }

    #[ignore] // TODO: fix
    #[test]
    fn ret_cc_timing() {
        run_acceptance_test(Path::new("./tests/mts/acceptance/ret_cc_timing.gb"))
    }

    #[ignore] // TODO: fix
    #[test]
    fn ret_timing() {
        run_acceptance_test(Path::new("./tests/mts/acceptance/ret_timing.gb"))
    }

    #[ignore] // TODO: fix
    #[test]
    fn rst_timing() {
        run_acceptance_test(Path::new("./tests/mts/acceptance/rst_timing.gb"))
    }
}
