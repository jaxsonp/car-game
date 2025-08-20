use cgmath::{Deg, Matrix4, Point3, Rad, Vector3};
use wgpu::SurfaceConfiguration;
use winit::{
    event::{ElementState, KeyEvent, WindowEvent},
    keyboard::{KeyCode, PhysicalKey},
};

pub struct Camera {
    eye: Point3<f32>,
    target: Point3<f32>,
    up: Vector3<f32>,
    proj: Projection,
}
impl Camera {
    const DEFAULT_FOVY: Deg<f32> = Deg(45.0);

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
            proj: Projection::new(config.width, config.height, Self::DEFAULT_FOVY),
        }
    }

    pub fn get_view_projection_matrix(&self) -> CameraUniformMatrix {
        let view_matrix = Matrix4::look_at_rh(self.eye, self.target, self.up);
        return CameraUniformMatrix {
            view_proj: (OPENGL_TO_WGPU_MATRIX * self.proj.matrix * view_matrix).into(),
        };
    }

    pub fn handle_resize(&mut self, width: u32, height: u32) {
        self.proj.handle_resize(width, height);
    }
}

struct Projection {
    aspect_ratio: f32,
    fovy: Rad<f32>,
    // calculated matrix
    matrix: Matrix4<f32>,
}
impl Projection {
    const CLIP_NEAR: f32 = 0.1;
    const CLIP_FAR: f32 = 100.0;

    fn new<R: Into<Rad<f32>>>(width: u32, height: u32, fovy: R) -> Self {
        let fovy = fovy.into();
        let aspect_ratio = (width as f32) / (height as f32);
        Projection {
            aspect_ratio,
            fovy,
            matrix: cgmath::perspective(fovy, aspect_ratio, Self::CLIP_NEAR, Self::CLIP_FAR),
        }
    }

    fn handle_resize(&mut self, width: u32, height: u32) {
        self.aspect_ratio = (width as f32) / (height as f32);
        self.matrix = cgmath::perspective(
            self.fovy,
            self.aspect_ratio,
            Self::CLIP_NEAR,
            Self::CLIP_FAR,
        );
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

pub struct CameraController {
    is_forward_pressed: bool,
    is_backward_pressed: bool,
    is_left_pressed: bool,
    is_right_pressed: bool,
}

impl CameraController {
    const SPEED: f32 = 0.1;

    pub fn new() -> Self {
        Self {
            is_forward_pressed: false,
            is_backward_pressed: false,
            is_left_pressed: false,
            is_right_pressed: false,
        }
    }

    pub fn process_events(&mut self, event: &WindowEvent) -> bool {
        match event {
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        state,
                        physical_key: PhysicalKey::Code(keycode),
                        ..
                    },
                ..
            } => {
                let is_pressed = *state == ElementState::Pressed;
                match keycode {
                    KeyCode::KeyW | KeyCode::ArrowUp => {
                        self.is_forward_pressed = is_pressed;
                        true
                    }
                    KeyCode::KeyA | KeyCode::ArrowLeft => {
                        self.is_left_pressed = is_pressed;
                        true
                    }
                    KeyCode::KeyS | KeyCode::ArrowDown => {
                        self.is_backward_pressed = is_pressed;
                        true
                    }
                    KeyCode::KeyD | KeyCode::ArrowRight => {
                        self.is_right_pressed = is_pressed;
                        true
                    }
                    _ => false,
                }
            }
            _ => false,
        }
    }

    pub fn update_camera(&self, camera: &mut Camera) {
        use cgmath::InnerSpace;
        let forward = camera.target - camera.eye;
        let forward_norm = forward.normalize();
        let forward_mag = forward.magnitude();

        // Prevents glitching when the camera gets too close to the
        // center of the scene.
        if self.is_forward_pressed && forward_mag > CameraController::SPEED {
            camera.eye += forward_norm * CameraController::SPEED;
        }
        if self.is_backward_pressed {
            camera.eye -= forward_norm * CameraController::SPEED;
        }

        let right = forward_norm.cross(camera.up);

        // Redo radius calc in case the forward/backward is pressed.
        let forward = camera.target - camera.eye;
        let forward_mag = forward.magnitude();

        if self.is_right_pressed {
            // Rescale the distance between the target and the eye so
            // that it doesn't change. The eye, therefore, still
            // lies on the circle made by the target and eye.
            camera.eye = camera.target
                - (forward + right * CameraController::SPEED).normalize() * forward_mag;
        }
        if self.is_left_pressed {
            camera.eye = camera.target
                - (forward - right * CameraController::SPEED).normalize() * forward_mag;
        }
    }
}
