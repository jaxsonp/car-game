use car_game_utils::RingBuffer;
use instant::Instant;

/// Delta time is capped to prevent physics bugs when unfocuses can't be detected
const DELTA_TIME_MAX: f32 = 0.1;

/// Tracks framerate in a circular buffer, maintaining the average
pub struct FramerateCounter {
    last_tick: Instant,
    buf: RingBuffer,
}

impl FramerateCounter {
    pub fn new(buffer_size: usize) -> Self {
        FramerateCounter {
            last_tick: Instant::now(),
            buf: RingBuffer::new(buffer_size, DELTA_TIME_MAX),
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

        self.buf.push(delta);

        self.last_tick = now;
        return delta;
    }

    pub fn fps(&self) -> f32 {
        1.0 / self.buf.mean
    }
}
