use std::ops::Div;
use std::time::Duration;
use crate::interrupts::Interrupt;
use macros::BitAccessor;

// https://gbdev.io/pandocs/Specifications.html#specifications
const CPU_INSTRUCTION_PER_SECONDS: u32 = 1 << 22;

// Information from: https://gbdev.io/pandocs/Timer_and_Divider_Registers.html
pub const TIMER_START_ADDRESS: u16 = 0xFF04;
pub const DIVIDE_REGISTER_ADDRESS: u16 = 0xFF04;
pub const TIME_COUNTER_ADDRESS: u16 = 0xFF05;
pub const TIMER_MODULO_ADDRESS: u16 = 0xFF06;
pub const TIMER_CONTROL_ADDRESS: u16 = 0xFF07;
pub const TIMER_END_ADDRESS: u16 = 0xFF07;
pub const TIMER_0_DIVIDER: u64 = 1024;
pub const TIMER_1_DIVIDER: u64 = 16;
pub const TIMER_2_DIVIDER: u64 = 64;
pub const TIMER_3_DIVIDER: u64 = 256;

pub fn convert_cycles_to_duration(nb_cycles: u64) -> Duration {
    Duration::from_secs(nb_cycles).div(CPU_INSTRUCTION_PER_SECONDS)
}

const TIMER_CONTROL_UNUSED_BITS: u8 = 0b1111_1000;

#[derive(BitAccessor, Debug, Copy, Clone, Default)]
pub struct Timer {
    divide_cycles: u64,
    divide_register: u8,
    timer_cycles: u64,
    timer_counter: u8,
    timer_modulo: u8,

    #[bit_offset_size(timer_enable, 2, 1)]
    #[bit_offset_size(input_clock_select, 0, 2)]
    timer_control: u8,
}

impl Timer {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn update(&mut self, nb_cycles: u64) -> Option<Interrupt> {
        // Information from: https://gbdev.io/pandocs/Timer_and_Divider_Registers.html
        self.divide_cycles = self.divide_cycles.wrapping_add(nb_cycles);
        self.divide_register = ((self.divide_cycles >> 8) & 0xFF) as u8;

        if self.read_timer_enable() == 0 {
            return None;
        }

        let divider = self.get_timer_divider();

        self.timer_cycles += nb_cycles;
        if self.timer_cycles >= divider {
            let increment = self.timer_cycles / divider;
            self.timer_cycles %= divider;
            let counter = self.timer_counter as u64 + increment;
            if counter > 0xFF {
                let overflow = counter & 0xFF;
                let modulo = 0x100 -  (self.timer_modulo as u64);
                let increment_after_wrap = overflow % modulo;
                self.timer_counter = self.timer_modulo + (increment_after_wrap as u8);
                return Some(Interrupt::Timer);
            } else {
                self.timer_counter = counter as u8;
            }
        }

        None
    }

    pub fn write(&mut self, address: u16, value: u8) {
        match address {
            // Divide register is reset when written to:
            // https://gbdev.io/pandocs/Timer_and_Divider_Registers.html#ff04--div-divider-register
            DIVIDE_REGISTER_ADDRESS => {
                self.divide_cycles = 0;
                self.divide_register = 0;
                // Timer
                self.timer_cycles = if self.is_falling_edge(self.timer_cycles) {
                    self.get_timer_divider()
                } else {
                    0
                };
            }
            TIME_COUNTER_ADDRESS => self.timer_counter = value,
            TIMER_MODULO_ADDRESS => self.timer_modulo = value,
            TIMER_CONTROL_ADDRESS => self.timer_control = value,
            _ => panic!("Invalid address for timer {}", address),
        }
    }

    pub fn read(&self, address: u16) -> u8 {
        match address {
            DIVIDE_REGISTER_ADDRESS => self.divide_register,
            TIME_COUNTER_ADDRESS => self.timer_counter,
            TIMER_MODULO_ADDRESS => self.timer_modulo,
            TIMER_CONTROL_ADDRESS => self.timer_control | TIMER_CONTROL_UNUSED_BITS,
            _ => panic!("Invalid address for timer {}", address),
        }
    }

    fn is_falling_edge(&self, timer_cycles: u64) -> bool {
        match self.read_input_clock_select() {
            0 => (timer_cycles & 0b10_0000_0000) > 0,
            1 => (timer_cycles & 0b1000) > 0,
            2 => (timer_cycles & 0b10_0000) > 0,
            3 => (timer_cycles & 0b1000_0000) > 0,
            _ => unreachable!("This should never happen"),
        }
    }

