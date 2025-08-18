use std::sync::Arc;

use wasm_bindgen::prelude::*;
use web_sys::window;
use wgpu::RequestAdapterOptions;
use winit::{
    application::ApplicationHandler,
    event::*,
    event_loop::{ActiveEventLoop, EventLoop},
    keyboard::{KeyCode, PhysicalKey},
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

// This will store the state of our game
pub struct State {
    surface: wgpu::Surface<'static>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    is_surface_configured: bool,
    window: Arc<Window>,
}

impl State {
    // We don't need this to be async right now,
    // but we will in the next tutorial
    pub async fn new(window: Arc<Window>) -> anyhow::Result<Self> {
        let size = window.inner_size();

        // The instance is a handle to our GPU
        // BackendBit::PRIMARY => Vulkan + Metal + DX12 + Browser WebGPU
        let instance = wgpu::util::new_instance_with_webgpu_detection(&wgpu::InstanceDescriptor {
            backends: wgpu::Backends::GL,
            ..Default::default()
        })
        .await;

        let surface = instance.create_surface(window.clone()).unwrap();

        let adapter = instance
            .request_adapter(&RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::None,
                force_fallback_adapter: false,
                compatible_surface: Some(&surface),
            })
            .await
            .expect_throw("No suitable adapter found");

        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor {
                label: None,
                required_features: wgpu::Features::empty(),
                // WebGL doesn't support all of wgpu's features, so if
                // we're building for the web we'll have to disable some.
                required_limits: wgpu::Limits::downlevel_webgl2_defaults(),
                memory_hints: Default::default(),
                trace: wgpu::Trace::Off,
            })
            .await?;

        let surface_caps = surface.get_capabilities(&adapter);
        // Shader code in this tutorial assumes an sRGB surface texture. Using a different
        // one will result in all the colors coming out darker. If you want to support non
        // sRGB surfaces, you'll need to account for that when drawing to the frame.
        let surface_format = surface_caps
            .formats
            .iter()
            .find(|f| f.is_srgb())
            .copied()
            .unwrap_or(surface_caps.formats[0]);
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };

        Ok(Self {
            surface,
            device,
            queue,
            config,
            is_surface_configured: false,
            window,
        })
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        if width > 0 && height > 0 {
            self.config.width = width;
            self.config.height = height;
            self.surface.configure(&self.device, &self.config);
            self.is_surface_configured = true;
        }
        log::trace!("Resized ({width}x{height})")
    }

    pub fn handle_key(&self, event_loop: &ActiveEventLoop, code: KeyCode, is_pressed: bool) {
        match (code, is_pressed) {
            (KeyCode::Escape, true) => event_loop.exit(),
            _ => {}
        }
    }

    fn update(&mut self) {
        // remove `todo!()`
    }

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        self.window.request_redraw();

        // We can't render unless the surface is configured
        if !self.is_surface_configured {
            return Ok(());
        }

        let output = self.surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });
        {
            let _render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.1,
                            g: 0.2,
                            b: 0.3,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                    depth_slice: None,
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
            });
        }

        // submit will accept anything that implements IntoIter
        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}

pub struct App {
    proxy: Option<winit::event_loop::EventLoopProxy<State>>,
    state: Option<State>,
}

impl App {
    pub fn new(event_loop: &EventLoop<State>) -> Self {
        let proxy = Some(event_loop.create_proxy());
        Self { state: None, proxy }
    }
}

impl ApplicationHandler<State> for App {
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
                            State::new(window)
                                .await
                                .expect("Unable to create canvas!!!")
                        )
                        .is_ok()
                )
            });
        }
    }

    fn user_event(&mut self, _event_loop: &ActiveEventLoop, mut event: State) {
        // This is where proxy.send_event() ends up
        event.window.request_redraw();
        event.resize(
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
            Some(canvas) => canvas,
            None => return,
        };

        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::Resized(size) => state.resize(size.width, size.height),
            WindowEvent::RedrawRequested => {
                state.render();
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
    }
}
