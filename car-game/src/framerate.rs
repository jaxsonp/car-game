use instant::{Duration, Instant};

/// Tracks framerate per "period", specified with `rate` in `::new(...)`. A callback is used to communicated recalculated FPS values once per period.
pub struct FramerateCounter {
    last_frame: Instant,
    period_start: Instant,
    period_frame_count: u32,
    period_duration: f32, // in seconds
    pub fps: f32,
}

impl FramerateCounter {
    pub fn new(rate: Duration) -> Self {
        FramerateCounter {
            last_frame: Instant::now(),
            period_start: Instant::now(),
            period_frame_count: 0,
            period_duration: rate.as_secs_f32(),
            fps: 0.0,
        }
    }

    /// Returns time delta in seconds
    pub fn tick(&mut self) -> f32 {
        let now = Instant::now();
        self.period_frame_count += 1;

        if (now - self.period_start).as_secs_f32() > self.period_duration {
            // period over
            self.fps = (self.period_frame_count as f32) / self.period_duration;

            self.period_frame_count = 0;
            self.period_start = now;
        }

        let delta = (now - self.last_frame).as_secs_f32();
        self.last_frame = now;
        return delta;
    }
}
