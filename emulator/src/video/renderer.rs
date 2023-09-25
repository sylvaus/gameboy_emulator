use crate::video::controller::VideoController;

pub const SCREEN_WIDTH: u32 = 160;
pub const SCREEN_HEIGHT: u32 = 144;

/// Information from: https://gbdev.io/pandocs/Scrolling.html#ff4aff4b--wy-wx-window-y-position-x-position-plus-7
pub const WINDOW_X_OFFSET: usize = 7;

pub trait VideoRenderer {
    fn scanline(&mut self, video: &VideoController);
    fn update_frame(&mut self);
}
