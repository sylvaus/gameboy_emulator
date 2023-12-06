use crate::joypad::InputProvider;
use crate::video::renderer::Screen;

#[cfg(feature = "sdl2-ui")]
pub(crate) mod sdl2;
mod eframe;

pub trait Gui: Screen + InputProvider {}