    fn get_timer_divider(&self) -> u64 {
        match self.read_input_clock_select() {
            0 => TIMER_0_DIVIDER,
            1 => TIMER_1_DIVIDER,
            2 => TIMER_2_DIVIDER,
            3 => TIMER_3_DIVIDER,
            _ => unreachable!("This should never happen"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn divider_increment() {
        let mut timer = Timer::new();
        assert_eq!(timer.divide_register, 0);

        timer.update(250);
        assert_eq!(timer.divide_register, 0);

        timer.update(250);
        assert_eq!(timer.divide_register, 1);
    }

    #[test]
    fn divider_wrap() {
        let mut timer = Timer::new();

        timer.update(255 * 256 + 255);
        assert_eq!(timer.divide_register, 255);

        timer.update(1);
        assert_eq!(timer.divide_register, 0);
    }

    #[test]
    fn timer_increment_clock_select_0() {
        let mut timer = Timer::new();
        timer.timer_control = 0b100;
        assert_eq!(timer.timer_counter, 0);

        timer.update(1023);
        assert_eq!(timer.timer_counter, 0);

        assert_eq!(timer.update(1), None);
        assert_eq!(timer.timer_counter, 1);
    }

    #[test]
    fn timer_increment_clock_select_1() {
        let mut timer = Timer::new();
        timer.timer_control = 0b101;
        assert_eq!(timer.timer_counter, 0);

        timer.update(15);
        assert_eq!(timer.timer_counter, 0);

        assert_eq!(timer.update(1), None);
        assert_eq!(timer.timer_counter, 1);
    }

    #[test]
    fn timer_increment_clock_select_2() {
        let mut timer = Timer::new();
        timer.timer_control = 0b110;
        assert_eq!(timer.timer_counter, 0);

        timer.update(63);
        assert_eq!(timer.timer_counter, 0);

        assert_eq!(timer.update(1), None);
        assert_eq!(timer.timer_counter, 1);
    }

    #[test]
    fn timer_increment_clock_select_3() {
        let mut timer = Timer::new();
        timer.timer_control = 0b111;
        assert_eq!(timer.timer_counter, 0);

        timer.update(255);
        assert_eq!(timer.timer_counter, 0);

        assert_eq!(timer.update(1), None);
        assert_eq!(timer.timer_counter, 1);
    }

    #[test]
    fn timer_increment_counter_twice() {
        let mut timer = Timer::new();
        timer.timer_control = 0b101;
        assert_eq!(timer.timer_counter, 0);

        assert_eq!(timer.update(32), None);
        assert_eq!(timer.timer_counter, 2);
    }

    #[test]
    fn timer_does_not_increment_if_disabled() {
        let mut timer = Timer::new();
        timer.timer_control = 0b000;
        assert_eq!(timer.timer_counter, 0);

        assert_eq!(timer.update(1025), None);
        assert_eq!(timer.timer_counter, 0);
    }

    #[test]
    fn timer_wrap_exact() {
        let mut timer = Timer::new();
        timer.timer_control = 0b101;
        timer.timer_counter = 0xFF;
        timer.timer_modulo = 0xF4;

        assert_eq!(timer.update(16), Some(Interrupt::Timer));
        assert_eq!(timer.timer_counter, 0xF4);
    }

    #[test]
    fn timer_wrap_overflow() {
        let mut timer = Timer::new();
        timer.timer_control = 0b101;
        timer.timer_counter = 0xFF;
        timer.timer_modulo = 0xF4;

        assert_eq!(timer.update(32), Some(Interrupt::Timer));
        assert_eq!(timer.timer_counter, 0xF5);
    }

    #[test]
    fn timer_wrap_multiple_times() {
        let mut timer = Timer::new();
        timer.timer_control = 0b101;
        // 6 * 16 = 96 cycles to trigger an interrupt
        timer.timer_counter = 0xFA;
        timer.timer_modulo = 0xFA;

        assert_eq!(timer.update(96 * 2 + 32), Some(Interrupt::Timer));
        assert_eq!(timer.timer_counter, 0xFC);
    }

    #[test]
    fn timer_every_4_m_cycles_no_interrupt() {
        let mut timer = Timer::new();
        timer.timer_control = 0b101;
        timer.timer_counter = 0xFF;
        timer.timer_modulo = 0xFF;

        assert_eq!(timer.update(10), None);
        assert_eq!(timer.timer_counter, 0xFF);
    }

    #[test]
    fn timer_every_4_m_cycles_interrupt_no_overflow() {
        let mut timer = Timer::new();
        timer.timer_control = 0b101;
        timer.timer_counter = 0xFF;
        timer.timer_modulo = 0xFF;

        assert_eq!(timer.update(16), Some(Interrupt::Timer));
        assert_eq!(timer.timer_counter, 0xFF);
    }

    #[test]
    fn timer_every_4_m_cycles_interrupt_and_overflow() {
        let mut timer = Timer::new();
        timer.timer_control = 0b101;
        timer.timer_counter = 0xFF;
        timer.timer_modulo = 0xFF;

        assert_eq!(timer.update(16*2 + 5), Some(Interrupt::Timer));
        assert_eq!(timer.timer_counter, 0xFF);
        assert_eq!(timer.timer_cycles, 5);
    }
}
