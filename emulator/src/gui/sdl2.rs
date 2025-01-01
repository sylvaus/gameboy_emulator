use crate::gui::Gui;
use crate::joypad::{InputProvider, JoypadState};
use crate::video::renderer::{Color, Screen, SCREEN_HEIGHT, SCREEN_WIDTH};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::PixelFormatEnum::ARGB8888;
use sdl2::rect::Rect;
use sdl2::render::{Texture, TextureCreator, WindowCanvas};
use sdl2::video::WindowContext;
use sdl2::{EventPump, Sdl};

const INIT_SCALE: u32 = 3;

const NB_BYTES_PER_PIXEL: usize = 4;
const SCREEN_WIDTH_BYTES: usize = (SCREEN_WIDTH as usize) * NB_BYTES_PER_PIXEL;
const NB_PIXELS: usize = ((SCREEN_WIDTH * SCREEN_HEIGHT) as usize) * NB_BYTES_PER_PIXEL;

pub struct Sdl2Gui<'a> {
    // TODO: split graphic and input into two sub structs
    // Graphic Fields
    screen: Sdl2Screen<'a>,

    // Input fields
    events: EventPump,
    joypad: JoypadState,
    quit_pressed: bool,
}

impl<'a> Sdl2Gui<'a> {
    pub fn new(canvas: &'a mut WindowCanvas, texture: Texture<'a>, events: EventPump) -> Self {
        Self {
            screen: Sdl2Screen::new(canvas, texture),
            events,
            joypad: JoypadState::default(),
            quit_pressed: false,
        }
    }
}

impl<'a> Gui for Sdl2Gui<'a> {}

impl<'a> Screen for Sdl2Gui<'a> {
    fn write_pixel(&mut self, x: usize, y: usize, color: &Color) {
        self.screen.write_pixel(x, y, color);
    }

    fn update_frame(&mut self) {
        self.screen.update();
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
        let keys: Vec<Keycode> = self
            .events
            .keyboard_state()
            .pressed_scancodes()
            .filter_map(Keycode::from_scancode)
            .collect();

        self.joypad.right = keys.contains(&Keycode::Right);
        self.joypad.left = keys.contains(&Keycode::Left);
        self.joypad.down = keys.contains(&Keycode::Down);
        self.joypad.up = keys.contains(&Keycode::Up);
        self.joypad.a = keys.contains(&Keycode::Q);
        self.joypad.b = keys.contains(&Keycode::W);
        self.joypad.select = keys.contains(&Keycode::RShift);
        self.joypad.start = keys.contains(&Keycode::Return);
    }

    fn get_inputs(&self) -> JoypadState {
        self.joypad.clone()
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

struct Sdl2Screen<'a> {
    canvas: &'a mut WindowCanvas,
    texture: Texture<'a>,
    pixels: [u8; NB_PIXELS],
    screen: Rect,
}

impl<'a> Sdl2Screen<'a> {
    pub fn new(canvas: &'a mut WindowCanvas, texture: Texture<'a>) -> Self {
        Self {
            canvas,
            texture,
            pixels: [0; NB_PIXELS],
            screen: Rect::new(0, 0, SCREEN_WIDTH, SCREEN_HEIGHT),
        }
    }

    fn write_pixel(&mut self, x: usize, y: usize, color: &Color) {
        let start_position = y * SCREEN_WIDTH_BYTES + x * NB_BYTES_PER_PIXEL;
        self.pixels[start_position] = color.blue;
        self.pixels[start_position + 1] = color.green;
        self.pixels[start_position + 2] = color.red;
        self.pixels[start_position + 3] = color.alpha;
    }

    fn update(&mut self) {
        self.canvas.clear();
        self.texture
            .update(self.screen, &self.pixels, SCREEN_WIDTH_BYTES)
            .unwrap();
        self.canvas
            .copy(&self.texture, None, Some(self.screen))
            .unwrap();
        self.canvas.present()
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
            .window(
                "Emulator",
                SCREEN_WIDTH * INIT_SCALE,
                SCREEN_HEIGHT * INIT_SCALE,
            )
            .position_centered()
            .build()
            .unwrap();

        let mut canvas = window.into_canvas().present_vsync().build().unwrap();
        // TODO: implement dynamic resizing.
        canvas
            .set_scale(INIT_SCALE as f32, INIT_SCALE as f32)
            .expect("Scaling should not fail");
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
