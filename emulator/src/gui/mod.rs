use crate::joypad::InputProvider;
use crate::video::renderer::VideoRenderer;

pub(crate) mod sdl2;

pub trait Gui: VideoRenderer + InputProvider {}
