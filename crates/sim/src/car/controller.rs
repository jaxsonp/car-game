use instant::Instant;
use winit::keyboard::KeyCode;

pub struct CarController {
    pub w_pressed: bool,
    pub a_pressed: bool,
    pub s_pressed: bool,
    pub d_pressed: bool,
    pub shift_pressed: bool,

    pub car_can_unflip: bool,
    pub r_press_start: Option<Instant>,
}
impl CarController {
    pub fn new() -> Self {
        CarController {
            w_pressed: false,
            a_pressed: false,
            s_pressed: false,
            d_pressed: false,
            shift_pressed: false,
            car_can_unflip: false,
            r_press_start: None,
        }
    }

    pub fn handle_key_event(&mut self, key: KeyCode, pressed: bool) {
        match key {
            KeyCode::KeyW => self.w_pressed = pressed,
            KeyCode::KeyA => self.a_pressed = pressed,
            KeyCode::KeyS => self.s_pressed = pressed,
            KeyCode::KeyD => self.d_pressed = pressed,
            KeyCode::ShiftLeft | KeyCode::ShiftRight => self.shift_pressed = pressed,
            KeyCode::KeyR => {
                if pressed {
                    if self.r_press_start.is_none() {
                        self.r_press_start = Some(Instant::now());
                    }
                } else {
                    self.r_press_start = None;
                }
            }
            _ => {}
        }
    }

    /// In seconds
    pub fn r_hold_duration(&self) -> Option<f32> {
        return self.r_press_start.map(|start_t| {
            Instant::now()
                .saturating_duration_since(start_t)
                .as_secs_f32()
        });
    }
}
