pub const AUDIO_WAVE_START_ADDRESS: u16 = 0xFF10;
pub const AUDIO_WAVE_END_ADDRESS: u16 = 0xFF3F;

pub struct SoundController {}

impl SoundController {
    pub fn read(&self, _address: u16) -> u8 {
        // TODO: Implement sound communication.
        0
    }

    pub fn write(&mut self, _address: u16, _value: u8) {
        // TODO: Implement sound communication.
    }
}
