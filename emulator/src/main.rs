#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
//
// use crate::cartridge::load_cartridge;
// // use crate::emulator::Emulator;
// use crate::logging::init_log;
// use clap::Parser;
// use log::Level;
//
// mod cartridge;
// mod emulator;
// mod generated;
// mod gui;
// mod interrupts;
// mod joypad;
// mod logging;
// mod memory;
// mod serial;
// mod sound;
// mod timer;
// mod video;
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
//
// /// State per thread.
// struct ThreadState {
//     thread_nr: usize,
//     title: String,
//     name: String,
//     age: u32,
// }
//
// impl ThreadState {
//     fn new(thread_nr: usize) -> Self {
//         let title = format!("Background thread {thread_nr}");
//         Self {
//             thread_nr,
//             title,
//             name: "Arthur".into(),
//             age: 12 + thread_nr as u32 * 10,
//         }
//     }
//
//     fn show(&mut self, ctx: &egui::Context) {
//         let pos = egui::pos2(16.0, 128.0 * (self.thread_nr as f32 + 1.0));
//         egui::Window::new(&self.title)
//             .default_pos(pos)
//             .show(ctx, |ui| {
//                 ui.horizontal(|ui| {
//                     ui.label("Your name: ");
//                     ui.text_edit_singleline(&mut self.name);
//                 });
//                 ui.add(egui::Slider::new(&mut self.age, 0..=120).text("age"));
//                 if ui.button("Click each year").clicked() {
//                     self.age += 1;
//                 }
//                 ui.label(format!("Hello '{}', age {}", self.name, self.age));
//             });
//     }
// }
//
// fn new_worker(
//     thread_nr: usize,
//     on_done_tx: mpsc::SyncSender<()>,
// ) -> (JoinHandle<()>, mpsc::SyncSender<egui::Context>) {
//     let (show_tx, show_rc) = mpsc::sync_channel(0);
//     let handle = std::thread::Builder::new()
//         .name(format!("EguiPanelWorker {thread_nr}"))
//         .spawn(move || {
//             let mut state = ThreadState::new(thread_nr);
//             while let Ok(ctx) = show_rc.recv() {
//                 state.show(&ctx);
//                 let _ = on_done_tx.send(());
//             }
//         })
//         .expect("failed to spawn thread");
//     (handle, show_tx)
// }
//
// struct MyApp {
//     threads: Vec<(JoinHandle<()>, mpsc::SyncSender<egui::Context>)>,
//     on_done_tx: mpsc::SyncSender<()>,
//     on_done_rc: mpsc::Receiver<()>,
// }
//
// impl MyApp {
//     fn new() -> Self {
//         let threads = Vec::with_capacity(3);
//         let (on_done_tx, on_done_rc) = mpsc::sync_channel(0);
//
//         let mut slf = Self {
//             threads,
//             on_done_tx,
//             on_done_rc,
//         };
//
//         slf.spawn_thread();
//         slf.spawn_thread();
//
//         slf
//     }
//
//     fn spawn_thread(&mut self) {
//         let thread_nr = self.threads.len();
//         self.threads
//             .push(new_worker(thread_nr, self.on_done_tx.clone()));
//     }
// }
//
// impl std::ops::Drop for MyApp {
//     fn drop(&mut self) {
//         for (handle, show_tx) in self.threads.drain(..) {
//             std::mem::drop(show_tx);
//             handle.join().unwrap();
//         }
//     }
// }
//
// impl eframe::App for MyApp {
//     fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
//         egui::Window::new("Main thread").show(ctx, |ui| {
//             if ui.button("Spawn another thread").clicked() {
//                 self.spawn_thread();
//             }
//         });
//
//         for (_handle, show_tx) in &self.threads {
//             let _ = show_tx.send(ctx.clone());
//         }
//
//         for _ in 0..self.threads.len() {
//             let _ = self.on_done_rc.recv();
//         }
//     }
// }
use eframe::egui::{self, ColorImage};

fn main() -> Result<(), eframe::Error> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        // renderer: eframe::Renderer::Wgpu,
        ..Default::default()
    };
    eframe::run_native(
        "Take screenshots and display with eframe/egui",
        options,
        Box::new(|_cc| Box::<MyApp>::default()),
    )
}

#[derive(Default)]
struct MyApp {
    continuously_take_screenshots: bool,
    texture: Option<egui::TextureHandle>,
    screenshot: Option<ColorImage>,
    save_to_file: bool,
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            if let Some(screenshot) = self.screenshot.take() {
                self.texture = Some(ui.ctx().load_texture(
                    "screenshot",
                    screenshot,
                    Default::default(),
                ));
            }

            ui.horizontal(|ui| {
                ui.checkbox(
                    &mut self.continuously_take_screenshots,
                    "continuously take screenshots",
                );

                if ui.button("save to 'top_left.png'").clicked() {
                    self.save_to_file = true;
                    frame.request_screenshot();
                }

                ui.with_layout(egui::Layout::top_down(egui::Align::RIGHT), |ui| {
                    if self.continuously_take_screenshots {
                        if ui
                            .add(egui::Label::new("hover me!").sense(egui::Sense::hover()))
                            .hovered()
                        {
                            ctx.set_visuals(egui::Visuals::dark());
                        } else {
                            ctx.set_visuals(egui::Visuals::light());
                        };
                        frame.request_screenshot();
                    } else if ui.button("take screenshot!").clicked() {
                        frame.request_screenshot();
                    }
                });
            });
            //
            // if let Some(texture) = self.texture.as_ref() {
            //     ui.image((texture.id(), ui.available_size()));
            // } else {
                ui.spinner();
            // }

            ctx.request_repaint();
        });
    }

    fn post_rendering(&mut self, _window_size: [u32; 2], frame: &eframe::Frame) {
        if let Some(screenshot) = frame.screenshot() {
            if self.save_to_file {
                let pixels_per_point = frame.info().native_pixels_per_point;
                let region =
                    egui::Rect::from_two_pos(egui::Pos2::ZERO, egui::Pos2 { x: 100., y: 100. });
                let top_left_corner = screenshot.region(&region, pixels_per_point);
                // image::save_buffer(
                //     "top_left.png",
                //     top_left_corner.as_raw(),
                //     top_left_corner.width() as u32,
                //     top_left_corner.height() as u32,
                //     image::ColorType::Rgba8,
                // )
                //     .unwrap();
                self.save_to_file = false;
            }
            self.screenshot = Some(screenshot);
        }
    }
}