#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use crate::gui::eframe::run_emulator;

mod cartridge;
mod emulator;
mod generated;
mod gui;
mod interrupts;
mod joypad;
mod memory;
mod serial;
mod sound;
mod timer;
mod video;
mod throttler;
mod statistics;

fn main() -> Result<(), eframe::Error> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    run_emulator()
}

