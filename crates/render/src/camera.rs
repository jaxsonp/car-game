use cgmath::{Deg, Matrix4, Point3, Rad, Vector3};
use wgpu::SurfaceConfiguration;

pub struct Camera {
    pub eye: Point3<f32>,
    pub target: Point3<f32>,
    pub up: Vector3<f32>,
    pub aspect_ratio: f32,
    pub fovy: Rad<f32>,
}
impl Camera {
    const DEFAULT_FOVY: Deg<f32> = Deg(70.0);
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
        let view_matrix = Matrix4::look_at_rh(self.eye, self.target, self.up);
        let proj_matrix = cgmath::perspective(
            self.fovy,
            self.aspect_ratio,
            Self::CLIP_NEAR,
            Self::CLIP_FAR,
        );
        return CameraUniformMatrix {
            view_proj: (OPENGL_TO_WGPU_MATRIX * proj_matrix * view_matrix).into(),
        };
    }

    pub fn handle_resize(&mut self, width: u32, height: u32) {
        self.aspect_ratio = (width as f32) / (height as f32);
    }
}

#[rustfmt::skip]
const OPENGL_TO_WGPU_MATRIX: cgmath::Matrix4<f32> = cgmath::Matrix4::from_cols(
    cgmath::Vector4::new(1.0, 0.0, 0.0, 0.0),
    cgmath::Vector4::new(0.0, 1.0, 0.0, 0.0),
    cgmath::Vector4::new(0.0, 0.0, 0.5, 0.0),
    cgmath::Vector4::new(0.0, 0.0, 0.5, 1.0),
);

/// Needed this format to pass into buffer
#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniformMatrix {
    view_proj: [[f32; 4]; 4],
}
