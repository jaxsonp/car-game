use winit::keyboard::KeyCode;

pub struct CarController {
    pub w_pressed: bool,
    pub s_pressed: bool,
}
impl CarController {
    pub fn new() -> Self {
        CarController {
            w_pressed: false,
            s_pressed: false,
        }
    }

    pub fn handle_key_event(&mut self, key: KeyCode, pressed: bool) {
        match key {
            KeyCode::KeyW => self.w_pressed = pressed,
            KeyCode::KeyS => self.s_pressed = pressed,
            _ => {}
        }
    }
}
