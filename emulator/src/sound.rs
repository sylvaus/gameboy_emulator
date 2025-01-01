use macros::BitAccessor;

// TODO: Implement sound.

const AUDIO_CONTROL_UNUSED_BITS: u8 = 0b0111_0000u8;


// https://gbdev.io/pandocs/Audio_Registers.html#ff26--nr52-audio-master-control
#[derive(BitAccessor, Default, Debug)]
struct AudioControl {
    #[bit_offset_size(on_off, 7, 1)]
    #[bit_offset_size(unused, 4, 3)]
    #[bit_offset_size(ch4_on, 3, 1)]
    #[bit_offset_size(ch3_on, 2, 1)]
    #[bit_offset_size(ch2_on, 1, 1)]
    #[bit_offset_size(ch1_on, 0, 1)]
    value: u8,
}

// https://gbdev.io/pandocs/Audio_Registers.html#ff25--nr51-sound-panning
#[derive(BitAccessor, Default, Debug)]
struct Panning {
    #[bit_offset_size(ch4_left, 7, 1)]
    #[bit_offset_size(ch3_left, 6, 1)]
    #[bit_offset_size(ch2_left, 5, 1)]
    #[bit_offset_size(ch1_left, 4, 1)]
    #[bit_offset_size(ch4_right, 3, 1)]
    #[bit_offset_size(ch3_right, 2, 1)]
    #[bit_offset_size(ch2_right, 1, 1)]
    #[bit_offset_size(ch1_right, 0, 1)]
    value: u8,
}

// https://gbdev.io/pandocs/Audio_Registers.html#ff24--nr50-master-volume--vin-panning
#[derive(BitAccessor, Default, Debug)]
struct VolumeAndVinPanning {
    #[bit_offset_size(vin_left, 7, 1)]
    #[bit_offset_size(volume_left, 4, 1)]
    #[bit_offset_size(vin_right, 3, 1)]
    #[bit_offset_size(colume_right, 0, 1)]
    value: u8,
}

const SWEEP_UNUSED_BITS: u8 = 0b1000_0000u8;
const PERIOD_HIGH_AND_CONTROL_UNUSED_BITS: u8 = 0b0011_1000u8;
const PERIOD_HIGH_AND_CONTROL_WRITE_ONLY_BITS: u8 = 0b1000_0111u8;

#[derive(BitAccessor, Debug, Default)]
struct Channel1 {
    // https://gbdev.io/pandocs/Audio_Registers.html#ff10--nr10-channel-1-sweep
    #[bit_offset_size(sweep_unused, 7, 1)]
    #[bit_offset_size(pace, 4, 3)]
    #[bit_offset_size(direction, 3, 1)]
    #[bit_offset_size(step, 0, 2)]
    sweep: u8,

    // https://gbdev.io/pandocs/Audio_Registers.html#ff11--nr11-channel-1-length-timer--duty-cycle
    #[bit_offset_size(wave_duty, 6, 2)]
    #[bit_offset_size(initial_length_timer, 0, 6)]
    length_timer_duty_cycle: u8,

    // https://gbdev.io/pandocs/Audio_Registers.html#ff12--nr12-channel-1-volume--envelope
    #[bit_offset_size(initial_volume, 4, 4)]
    #[bit_offset_size(envelope_direction, 3, 1)]
    #[bit_offset_size(sweep_pace, 0, 3)]
    volume_envelope: u8,

    // https://gbdev.io/pandocs/Audio_Registers.html#ff13--nr13-channel-1-period-low-write-only
    period_low: u8,

    // https://gbdev.io/pandocs/Audio_Registers.html#ff14--nr14-channel-1-period-high--control
    #[bit_offset_size(trigger, 7, 1)]
    #[bit_offset_size(length_enable, 6, 1)]
    #[bit_offset_size(period_high_and_control_unused, 3, 3)]
    #[bit_offset_size(period, 0, 3)]
    period_high_and_control: u8,
}

// https://gbdev.io/pandocs/Audio_Registers.html?search=FF15
#[derive(BitAccessor, Debug, Default)]
struct Channel2 {
    #[bit_offset_size(wave_duty, 6, 2)]
    #[bit_offset_size(initial_length_timer, 0, 6)]
    length_timer_duty_cycle: u8,

