use crate::cartridge::load_cartridge;
use clap::Parser;

mod cartridge;
mod generated;
mod memory;
mod timer;
mod video;
mod joypad;
mod serial;
mod sound;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long)]
    rom_path: std::path::PathBuf,
}

fn main() {
    let args = Args::parse();

    let cartridge = load_cartridge(args.rom_path.as_path()).unwrap_or_else(|e| {
        panic!(
            "Could not read cartridge with path {:?}: {:?}",
            args.rom_path.as_path(),
            e
        )
    });

    println!("Cartridge: {}", cartridge);
}
