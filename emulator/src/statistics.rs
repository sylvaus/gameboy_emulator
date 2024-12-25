use std::time::Instant;

const FPS_FILTERING_GAIN: f64 = 0.4;

#[derive(Debug, Copy, Clone, Default)]
pub struct Statistics {
    pub fps: f64,
}

pub struct StatisticsRecorder {
    last_frame: Option<Instant>,
    stats: Statistics,
}

impl StatisticsRecorder {
    pub fn new() -> Self {
        Self {
            last_frame: None,
            stats: Statistics::default(),
        }
    }

    pub fn record_frame(&mut self) {
        let now = Instant::now();
        if let Some(last) = self.last_frame {
            let frame_latency = now.duration_since(last);
            let new_fps = 1.0 / frame_latency.as_secs_f64();
            self.stats.fps =
                (new_fps * FPS_FILTERING_GAIN) + (self.stats.fps * (1.0 - FPS_FILTERING_GAIN));
        }
        self.last_frame = Some(now);
    }


    #[allow(dead_code)]
    pub fn stats(&self) -> Statistics {
        self.stats
    }
}
