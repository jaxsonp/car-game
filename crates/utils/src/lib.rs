use nalgebra::{Isometry3, Point3, Vector3};

pub struct RenderSnapshot {
    pub car_transform: Isometry3<f32>,
    /// How far below offset each wheel is (front-driver, front-pass, rear-driver, rear-pass)
    pub wheel_transforms: [Isometry3<f32>; 4],
    pub debug_string: Option<String>,
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
    pub const CLIP_FAR: f32 = 1000.0;

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
