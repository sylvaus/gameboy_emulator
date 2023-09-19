use crate::cartridge::load_cartridge;
use crate::emulator::Emulator;
use crate::gui::sdl2::get_sdl2_gui;
use crate::logging::init_log;
use clap::Parser;
use log::Level;

mod cartridge;
mod emulator;
mod generated;
mod gui;
mod interrupts;
mod joypad;
mod logging;
mod memory;
mod serial;
mod sound;
mod timer;
mod video;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long)]
    rom_path: std::path::PathBuf,
}

fn main() {
    let args = Args::parse();

    init_log(Level::Info);
    let (video, input, context) = get_sdl2_gui();

    let cartridge = load_cartridge(args.rom_path.as_path()).unwrap_or_else(|e| {
        panic!(
            "Could not read cartridge with path {:?}: {:?}",
            args.rom_path.as_path(),
            e
        )
    });

    println!("Cartridge: {}", cartridge);

    let mut emulator = Emulator::new(cartridge, video, input);
    emulator.run();
}
