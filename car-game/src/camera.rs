use cgmath::{Deg, Quaternion, Rotation, Rotation3, Vector3};
use winit::{
    event::{KeyEvent, WindowEvent},
    keyboard::{KeyCode, PhysicalKey},
};

use render::camera::Camera;

pub trait CameraController {
    fn handle_window_event(&mut self, event: &WindowEvent);
    fn update(&mut self, cam: &mut Camera);
}

/// A simple stand-in camera controller for flying around the scene creative mode style
pub struct DebugCameraController {
    w_pressed: bool,
    a_pressed: bool,
    s_pressed: bool,
    d_pressed: bool,
    shift_pressed: bool,
    space_pressed: bool,
    up_pressed: bool,
    down_pressed: bool,
    left_pressed: bool,
    right_pressed: bool,
}

impl DebugCameraController {
    const MOVE_SPEED: f32 = 0.15;
    const TURN_SPEED: Deg<f32> = Deg(1.2);

    pub fn new() -> Self {
        Self {
            w_pressed: false,
            a_pressed: false,
            s_pressed: false,
            d_pressed: false,
            shift_pressed: false,
            space_pressed: false,
            up_pressed: false,
            down_pressed: false,
            left_pressed: false,
            right_pressed: false,
        }
    }
}
impl CameraController for DebugCameraController {
    fn handle_window_event(&mut self, event: &WindowEvent) {
        match event {
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        physical_key: PhysicalKey::Code(key),
                        state: key_state,
                        ..
                    },
                ..
            } => {
                let pressed = key_state.is_pressed();
                match key {
                    KeyCode::KeyW => self.w_pressed = pressed,
                    KeyCode::KeyA => self.a_pressed = pressed,
                    KeyCode::KeyS => self.s_pressed = pressed,
                    KeyCode::KeyD => self.d_pressed = pressed,
                    KeyCode::Space => self.space_pressed = pressed,
                    KeyCode::ShiftLeft | KeyCode::ShiftRight => self.shift_pressed = pressed,
                    KeyCode::ArrowUp => self.up_pressed = pressed,
                    KeyCode::ArrowLeft => self.left_pressed = pressed,
                    KeyCode::ArrowDown => self.down_pressed = pressed,
                    KeyCode::ArrowRight => self.right_pressed = pressed,
                    _ => {}
                }
            }
            _ => {}
        }
    }

    fn update(&mut self, camera: &mut Camera) {
        use cgmath::InnerSpace;
        let mut facing = (camera.target - camera.eye).normalize();

        if self.left_pressed {
            facing = Quaternion::from_angle_y(Self::TURN_SPEED).rotate_vector(facing);
        }
        if self.right_pressed {
            facing = Quaternion::from_angle_y(-Self::TURN_SPEED).rotate_vector(facing);
        }
        let right = facing.cross(Vector3::unit_y()).normalize();
        if self.up_pressed {
            facing = Quaternion::from_axis_angle(right, Self::TURN_SPEED).rotate_vector(facing);
        }
        if self.down_pressed {
            facing = Quaternion::from_axis_angle(right, -Self::TURN_SPEED).rotate_vector(facing);
        }

        let horizon = Vector3::new(facing.x, 0.0, facing.z);
        if self.w_pressed {
            camera.eye += horizon * Self::MOVE_SPEED;
        }
        if self.a_pressed {
            camera.eye -= right * Self::MOVE_SPEED;
        }
        if self.s_pressed {
            camera.eye -= horizon * Self::MOVE_SPEED;
        }
        if self.d_pressed {
            camera.eye += right * Self::MOVE_SPEED;
        }
        if self.space_pressed {
            camera.eye += Vector3::unit_y() * Self::MOVE_SPEED;
        }
        if self.shift_pressed {
            camera.eye -= Vector3::unit_y() * Self::MOVE_SPEED;
        }

        camera.target = camera.eye + facing;
    }
}
