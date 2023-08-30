const DIVIDE_REGISTER_ADDRESS: u16 = 0xFF04;
const TIME_COUNTER_ADDRESS: u16 = 0xFF05;
const TIMER_MODULO_ADDRESS: u16 = 0xFF06;
const TIMER_CONTROL_ADDRESS: u16 = 0xFF07;

struct Timer {
    divide_register: u8,
    timer_counter: u8,
    timer_modulo: u8,
    timer_control: u8,
}

impl Timer {
    pub fn write(&mut self, address: u16, value: u8) {
        match address {
            // Divide register is reset when written to:
            // https://gbdev.io/pandocs/Timer_and_Divider_Registers.html#ff04--div-divider-register
            DIVIDE_REGISTER_ADDRESS => self.divide_register = 0,
            TIME_COUNTER_ADDRESS => self.timer_counter = value,
            TIMER_MODULO_ADDRESS => self.timer_modulo = value,
            TIMER_CONTROL_ADDRESS => self.timer_control = value,
            _ => panic!("Invalid address for timer {}", address),
        }
    }

    pub fn read(&mut self, address: u16) -> u8 {
        match address {
            DIVIDE_REGISTER_ADDRESS => self.divide_register,
            TIME_COUNTER_ADDRESS => self.timer_counter,
            TIMER_MODULO_ADDRESS => self.timer_modulo,
            TIMER_CONTROL_ADDRESS => self.timer_control,
            _ => panic!("Invalid address for timer {}", address),
        }
    }
}
