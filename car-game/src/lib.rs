mod debug_controller;
mod framerate;
mod web_interface;

use std::sync::Arc;

use car_game_render::RenderState;
use car_game_sim::GameSimulation;
use car_game_utils::RingBuffer;
use wasm_bindgen::prelude::*;
use web_sys::js_sys::JsString;
use winit::{
    application::ApplicationHandler,
    event::*,
    event_loop::{ActiveEventLoop, EventLoop},
    keyboard::{Key, KeyCode, NamedKey, PhysicalKey},
    window::Window,
};

use debug_controller::DebugCameraController;
use framerate::FramerateCounter;

// Only send UI updates to the web side every __ frames
const DEBUG_TEXT_UPDATE_RATE: u64 = 5;
const SPEED_GAUGE_UPDATE_RATE: u64 = 2;

#[wasm_bindgen]
pub fn run_game(canvas_id: JsString) -> Result<(), wasm_bindgen::JsValue> {
    console_error_panic_hook::set_once();
    console_log::init_with_level(if cfg!(debug_assertions) {
        log::Level::Debug
    } else {
        log::Level::Info
    })
    .expect_throw("Failed to initialize console logging");
    log::info!("Starting car game");

    let event_loop = EventLoop::with_user_event()
        .build()
        .expect_throw("Failed to create event loop");
    let mut app = App::new(&event_loop, canvas_id.as_string().unwrap());
    event_loop
        .run_app(&mut app)
        .expect_throw("Failure during event loop");

    Ok(())
}

pub struct App {
    canvas_id: String,
    proxy: Option<winit::event_loop::EventLoopProxy<RenderState>>,
    render_state: Option<RenderState>,
    paused: bool,

    sim: GameSimulation,
    fps_counter: FramerateCounter,
    speed_averager: RingBuffer,
    debug_text_shown: bool,
    debug_camera_activated: bool,
    debug_camera_controller: DebugCameraController,
}

impl App {
    pub fn new(event_loop: &EventLoop<RenderState>, canvas_id: String) -> Self {
        Self {
            canvas_id,
            proxy: Some(event_loop.create_proxy()),
            render_state: None,
            paused: false,
            sim: GameSimulation::new(),
            fps_counter: FramerateCounter::new(1),
            speed_averager: RingBuffer::new(30, 0.0),
            debug_text_shown: false,
            debug_camera_activated: false,
            debug_camera_controller: DebugCameraController::new(),
        }
    }
}

impl ApplicationHandler<RenderState> for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let mut window_attributes = Window::default_attributes();

        use wasm_bindgen::JsCast;
        use winit::platform::web::WindowAttributesExtWebSys;

        let window = wgpu::web_sys::window().expect_throw("Failed to get window");
        let document = window.document().expect_throw("Failed to get document");
        let canvas = document
            .get_element_by_id(self.canvas_id.as_str())
            .expect_throw("Failed to find canvas in document");
        let html_canvas_element = canvas.unchecked_into();
        window_attributes = window_attributes.with_canvas(Some(html_canvas_element));

        let window = Arc::new(event_loop.create_window(window_attributes).unwrap_throw());

        // using the event loop to create render state asyncronously and send it into the event loop
        if let Some(proxy) = self.proxy.take() {
            wasm_bindgen_futures::spawn_local(async move {
                assert!(
                    proxy
                        .send_event(
                            RenderState::new(window)
                                .await
                                .expect("Unable to create render_state")
                        )
                        .is_ok()
                )
            });
        }

        #[cfg(debug_assertions)]
        {
            // show debug text by default if debug build
            self.debug_text_shown = true;
            web_interface::show_debug_text(true);
        }
    }

    fn user_event(&mut self, _event_loop: &ActiveEventLoop, mut event: RenderState) {
        // This is where proxy.send_event() ends up
        event.window.request_redraw();
        event.handle_resize(
            event.window.inner_size().width,
            event.window.inner_size().height,
        );
        self.render_state = Some(event);
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        let render_state = match &mut self.render_state {
            Some(rs) => rs,
            None => return,
        };

        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::Resized(size) => render_state.handle_resize(size.width, size.height),
            WindowEvent::RedrawRequested => {
                // where the magic happens

                // delta time in seconds
                let dt = self.fps_counter.tick();
                // delta time in expected frame time (60fps)
                let adjusted_dt = dt * 60.0;
                let render_snapshot = if !self.paused {
                    if self.debug_camera_activated {
                        self.debug_camera_controller
                            .update_camera(adjusted_dt, &mut render_state.scene.camera);
                    } else {
                        self.sim
                            .update_camera(adjusted_dt, &mut render_state.scene.camera);
                    }

                    if self.sim.t % DEBUG_TEXT_UPDATE_RATE == 0 && self.debug_text_shown {
                        web_interface::set_debug_text(
                            format!(
                                "fps: {:.2}\nview: {}\n\n{}\n{}",
                                self.fps_counter.fps(),
                                if self.debug_camera_activated {
                                    "freecam"
                                } else {
                                    "car"
                                },
                                render_state.get_debug_string(),
                                self.sim.get_debug_string(),
                            )
                            .as_str(),
                        );
                    }

                    let snapshot = self.sim.step(dt, !self.debug_camera_activated);

                    self.speed_averager.push(snapshot.car_speed);
                    if self.sim.t % SPEED_GAUGE_UPDATE_RATE == 0 {
                        web_interface::set_speed(self.speed_averager.mean / 38.0);
                    }

                    Some(snapshot)
                } else {
                    None
                };

                render_state
                    .render(render_snapshot)
                    .expect_throw("Render failed");
            }
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        physical_key: PhysicalKey::Code(code),
                        ref logical_key,
                        state: key_state,
                        ..
                    },
                ..
            } => {
                let pressed = key_state.is_pressed();
                match (code, pressed) {
                    (KeyCode::Escape, true) => {
                        log::debug!("Toggled pause");
                        self.paused = !self.paused;
                        web_interface::show_pause_menu(self.paused);
                    }
                    (KeyCode::Tab, true) => {
                        log::debug!("Switched camera mode");
                        self.debug_camera_activated = !self.debug_camera_activated;
                    }
                    _ => {}
                }
                if pressed && matches!(logical_key, Key::Named(NamedKey::F1)) {
                    self.debug_text_shown = !self.debug_text_shown;
                    log::debug!("Toggled debug text: {}", self.debug_text_shown);
                    web_interface::show_debug_text(self.debug_text_shown);
                }

                self.debug_camera_controller.handle_key_event(code, pressed);
                self.sim.controller.handle_key_event(code, pressed);
            }
            WindowEvent::Focused(focused) => {
                log::debug!("Focused: {focused}");
                if focused == false {
                    if !self.paused {
                        web_interface::show_pause_menu(true);
                        self.paused = true;
                    }
                }
            }
            _ => {}
        }

        render_state.handle_window_event(&event);
    }
}