    #[bit_offset_size(initial_volume, 4, 4)]
    #[bit_offset_size(envelope_direction, 3, 1)]
    #[bit_offset_size(sweep_pace, 0, 3)]
    volume_envelope: u8,

    period_low: u8,

    #[bit_offset_size(trigger, 7, 1)]
    #[bit_offset_size(length_enable, 6, 1)]
    #[bit_offset_size(period_high_and_control_unused, 3, 3)]
    #[bit_offset_size(period, 0, 3)]
    period_high_and_control: u8,
}

const DAC_ENABLE_UNUSED_BITS: u8 = 0b0111_1111u8;
const OUTPUT_LEVEL_UNUSED_BITS: u8 = 0b1001_1111u8;

#[derive(BitAccessor, Debug, Default)]
struct Channel3 {
    // https://gbdev.io/pandocs/Audio_Registers.html#ff1a--nr30-channel-3-dac-enable
    #[bit_offset_size(dac_on_off, 7, 1)]
    #[bit_offset_size(dac_enable_unused, 0, 7)]
    dac_enable: u8,

    // https://gbdev.io/pandocs/Audio_Registers.html#ff1b--nr31-channel-3-length-timer-write-only
    length_timer: u8,

    // https://gbdev.io/pandocs/Audio_Registers.html#ff1c--nr32-channel-3-output-level
    #[bit_offset_size(output_level_unused_2, 7, 1)]
    #[bit_offset_size(output_level_value, 5, 2)]
    #[bit_offset_size(output_level_unused_1, 0, 5)]
    output_level: u8,

    // https://gbdev.io/pandocs/Audio_Registers.html#ff1d--nr33-channel-3-period-low-write-only
    period_low: u8,

    // https://gbdev.io/pandocs/Audio_Registers.html#ff1e--nr34-channel-3-period-high--control
    #[bit_offset_size(trigger, 7, 1)]
    #[bit_offset_size(length_enable, 6, 1)]
    #[bit_offset_size(period_high_and_control_unused, 3, 3)]
    #[bit_offset_size(period, 0, 3)]
    period_high_and_control: u8,

    // https://gbdev.io/pandocs/Audio_Registers.html#ff30ff3f--wave-pattern-ram
    wave_pattern: [u8; 0x10],
}
const CHANNEL_4_CONTROL_UNUSED_BITS: u8 = 0b0011_1111u8;

#[derive(BitAccessor, Debug, Default)]
struct Channel4 {
    // https://gbdev.io/pandocs/Audio_Registers.html#ff20--nr41-channel-4-length-timer-write-only
    #[bit_offset_size(length_timer_unused, 6, 2)]
    #[bit_offset_size(initial_length_timer, 0, 6)]
    length_timer: u8,

    // https://gbdev.io/pandocs/Audio_Registers.html#ff21--nr42-channel-4-volume--envelope
    #[bit_offset_size(initial_volume, 4, 4)]
    #[bit_offset_size(envelope_direction, 3, 1)]
    #[bit_offset_size(sweep_pace, 0, 3)]
    volume_envelope: u8,

    // https://gbdev.io/pandocs/Audio_Registers.html#ff22--nr43-channel-4-frequency--randomness
    #[bit_offset_size(clock_shift, 4, 4)]
    #[bit_offset_size(lfsr_width, 3, 1)]
    #[bit_offset_size(clock_divider, 0, 3)]
    frequency_randomness: u8,

    // https://gbdev.io/pandocs/Audio_Registers.html#ff23--nr44-channel-4-control
    #[bit_offset_size(trigger, 4, 4)]
    #[bit_offset_size(length_enable, 3, 1)]
    #[bit_offset_size(control_unused, 0, 3)]
    control: u8,
}

#[derive(Debug, Default)]
pub struct SoundController {
    audio_control: AudioControl,
    panning: Panning,
    volume: VolumeAndVinPanning,
    channel_1: Channel1,
    channel_2: Channel2,
    channel_3: Channel3,
    channel_4: Channel4,
}

