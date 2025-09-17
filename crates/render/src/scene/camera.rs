use car_game_utils::Camera;
use nalgebra::{Matrix4, Perspective3};

use crate::uniforms::Matrix4Uniform;

pub fn get_view_projection_matrix(camera: &Camera) -> Matrix4Uniform {
    let view_matrix = Matrix4::look_at_rh(&camera.eye, &camera.target, &camera.up);
    let proj_matrix = Perspective3::new(
        camera.aspect_ratio,
        camera.fovy,
        Camera::CLIP_NEAR,
        Camera::CLIP_FAR,
    )
    .to_homogeneous();
    return Matrix4Uniform::from((OPENGL_TO_WGPU_MATRIX * proj_matrix) * view_matrix);
}

#[rustfmt::skip]
pub const OPENGL_TO_WGPU_MATRIX: Matrix4<f32> = Matrix4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.5,
    0.0, 0.0, 0.0, 1.0,
);
