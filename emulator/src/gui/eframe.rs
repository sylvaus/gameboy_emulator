use crate::cartridge::load_cartridge;
use crate::emulator::ThreadedEmulator;
use crate::joypad::JoypadState;
use crate::video::renderer::{Color, Screen, SCREEN_HEIGHT, SCREEN_WIDTH};
use eframe::egui;
use eframe::egui::{Button, Color32, ColorImage, Key, TextureOptions};
use std::sync::Arc;
use crate::data_completion::CompletionStatus;

pub fn run_emulator() -> eframe::Result {
    let options = eframe::NativeOptions {
        ..Default::default()
    };
    eframe::run_native(
        "GameBoy emulator",
        options,
        Box::new(|_| Ok(Box::new(GBEmulatorApp::new()))),
    )
}

#[derive()]
struct GBEmulatorApp {
    texture: Option<egui::TextureHandle>,
    emulator: ThreadedEmulator,
}

impl GBEmulatorApp {
    fn new() -> Self {
        Self {
            texture: Default::default(),
            emulator: ThreadedEmulator::new(),
        }
    }
}

impl eframe::App for GBEmulatorApp {
    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        ctx.input(|state| {
            let joypad = JoypadState {
                right: state.key_down(Key::ArrowRight),
                left: state.key_down(Key::ArrowLeft),
                up: state.key_down(Key::ArrowUp),
                down: state.key_down(Key::ArrowDown),
                a: state.key_down(Key::A),
                b: state.key_down(Key::Z),
                select: state.modifiers.shift,
                start: state.key_down(Key::Enter),
            };
            self.emulator.update_inputs(joypad);
        });

        self.add_top_panel(ctx);
        self.add_central_panel(ctx);
    }
}

impl GBEmulatorApp {
    fn add_top_panel(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::MenuBar::new().ui(ui, |ui| {
                ui.menu_button("File", |ui| self.menu_file_buttons(ui));
                ui.add_space(16.0);

                egui::widgets::global_theme_preference_buttons(ui);
            });
        });
    }
    fn add_central_panel(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            self.texture.get_or_insert_with(|| {
                ui.ctx()
                    .load_texture("gb_image", default_gb_image(), TextureOptions::NEAREST)
            });

            if let Some(texture) = self.texture.as_ref() {
                ui.image((texture.id(), ui.available_size()));
            }

            // TODO: Maybe only request repaint when the image changed
            ctx.request_repaint();
        });
    }

    fn menu_file_buttons(&mut self, ui: &mut egui::Ui) {
        if ui.button("Open Rom").clicked() {
            self.open_rom();
        }

        if ui.add_enabled(self.emulator.is_running(), Button::new("Load State File")).clicked() {
            self.load_save_file();
        }

        if ui.add_enabled(self.emulator.is_running(), Button::new("Save State File")).clicked() {
            self.save_save_file();
        }

        if ui.button("Quit").clicked() {
            self.quit();
        }
    }

    fn open_rom(&mut self) {
        let answer = rfd::FileDialog::new()
            .set_title("Select ROM to Play")
            .add_filter("ROM", &["gb", "gbc", "gba"])
            .pick_file();

        if let Some(path) = answer {
            if let (Some(handle), Ok(cartridge)) = (&self.texture, load_cartridge(path.as_path())) {
                self.emulator
                    .start(cartridge, Box::new(AppScreen::new(handle.clone())));
            }
        }
    }

    fn load_save_file(&mut self) {
        let answer = rfd::FileDialog::new()
            .set_title("Select State File")
            .add_filter("State File", &["save"])
            .pick_file();

        let status = if let Some(path) = answer {
            self.emulator.load_state(path)
        } else {
            return;
        };

        match status {
            CompletionStatus::Success => {}
            CompletionStatus::Failure(status) => {
                rfd::MessageDialog::new()
                    .set_title("Error Loading State File")
                    .set_description(status)
                    .show();
            }
            CompletionStatus::Timeout => {
                rfd::MessageDialog::new()
                    .set_title("Error Loading State File")
                    .set_description("State file loading operation timed out")
                    .show();
            }
        }
    }

    fn save_save_file(&mut self) {
        let answer = rfd::FileDialog::new()
            .set_title("Select State File")
            .add_filter("State File", &["save"])
            .save_file();

        let status = if let Some(path) = answer {
            self.emulator.save_state(path)
        } else {
            return;
        };

        match status {
            CompletionStatus::Success => {}
            CompletionStatus::Failure(status) => {
                rfd::MessageDialog::new()
                    .set_title("Error Saving State File")
                    .set_description(status)
                    .show();
            }
            CompletionStatus::Timeout => {
                rfd::MessageDialog::new()
                    .set_title("Error Saving State File")
                    .set_description("State file saving operation timed out")
                    .show();
            }
        }

    }

    fn quit(&mut self) {
        // TODO: handle exit more gracefully
        self.emulator.stop();
        std::process::exit(0);
    }
}

struct AppScreen {
    image: Arc<ColorImage>,
    texture: egui::TextureHandle,
}

impl Screen for AppScreen {
    fn write_pixel(&mut self, x: usize, y: usize, color: &Color) {
        Arc::make_mut(&mut self.image)[(x, y)] =
            Color32::from_rgba_unmultiplied(color.red, color.green, color.blue, color.alpha);
    }

    fn update_frame(&mut self) {
        self.texture
            .set(self.image.clone(), TextureOptions::NEAREST);
    }
}

impl AppScreen {
    fn new(texture: egui::TextureHandle) -> Self {
        Self {
            image: default_gb_image(),
            texture,
        }
    }
}

fn default_gb_image() -> Arc<ColorImage> {
    Arc::new(ColorImage::filled(
        [SCREEN_WIDTH as usize, SCREEN_HEIGHT as usize],
        Color32::BLACK,
    ))
}
