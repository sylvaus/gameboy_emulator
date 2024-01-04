use crate::joypad::InputProvider;
use crate::video::renderer::Screen;

mod eframe;
#[cfg(feature = "sdl2-ui")]
pub(crate) mod sdl2;

pub trait Gui: Screen + InputProvider {}
