use std::ops::Div;
use std::time::{Duration, Instant};
use spin_sleep::sleep;

const CPU_FREQUENCY: u32 = 1 << 22;

pub struct Throttler {
    previous: Instant,
}

impl Throttler {
    pub fn new() -> Self {
        Self {
            previous: Instant::now(),
        }
    }

    pub fn throttle_for_cycles(&mut self, nb_cycles: u64) {
        if nb_cycles > 0 {
            let expected_time = Duration::from_secs(nb_cycles).div(CPU_FREQUENCY);
            let actual_time = self.previous.elapsed();
            let time_left = expected_time.saturating_sub(actual_time);
            if time_left > Duration::from_millis(1) {
                sleep(time_left);
            }
        }
        self.previous = Instant::now()
    }
}
