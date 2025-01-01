use crate::joypad::{InputProvider, JoypadState};
use crate::video::renderer::{Color, Screen};

pub mod eframe;
#[cfg(feature = "sdl2-ui")]
pub(crate) mod sdl2;

pub trait Gui: Screen + InputProvider {}

pub struct NoOpGui {}

impl NoOpGui {
    pub fn new() -> Self {
        Self {}
    }
}

impl Screen for NoOpGui {
    fn write_pixel(&mut self, _x: usize, _y: usize, _color: &Color) {

    }

    fn update_frame(&mut self) {

    }
}


impl InputProvider for NoOpGui {
    fn update_inputs(&mut self) {
    }

    fn get_inputs(&self) -> JoypadState {
        JoypadState::default()
    }
}

impl Gui for NoOpGui {}