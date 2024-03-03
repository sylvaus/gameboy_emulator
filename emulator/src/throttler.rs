use std::ops::Div;
use std::thread::sleep;
use std::time::{Duration, Instant};

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
            let time_left =
                expected_time.saturating_sub(Instant::now().duration_since(self.previous));
            if time_left > Duration::from_millis(1) {
                sleep(time_left);
            }
        }
        self.previous = Instant::now()
    }
}
