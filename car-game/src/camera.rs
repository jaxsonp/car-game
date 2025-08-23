use nalgebra::{Rotation, Unit, Vector3};
use winit::{
    event::{KeyEvent, WindowEvent},
    keyboard::{KeyCode, PhysicalKey},
};

use render::camera::Camera;

pub trait CameraController {
    fn handle_window_event(&mut self, event: &WindowEvent);
    fn update(&mut self, t_delta: f32, cam: &mut Camera);
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
    /// units per sec
    const MOVE_SPEED: f32 = 9.5;
    /// rads per sec
    const TURN_SPEED: f32 = 80f32.to_radians();

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

    fn update(&mut self, t_delta: f32, camera: &mut Camera) {
        let turn_speed = Self::TURN_SPEED * t_delta;
        let move_speed = Self::MOVE_SPEED * t_delta;

        let mut facing = Unit::new_normalize(camera.target - camera.eye);
        let up = Vector3::y_axis();

        if self.left_pressed {
            facing = Rotation::from_axis_angle(&up, turn_speed) * facing;
        }
        if self.right_pressed {
            facing = Rotation::from_axis_angle(&up, -turn_speed) * facing;
        }
        let right = Unit::new_normalize(facing.cross(&up));
        if self.up_pressed {
            facing = Rotation::from_axis_angle(&right, turn_speed) * facing;
        }
        if self.down_pressed {
            facing = Rotation::from_axis_angle(&right, -turn_speed) * facing;
        }

        let horizon = Unit::new_normalize(Vector3::new(facing.x, 0.0, facing.z));
        if self.w_pressed {
            camera.eye += horizon.into_inner() * move_speed;
        }
        if self.a_pressed {
            camera.eye -= right.into_inner() * move_speed;
        }
        if self.s_pressed {
            camera.eye -= horizon.into_inner() * move_speed;
        }
        if self.d_pressed {
            camera.eye += right.into_inner() * move_speed;
        }
        if self.space_pressed {
            camera.eye += Vector3::y() * move_speed;
        }
        if self.shift_pressed {
            camera.eye -= Vector3::y() * move_speed;
        }

        camera.target = camera.eye + facing.into_inner();
    }
}
