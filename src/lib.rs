use std::sync::Arc;

use render::RenderState;
use wasm_bindgen::prelude::*;
use winit::{
    application::ApplicationHandler,
    event::*,
    event_loop::{ActiveEventLoop, EventLoop},
    keyboard::PhysicalKey,
    window::Window,
};

const CANVAS_ID: &str = "main-canvas";

#[wasm_bindgen]
pub fn run_game() -> Result<(), wasm_bindgen::JsValue> {
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Debug)
        .expect_throw("Failed to initialize console logging");
    log::info!("Initializing car game");

    let event_loop = EventLoop::with_user_event()
        .build()
        .expect_throw("Failed to create event loop");
    let mut app = App::new(&event_loop);
    event_loop
        .run_app(&mut app)
        .expect_throw("Failure during event loop");

    Ok(())
}

pub struct App {
    proxy: Option<winit::event_loop::EventLoopProxy<RenderState>>,
    state: Option<RenderState>,
}

impl App {
    pub fn new(event_loop: &EventLoop<RenderState>) -> Self {
        let proxy = Some(event_loop.create_proxy());
        Self { state: None, proxy }
    }
}

impl ApplicationHandler<RenderState> for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        log::trace!("Application resumed");
        let mut window_attributes = Window::default_attributes();

        use wasm_bindgen::JsCast;
        use winit::platform::web::WindowAttributesExtWebSys;

        let window = wgpu::web_sys::window().expect_throw("Failed to get window");
        let document = window.document().expect_throw("Failed to get document");
        let canvas = document
            .get_element_by_id(CANVAS_ID)
            .expect_throw("Failed to find canvas in document");
        let html_canvas_element = canvas.unchecked_into();
        window_attributes = window_attributes.with_canvas(Some(html_canvas_element));

        let window = Arc::new(event_loop.create_window(window_attributes).unwrap());

        // Run the future asynchronously and use the
        // proxy to send the results to the event loop
        if let Some(proxy) = self.proxy.take() {
            wasm_bindgen_futures::spawn_local(async move {
                assert!(
                    proxy
                        .send_event(
                            RenderState::new(window)
                                .await
                                .expect("Unable to create canvas!!!")
                        )
                        .is_ok()
                )
            });
        }
    }

    fn user_event(&mut self, _event_loop: &ActiveEventLoop, mut event: RenderState) {
        // This is where proxy.send_event() ends up
        event.window.request_redraw();
        event.handle_resize(
            event.window.inner_size().width,
            event.window.inner_size().height,
        );
        self.state = Some(event);
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        let state = match &mut self.state {
            Some(state) => state,
            None => return,
        };

        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::Resized(size) => state.handle_resize(size.width, size.height),
            WindowEvent::RedrawRequested => {
                state.update();
                state.render().expect_throw("Render failed");
            }
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        physical_key: PhysicalKey::Code(code),
                        state: key_state,
                        ..
                    },
                ..
            } => state.handle_key(event_loop, code, key_state.is_pressed()),
            _ => {}
        }
        state.camera_controller.process_events(&event);
    }
}
