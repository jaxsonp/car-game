use nalgebra::{Matrix4, Perspective3};
use utils::Camera;

pub fn get_view_projection_matrix(camera: &Camera) -> CameraUniformMatrix {
    let view_matrix = Matrix4::look_at_rh(&camera.eye, &camera.target, &camera.up);
    let proj_matrix = Perspective3::new(
        camera.aspect_ratio,
        camera.fovy,
        Camera::CLIP_NEAR,
        Camera::CLIP_FAR,
    )
    .to_homogeneous();
    return CameraUniformMatrix {
        view_proj: (proj_matrix * view_matrix).into(),
    };
}

/// Needed this format to pass into buffer
#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniformMatrix {
    view_proj: [[f32; 4]; 4],
}
