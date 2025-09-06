mod gui;
mod scene;
mod uniforms;

use std::sync::Arc;

use utils::*;
use wasm_bindgen::prelude::*;
use wgpu::RequestAdapterOptions;
use winit::{event::WindowEvent, window::Window};

use gui::GuiOverlay;
use scene::Scene;

/// Main rendering object
pub struct RenderState {
    surface: wgpu::Surface<'static>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    is_surface_configured: bool,
    depth_texture: DepthTexture,

    pub scene: Scene,
    pub gui: GuiOverlay,

    // needs to be last
    pub window: Arc<Window>,
}

impl RenderState {
    pub async fn new(window: Arc<Window>) -> anyhow::Result<Self> {
        let size = window.inner_size();

        // choose webgpu if available, else webgl
        let instance = wgpu::util::new_instance_with_webgpu_detection(&wgpu::InstanceDescriptor {
            backends: wgpu::Backends::BROWSER_WEBGPU | wgpu::Backends::GL,
            ..Default::default()
        })
        .await;

        let surface = instance
            .create_surface(window.clone())
            .expect_throw("failed to create surface handle");

        let adapter = instance
            .request_adapter(&RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::None,
                force_fallback_adapter: false,
                compatible_surface: Some(&surface),
            })
            .await
            .expect_throw("no suitable adapter found");

        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor {
                label: None,
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::downlevel_webgl2_defaults(),
                memory_hints: Default::default(),
                trace: wgpu::Trace::Off,
            })
            .await
            .expect_throw("failed to obtain device");

        let surface_caps = surface.get_capabilities(&adapter);

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

        let depth_texture = DepthTexture::new(&device, &config);

        let scene = Scene::new(&device, &config);
        let gui = GuiOverlay::new(&device, &config);

        Ok(Self {
            surface,
            device,
            queue,
            config,
            is_surface_configured: false,
            depth_texture,
            scene,
            gui,
            window,
        })
    }

    pub fn handle_resize(&mut self, width: u32, height: u32) {
        log::debug!("Resized ({width}x{height})");
        if width > 0 && height > 0 {
            self.config.width = width;
            self.config.height = height;

            self.surface.configure(&self.device, &self.config);
            self.is_surface_configured = true;
            self.depth_texture = DepthTexture::new(&self.device, &self.config);

            // update camera
            self.scene.camera.resize(width, height);
        }

        // gui text brush needs to know screen size
        self.gui.handle_resize(width, height, &self.queue);
    }

    pub fn handle_window_event(&mut self, _event: &WindowEvent) {}

    pub fn render(&mut self, snapshot: Option<RenderSnapshot>) -> Result<(), wgpu::SurfaceError> {
        self.window.request_redraw();

        if !self.is_surface_configured || snapshot.is_none() {
            return Ok(());
        }
        let snapshot = snapshot.unwrap();

        // preparation -----

        if let Some(text) = snapshot.debug_string.as_ref() {
            self.gui.debug_text.change_text(text);
        }

        self.gui.prepare(&self.device, &self.queue);
        self.scene.prepare(&self.queue, &snapshot);

        let output = self.surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        // rendering -----
        {
            // shadow map pass
            let mut render_pass: wgpu::RenderPass =
                encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some("shadow map render pass"),
                    color_attachments: &[],
                    depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                        view: &self.scene.shadow_mapper.texture_view,
                        depth_ops: Some(wgpu::Operations {
                            load: wgpu::LoadOp::Clear(1.0),
                            store: wgpu::StoreOp::Store,
                        }),
                        stencil_ops: None,
                    }),
                    timestamp_writes: None,
                    occlusion_query_set: None,
                });
            render_pass.set_pipeline(&self.scene.shadow_mapper.render_pipeline);
            render_pass.set_bind_group(0, &self.scene.shadow_mapper.bind_group, &[]);
            self.scene.shadow_map_render(&mut render_pass);
        }

        {
            // 3d render pass
            let mut render_pass: wgpu::RenderPass =
                encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some("3D render Pass"),
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view: &view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color::WHITE),
                            store: wgpu::StoreOp::Store,
                        },
                        depth_slice: None, // wgpu 26 feature
                    })],
                    depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                        view: &self.depth_texture.view,
                        depth_ops: Some(wgpu::Operations {
                            load: wgpu::LoadOp::Clear(1.0),
                            store: wgpu::StoreOp::Store,
                        }),
                        stencil_ops: None,
                    }),
                    timestamp_writes: None,
                    occlusion_query_set: None,
                });

            self.scene.render(&mut render_pass);
        }

        {
            // overlay render pass
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("overlay render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Load, // dont overwrite
                        store: wgpu::StoreOp::Store,
                    },
                    depth_slice: None, // wgpu 26 feature
                })],
                depth_stencil_attachment: None, // no depth buffer
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            self.gui.render(&mut render_pass);
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}

struct DepthTexture {
    view: wgpu::TextureView,
}
impl DepthTexture {
    const TEXTURE_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Depth32Float;

    pub fn new(device: &wgpu::Device, config: &wgpu::SurfaceConfiguration) -> DepthTexture {
        let size = wgpu::Extent3d {
            width: config.width.max(1),
            height: config.height.max(1),
            depth_or_array_layers: 1,
        };
        let descriptor = wgpu::TextureDescriptor {
            label: Some("Depth texture"),
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: Self::TEXTURE_FORMAT,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        };
        let texture = device.create_texture(&descriptor);

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        Self { view }
    }
}
