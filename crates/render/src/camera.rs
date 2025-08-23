use nalgebra::{Matrix4, Perspective3, Point3, Vector3};
use wgpu::SurfaceConfiguration;

pub struct Camera {
    pub eye: Point3<f32>,
    pub target: Point3<f32>,
    pub up: Vector3<f32>,
    pub aspect_ratio: f32,
    pub fovy: f32,
}
impl Camera {
    const DEFAULT_FOVY: f32 = 70.0f32.to_radians();
    const CLIP_NEAR: f32 = 0.1;
    const CLIP_FAR: f32 = 100.0;

    pub fn new<P: Into<Point3<f32>>, V: Into<Vector3<f32>>>(
        eye: P,
        target: P,
        up: V,
        config: &SurfaceConfiguration,
    ) -> Self {
        Camera {
            eye: eye.into(),
            target: target.into(),
            up: up.into(),
            aspect_ratio: (config.width as f32) / (config.height as f32),
            fovy: Self::DEFAULT_FOVY.into(),
        }
    }

    pub fn get_view_projection_matrix(&self) -> CameraUniformMatrix {
        let view_matrix = Matrix4::look_at_rh(&self.eye, &self.target, &self.up);
        let proj_matrix = Perspective3::new(
            self.aspect_ratio,
            self.fovy,
            Self::CLIP_NEAR,
            Self::CLIP_FAR,
        )
        .to_homogeneous();
        return CameraUniformMatrix {
            view_proj: (proj_matrix * view_matrix).into(),
        };
    }

    pub fn handle_resize(&mut self, width: u32, height: u32) {
        self.aspect_ratio = (width as f32) / (height as f32);
    }
}

/// Needed this format to pass into buffer
#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniformMatrix {
    view_proj: [[f32; 4]; 4],
}
