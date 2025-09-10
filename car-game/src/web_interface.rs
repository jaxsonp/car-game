use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = updateFPS)]
    pub fn update_hud_fps(fps: f32);
    #[wasm_bindgen(js_name = setDebugText)]
    pub fn set_debug_text(string: &str);
}
