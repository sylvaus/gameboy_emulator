use crate::gui::Gui;
use crate::joypad::{InputProvider, JoypadInput};
use crate::video::controller::VideoController;
use crate::video::renderer::{VideoRenderer, SCREEN_HEIGHT, SCREEN_WIDTH};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::PixelFormatEnum::ARGB8888;
use sdl2::rect::Rect;
use sdl2::render::{Texture, TextureCreator, WindowCanvas};
use sdl2::video::WindowContext;
use sdl2::{EventPump, Sdl};

const NB_BYTES_PER_PIXEL: usize = 4;
const SCREEN_WIDTH_BYTES: usize = (SCREEN_WIDTH as usize) * NB_BYTES_PER_PIXEL;
const NB_PIXELS: usize = ((SCREEN_WIDTH * SCREEN_HEIGHT) as usize) * NB_BYTES_PER_PIXEL;

struct Color {
    pub alpha: u8,
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

pub struct Sdl2Gui<'a> {
    // TODO: split graphic and input into two sub structs
    // Graphic Fields
    canvas: &'a mut WindowCanvas,
    texture: Texture<'a>,
    pixels: [u8; NB_PIXELS],

    // Input fields
    events: EventPump,
    quit_pressed: bool,
}

impl<'a> Sdl2Gui<'a> {
    pub fn new(canvas: &'a mut WindowCanvas, texture: Texture<'a>, events: EventPump) -> Self {
        Self {
            canvas,
            texture,
            pixels: [0; NB_PIXELS],
            events,
            quit_pressed: false,
        }
    }

    fn render_background_window(&mut self, video: &VideoController) -> [u8; SCREEN_WIDTH as usize] {
        // TODO: implement
        [0; SCREEN_WIDTH as usize]
    }

    fn render_sprites(&mut self, video: &VideoController) {
        // TODO: implement
    }

    fn write_pixel(&mut self, x: usize, y: usize, color: &Color) {
        let start_position = y * SCREEN_WIDTH_BYTES + x * NB_BYTES_PER_PIXEL;
        self.pixels[start_position] = color.blue;
        self.pixels[start_position + 1] = color.green;
        self.pixels[start_position + 2] = color.red;
        self.pixels[start_position + 3] = color.alpha;
    }
}

impl<'a> Gui for Sdl2Gui<'a> {}

fn create_screen_rect() -> Rect {
    Rect::new(0, 0, SCREEN_WIDTH, SCREEN_HEIGHT)
}

impl<'a> VideoRenderer for Sdl2Gui<'a> {
    fn scanline(&mut self, video: &VideoController) {
        if video.control.read_lcd_enable() == 0 {
            return;
        }

        if video.control.read_window_enable() != 0 {
            self.render_background_window(video);
        }
        if video.control.read_obj_enable() != 0 {
            self.render_sprites(video);
        }
    }

    fn update_frame(&mut self) {
        self.canvas.clear();

        let screen = create_screen_rect();
        self.texture
            .update(screen, &self.pixels, SCREEN_WIDTH_BYTES)
            .unwrap();
        self.canvas.copy(&self.texture, None, Some(screen)).unwrap();
        self.canvas.present()
    }
}

impl<'a> InputProvider for Sdl2Gui<'a> {
    fn update_inputs(&mut self) {
        for event in self.events.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    self.quit_pressed = true;
                    break;
                }
                _ => {}
            }
        }
    }

    fn set_inputs(&self, input: &mut JoypadInput) -> bool {
        // TODO: implement
        false
    }

    fn is_quit_pressed(&self) -> bool {
        self.quit_pressed
    }
}

pub struct Sdl2Context {
    pub context: Sdl,
}

impl Sdl2Context {
    pub fn new(context: Sdl) -> Self {
        Self { context }
    }
}

pub struct Sdl2GuiProvider {
    context: Sdl,
    canvas: WindowCanvas,
    texture_creator: TextureCreator<WindowContext>,
}

impl Sdl2GuiProvider {
    pub fn new() -> Self {
        let context = sdl2::init().unwrap();
        let video_subsystem = context.video().unwrap();

        let window = video_subsystem
            .window("Emulator", SCREEN_WIDTH, SCREEN_HEIGHT)
            .position_centered()
            .build()
            .unwrap();

        let canvas = window.into_canvas().build().unwrap();
        let texture_creator = canvas.texture_creator();
        Self {
            context,
            canvas,
            texture_creator,
        }
    }

    pub fn get(&mut self) -> Sdl2Gui {
        let texture = self
            .texture_creator
            .create_texture_streaming(ARGB8888, SCREEN_WIDTH, SCREEN_HEIGHT)
            .unwrap();
        Sdl2Gui::new(
            &mut self.canvas,
            texture,
            self.context.event_pump().unwrap(),
        )
    }
}
