#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::path::PathBuf;
use std::sync::Arc;
use std::thread;
use std::thread::JoinHandle;


// use crate::cartridge::load_cartridge;
// // use crate::emulator::Emulator;
// use crate::logging::init_log;
// use clap::Parser;
// use log::Level;
//
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
//
// use std::sync::mpsc;
// use std::thread::JoinHandle;
//
// use eframe::egui;
//
// #[derive(Parser, Debug)]
// #[command(author, version, about, long_about = None)]
// struct Args {
//     #[arg(long)]
//     rom_path: std::path::PathBuf,
// }
//
// fn main() {
//     env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
//     let options = eframe::NativeOptions {
//         initial_window_size: Some(egui::vec2(1024.0, 768.0)),
//         ..Default::default()
//     };
//     eframe::run_native(
//         "My parallel egui App",
//         options,
//         Box::new(|_cc| Box::new(MyApp::new())),
//     );
//     let args = Args::parse();
//
//     // init_log(Level::Info);
//     // let mut gui = Sdl2GuiProvider::new();
//     //
//     // let cartridge = load_cartridge(args.rom_path.as_path()).unwrap_or_else(|e| {
//     //     panic!(
//     //         "Could not read cartridge with path {:?}: {:?}",
//     //         args.rom_path.as_path(),
//     //         e
//     //     )
//     // });
//     //
//     // println!("Cartridge: {}", cartridge);
//     //
//     // let mut emulator = Emulator::new(cartridge, gui.get());
//     // emulator.run();
// }

use crate::cartridge::load_cartridge;
use crate::emulator::ThreadedEmulator;
use crate::video::renderer::{Color, Screen, SCREEN_HEIGHT, SCREEN_WIDTH};
use eframe::egui::{self, Color32, ColorImage};

fn main() -> Result<(), eframe::Error> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        ..Default::default()
    };
    eframe::run_native(
        "Take screenshots and display with eframe/egui",
        options,
        Box::new(|_| Box::new(MyApp::new())),
    )
}

/// Enum containing all the possible dialog answers
///
/// This allows spawning the dialog in a new thread.
enum AppDialogAnswer {
    OpenFile(Option<PathBuf>),
}

#[derive()]
struct MyApp {
    texture: Option<egui::TextureHandle>,
    emulator: ThreadedEmulator,
    dialog: Option<JoinHandle<AppDialogAnswer>>,
}

impl MyApp {
    fn new() -> Self {
        Self {
            texture: Default::default(),
            emulator: ThreadedEmulator::new(),
            dialog: None,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        if self
            .dialog
            .as_ref()
            .map_or(false, |handle| handle.is_finished())
        {
            let handle = self.dialog.take().unwrap();
            self.handle_dialog_answer(handle);
        }

        self.add_top_panel(ctx);
        self.add_central_panel(ctx);
    }

    fn post_rendering(&mut self, _window_size: [u32; 2], frame: &eframe::Frame) {
        // TODO: remove if not useful
    }
}

impl MyApp {
    fn add_top_panel(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.set_enabled(!self.is_dialog_open());

            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| self.menu_file_buttons(ui));
                ui.add_space(16.0);

                egui::widgets::global_dark_light_mode_buttons(ui);
            });
        });
    }
    fn add_central_panel(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.set_enabled(!self.is_dialog_open());

            self.texture.get_or_insert_with(|| {
                ui.ctx()
                    .load_texture("gb_image", default_gb_image(), Default::default())
            });

            if let Some(texture) = self.texture.as_ref() {
                ui.image((texture.id(), ui.available_size()));
            }

            // TODO: Maybe only request repaint when the image changed
            ctx.request_repaint();
        });
    }

    fn menu_file_buttons(&mut self, ui: &mut egui::Ui) {
        if ui.button("Open Rom").clicked() && !self.dialog.is_some() {
            self.dialog = Some(thread::spawn(|| {
                let answer = rfd::FileDialog::new()
                    .set_title("Select ROM to Play")
                    .add_filter("ROM", &["gb", "gbc", "gba"])
                    .pick_file();
                AppDialogAnswer::OpenFile(answer)
            }));
        }
    }

    fn is_dialog_open(&self) -> bool {
        self.dialog.is_some()
    }

    fn handle_dialog_answer(&mut self, handle: JoinHandle<AppDialogAnswer>) {
        match handle.join() {
            Ok(AppDialogAnswer::OpenFile(Some(path))) => {
                if let (Some(handle), Ok(cartridge)) = (&self.texture, load_cartridge(path.as_path())) {
                    self.emulator.start(cartridge, Box::new(AppScreen::new(handle.clone())));
                }
            }
            _ => {}
        };
    }
}

struct AppScreen {
    image: Arc<ColorImage>,
    texture: egui::TextureHandle,
}

impl Screen for AppScreen {
    fn write_pixel(&mut self, x: usize, y: usize, color: &Color) {
        Arc::make_mut(&mut self.image)[(x, y)] =
            Color32::from_rgba_premultiplied(color.red, color.green, color.blue, color.alpha);
    }

    fn update_frame(&mut self) {
        self.texture.set(self.image.clone(), Default::default());
    }
}

impl AppScreen {
    fn new(texture: egui::TextureHandle) -> Self {
        Self {
            image: default_gb_image(),
            texture
        }
    }
}

fn default_gb_image() -> Arc<ColorImage> {
    Arc::new(ColorImage::new(
        [SCREEN_WIDTH as usize, SCREEN_HEIGHT as usize],
        Color32::BLACK,
    ))
}