impl SoundController {
    pub fn read(&self, address: u16) -> u8 {
        match address {
            0xFF10 => self.channel_1.sweep | SWEEP_UNUSED_BITS,
            0xFF11 => self.channel_1.length_timer_duty_cycle,
            0xFF12 => self.channel_1.volume_envelope,
            0xFF13 => /* this field is write only */ 0xFF ,
            0xFF14 => self.channel_1.period_high_and_control | PERIOD_HIGH_AND_CONTROL_UNUSED_BITS | PERIOD_HIGH_AND_CONTROL_WRITE_ONLY_BITS,
            0xFF15 => /* unused address */ 0xFF,
            0xFF16 => self.channel_2.length_timer_duty_cycle,
            0xFF17 => self.channel_2.volume_envelope,
            0xFF18 => /* this field is write only */ 0xFF ,
            0xFF19 => self.channel_2.period_high_and_control | PERIOD_HIGH_AND_CONTROL_UNUSED_BITS | PERIOD_HIGH_AND_CONTROL_WRITE_ONLY_BITS,
            0xFF1A => self.channel_3.dac_enable | DAC_ENABLE_UNUSED_BITS,
            0xFF1B => /* this field is write only */ 0xFF ,
            0xFF1C => self.channel_3.output_level | OUTPUT_LEVEL_UNUSED_BITS,
            0xFF1D => /* this field is write only */ 0xFF ,
            0xFF1E => self.channel_3.period_high_and_control | PERIOD_HIGH_AND_CONTROL_UNUSED_BITS | PERIOD_HIGH_AND_CONTROL_WRITE_ONLY_BITS,
            0xFF1F => /* unused address */ 0xFF,
            0xFF20 => /* this field is write only */ 0xFF ,
            0xFF21 => self.channel_1.volume_envelope,
            0xFF22 => self.channel_4.frequency_randomness,
            0xFF23 => self.channel_4.control | CHANNEL_4_CONTROL_UNUSED_BITS,
            0xFF24 => self.volume.value,
            0xFF25 => self.panning.value,
            0xFF26 => self.audio_control.value | AUDIO_CONTROL_UNUSED_BITS,
            0xFF27..=0xFF29 => /* unused address */ 0xFF,
            0xFF30..=0xFF3F => self.channel_3.wave_pattern[address as usize - 0xFF30],

            _ => panic!("This function should never be called with address outside range [0xFF10, 0xFF3F], called with {}", address),
        }
    }

    pub fn write(&mut self, address: u16, value: u8) {
        match address {
            0xFF10 => self.channel_1.sweep = value,
            0xFF11 => self.channel_1.length_timer_duty_cycle = value,
            0xFF12 => self.channel_1.volume_envelope = value,
            0xFF13 => self.channel_1.period_low = value,
            0xFF14 => self.channel_1.period_high_and_control = value,
            0xFF15 => {/* Unused address */}
            0xFF16 => self.channel_2.length_timer_duty_cycle = value,
            0xFF17 => self.channel_2.volume_envelope = value,
            0xFF18 => self.channel_2.period_low = value,
            0xFF19 => self.channel_2.period_high_and_control = value,
            0xFF1A => self.channel_3.dac_enable = value,
            0xFF1B => self.channel_3.length_timer = value,
            0xFF1C => self.channel_3.output_level = value,
            0xFF1D => self.channel_3.period_low = value,
            0xFF1E => self.channel_3.period_high_and_control = value,
            0xFF1F => {/* unused address */},
            0xFF20 => self.channel_4.length_timer = value,
            0xFF21 => self.channel_4.volume_envelope = value,
            0xFF22 => self.channel_4.frequency_randomness = value,
            0xFF23 => self.channel_4.control = value,
            0xFF24 => self.volume.value = value,
            0xFF25 => self.panning.value = value,
            0xFF26 => self.audio_control.value = value,
            0xFF27..=0xFF29 => {/* Unused addresses */}
            0xFF30..=0xFF3F => self.channel_3.wave_pattern[address as usize - 0xFF30] = value,

            _ => panic!("This function should never be called with address outside range [0xFF10, 0xFF3F], called with {}", address),
        }
    }
}

// TODO: add unit tests.
