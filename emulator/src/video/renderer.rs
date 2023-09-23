use crate::video::controller::VideoController;

pub const SCREEN_WIDTH: u32 = 160;
pub const SCREEN_HEIGHT: u32 = 144;

pub trait VideoRenderer {
    fn scanline(&mut self, video: &VideoController);
    fn update_frame(&mut self);
}
