#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use emulator::gui::eframe::run_emulator;

fn main() -> Result<(), eframe::Error> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    run_emulator()
}

