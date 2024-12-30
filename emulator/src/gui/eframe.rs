use crate::cartridge::load_cartridge;
use crate::emulator::ThreadedEmulator;
use crate::joypad::JoypadState;
use crate::video::renderer::{Color, Screen, SCREEN_HEIGHT, SCREEN_WIDTH};
use eframe::egui;
use eframe::egui::{Color32, ColorImage, Key, TextureOptions};
use std::path::PathBuf;
use std::sync::Arc;
use std::thread;
use std::thread::JoinHandle;

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

/// Enum containing all the possible dialog answers
///
/// This allows spawning the dialog in a new thread.
enum AppDialogAnswer {
    OpenFile(Option<PathBuf>),
}

#[derive()]
struct GBEmulatorApp {
    texture: Option<egui::TextureHandle>,
    emulator: ThreadedEmulator,
    dialog: Option<JoinHandle<AppDialogAnswer>>,
}

impl GBEmulatorApp {
    fn new() -> Self {
        Self {
            texture: Default::default(),
            emulator: ThreadedEmulator::new(),
            dialog: None,
        }
    }
}

impl eframe::App for GBEmulatorApp {
    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        if self
            .dialog
            .as_ref()
            .map_or(false, |handle| handle.is_finished())
        {
            let handle = self.dialog.take().unwrap();
            self.handle_dialog_answer(handle);
        }

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
            if self.is_dialog_open() {
                ui.disable();
            }

            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| self.menu_file_buttons(ui));
                ui.add_space(16.0);

                egui::widgets::global_theme_preference_buttons(ui);
            });
        });
    }
    fn add_central_panel(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            if self.is_dialog_open() {
                ui.disable();
            }

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
                if let (Some(handle), Ok(cartridge)) =
                    (&self.texture, load_cartridge(path.as_path()))
                {
                    self.emulator
                        .start(cartridge, Box::new(AppScreen::new(handle.clone())));
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
            Color32::from_rgba_unmultiplied(color.red, color.green, color.blue, color.alpha);
    }

    fn update_frame(&mut self) {
        self.texture.set(self.image.clone(), TextureOptions::NEAREST);
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
    Arc::new(ColorImage::new(
        [SCREEN_WIDTH as usize, SCREEN_HEIGHT as usize],
        Color32::BLACK,
    ))
}
