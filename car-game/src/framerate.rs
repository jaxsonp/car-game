use instant::{Duration, Instant};

/// Tracks framerate per "period", specified with `rate` in `::new(...)`. A callback is used to communicated recalculated FPS values once per period.
pub struct FramerateCounter {
    period_start: Instant,
    period_frame_count: u32,
    period_duration: f32, // in seconds
    fps: f32,
    pub updated: bool,
}

impl FramerateCounter {
    pub fn new(rate: Duration) -> Self {
        FramerateCounter {
            period_start: Instant::now(),
            period_frame_count: 0,
            period_duration: rate.as_secs_f32(),
            fps: 0.0,
            updated: false,
        }
    }

    pub fn tick(&mut self) {
        self.period_frame_count += 1;

        if (Instant::now() - self.period_start).as_secs_f32() > self.period_duration {
            // period over
            self.fps = (self.period_frame_count as f32) / self.period_duration;
            self.updated = true;

            self.period_frame_count = 0;
            self.period_start = Instant::now();
        }
    }

    pub fn get_fps(&mut self) -> f32 {
        self.updated = false;
        return self.fps;
    }
}
