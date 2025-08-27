use winit::keyboard::KeyCode;

pub struct CarController {
    pub w_pressed: bool,
    pub a_pressed: bool,
    pub s_pressed: bool,
    pub d_pressed: bool,
}
impl CarController {
    pub fn new() -> Self {
        CarController {
            w_pressed: false,
            a_pressed: false,
            s_pressed: false,
            d_pressed: false,
        }
    }

    pub fn handle_key_event(&mut self, key: KeyCode, pressed: bool) {
        match key {
            KeyCode::KeyW => self.w_pressed = pressed,
            KeyCode::KeyA => self.a_pressed = pressed,
            KeyCode::KeyS => self.s_pressed = pressed,
            KeyCode::KeyD => self.d_pressed = pressed,
            _ => {}
        }
    }
}
