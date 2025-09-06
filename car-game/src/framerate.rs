use instant::Instant;

/// Delta time is capped to prevent physics bugs when unfocuses can't be detected
const DELTA_TIME_MAX: f32 = 0.1;

/// Tracks framerate in a circular buffer, maintaining the average
pub struct FramerateCounter {
    last_tick: Instant,

    buffer: Vec<f32>,
    buffer_pos: usize,
    buffer_mean: f32,

    inverse_size: f32,
}

impl FramerateCounter {
    pub fn new(buffer_size: usize) -> Self {
        if buffer_size == 0 {
            panic!("bruh");
        }
        FramerateCounter {
            last_tick: Instant::now(),
            buffer: vec![DELTA_TIME_MAX; buffer_size],
            buffer_pos: 0,
            buffer_mean: DELTA_TIME_MAX,
            inverse_size: 1.0 / (buffer_size as f32),
        }
    }

    /// Returns time delta in seconds
    pub fn tick(&mut self) -> f32 {
        let now = Instant::now();
        let mut delta = (now - self.last_tick).as_secs_f32();

        if delta > DELTA_TIME_MAX {
            log::warn!("Long delta time: {delta}");
            delta = DELTA_TIME_MAX;
        }

        // inserting value into buffer
        let old_val = self.buffer[self.buffer_pos];
        self.buffer[self.buffer_pos] = delta;
        self.buffer_pos = (self.buffer_pos + 1) % self.buffer.len();

        // updating buffer mean
        self.buffer_mean -= old_val * self.inverse_size;
        self.buffer_mean += delta * self.inverse_size;

        self.last_tick = now;
        return delta;
    }

    pub fn fps(&self) -> f32 {
        1.0 / self.buffer_mean
    }
}
