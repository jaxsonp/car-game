use nalgebra::{Isometry3, Point3, Vector3};

#[derive(Clone, Copy)]
pub struct RenderSnapshot {
    pub car_transform: Isometry3<f32>,
    pub car_speed: f32,
    pub wheel_transforms: [Isometry3<f32>; 4],
    pub skid_contacts: [Option<SkidContact>; 4],
    pub water_level: f32,
}

#[derive(Clone, Copy)]
pub struct SkidContact {
    pub pos: Point3<f32>,
    pub normal: Vector3<f32>,
}

pub struct Camera {
    pub eye: Point3<f32>,
    pub target: Point3<f32>,
    pub up: Vector3<f32>,
    pub aspect_ratio: f32,
    pub fovy: f32,
}
impl Camera {
    pub const DEFAULT_FOVY: f32 = 70.0f32.to_radians();
    pub const CLIP_NEAR: f32 = 0.1;
    pub const CLIP_FAR: f32 = 500.0;

    pub fn new<P: Into<Point3<f32>>, V: Into<Vector3<f32>>>(
        eye: P,
        target: P,
        up: V,
        width: f32,
        height: f32,
    ) -> Self {
        Camera {
            eye: eye.into(),
            target: target.into(),
            up: up.into(),
            aspect_ratio: width / height,
            fovy: Self::DEFAULT_FOVY.into(),
        }
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.aspect_ratio = (width as f32) / (height as f32);
    }
}

/// A circular buffer that tracks the current mean of its population
pub struct RingBuffer {
    buffer: Vec<f32>,
    pos: usize,
    inverse_size: f32,

    pub mean: f32,
}
impl RingBuffer {
    pub fn new(size: usize, init_value: f32) -> Self {
        if size == 0 {
            panic!("bruh");
        }
        RingBuffer {
            buffer: vec![init_value; size],
            pos: 0,
            inverse_size: 1.0 / (size as f32),
            mean: init_value,
        }
    }

    /// push a new value into the buffer, returns the resulting mean
    pub fn push(&mut self, val: f32) {
        let old_val = self.buffer[self.pos];
        self.buffer[self.pos] = val;
        self.pos = (self.pos + 1) % self.buffer.len();

        // updating buffer mean
        self.mean -= old_val * self.inverse_size;
        self.mean += val * self.inverse_size;
    }
}
