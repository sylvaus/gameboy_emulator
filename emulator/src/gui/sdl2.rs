use crate::joypad::{InputProvider, JoypadInput};
use crate::video::controller::VideoController;
use crate::video::renderer::{VideoRenderer, SCREEN_HEIGHT, SCREEN_WIDTH};
use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;
use sdl2::{EventPump, Sdl};

pub struct Sdl2VideoRenderer {
    canvas: WindowCanvas,
}

impl Sdl2VideoRenderer {
    pub fn new(canvas: WindowCanvas) -> Self {
        Self { canvas }
    }
}

impl VideoRenderer for Sdl2VideoRenderer {
    fn render(&mut self, video: &VideoController) {
        todo!()
    }
}

pub struct Sdl2InputProvider {
    events: EventPump,
}

impl Sdl2InputProvider {
    pub fn new(events: EventPump) -> Self {
        Self { events }
    }
}

impl InputProvider for Sdl2InputProvider {
    fn update(&mut self) {
        todo!()
    }

    fn set_inputs(&self, input: &mut JoypadInput) {
        todo!()
    }

    fn is_quit_pressed(&self) -> bool {
        todo!()
    }
}

pub struct Sdl2Context {
    context: Sdl,
}

impl Sdl2Context {
    pub fn new(context: Sdl) -> Self {
        Self { context }
    }
}

pub fn get_sdl2_gui() -> (Sdl2VideoRenderer, Sdl2InputProvider, Sdl2Context) {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("Emulator", SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    // Show a black screen at startup.
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();

    (
        Sdl2VideoRenderer::new(canvas),
        Sdl2InputProvider::new(sdl_context.event_pump().unwrap()),
        Sdl2Context::new(sdl_context),
    )
}
