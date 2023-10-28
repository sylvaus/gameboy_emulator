use crate::joypad::InputProvider;
use crate::video::renderer::Screen;

pub(crate) mod sdl2;
pub trait Gui: Screen + InputProvider {}
