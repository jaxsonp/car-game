use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = showPauseMenu)]
    pub fn show_pause_menu(show: bool);

    #[wasm_bindgen(js_name = setDebugText)]
    pub fn set_debug_text(string: &str);
}
