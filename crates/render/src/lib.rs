pub mod camera;
mod debug;
mod gui;
mod macros;
mod material;
mod mesh;
mod model;
mod scene;

use std::sync::Arc;

use wasm_bindgen::prelude::*;
use wgpu::{
    BindGroupDescriptor, BindGroupLayoutDescriptor, BindGroupLayoutEntry, BindingType,
    BufferBindingType, BufferDescriptor, RequestAdapterOptions, ShaderStages,
};
use winit::{event::WindowEvent, window::Window};

use crate::{camera::Camera, debug::DebugLines, gui::GuiOverlay, scene::RenderScene};

/// Main rendering object
pub struct RenderState {
    surface: wgpu::Surface<'static>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    is_surface_configured: bool,

    pub camera: Camera,
    pub scene: RenderScene,
    pub debug_lines: DebugLines,
    pub gui: GuiOverlay,

    depth_texture: DepthTexture,
    /// Per render pass bind group
    per_pass_bind_group: wgpu::BindGroup,
    camera_buffer: wgpu::Buffer,

    // needs to be last
    pub window: Arc<Window>,
}

impl RenderState {
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

        let surface = instance.create_surface(window.clone()).unwrap_throw();

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

        let camera = Camera::new([5.0, 1.0, 2.0], [0.0, 0.0, 0.0], [0.0, 1.0, 0.0], &config);

        let camera_buffer = device.create_buffer(&BufferDescriptor {
            label: Some("Camera Buffer"),
            //contents: bytemuck::cast_slice(&[camera_uniform]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            size: size_of::<camera::CameraUniformMatrix>() as u64,
            mapped_at_creation: false,
        });

        let per_pass_bind_group_layout =
            device.create_bind_group_layout(&BindGroupLayoutDescriptor {
                label: Some("per-pass bind group layout"),
                entries: &[BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStages::VERTEX,
                    ty: BindingType::Buffer {
                        ty: BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
            });

        let per_pass_bind_group = device.create_bind_group(&BindGroupDescriptor {
            label: Some("per-pass bind group"),
            layout: &per_pass_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: camera_buffer.as_entire_binding(),
            }],
        });

        let depth_texture = DepthTexture::new(&device, &config);

        let scene = RenderScene::new(&device, &config, &per_pass_bind_group_layout);
        let debug_lines = DebugLines::new(&device, &config, &per_pass_bind_group_layout);
        let gui = GuiOverlay::new(&device, &config);

        Ok(Self {
            surface,
            device,
            queue,
            config,
            is_surface_configured: false,
            window,
            camera,
            scene,
            debug_lines,
            gui,
            depth_texture,
            per_pass_bind_group,
            camera_buffer,
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
            self.camera.handle_resize(width, height);
        }

        // gui text brush needs to know screen size
        self.gui.handle_resize(width, height, &self.queue);
    }

    pub fn handle_window_event(&mut self, _event: &WindowEvent) {}

    pub fn render<'render>(&mut self) -> Result<(), wgpu::SurfaceError> {
        self.window.request_redraw();

        // We can't render unless the surface is configured
        if !self.is_surface_configured {
            return Ok(());
        }

        // preparation -----

        self.queue.write_buffer(
            &self.camera_buffer,
            0,
            bytemuck::cast_slice(&[self.camera.get_view_projection_matrix()]),
        );

        self.scene.prepare(&self.device, &self.queue);
        self.debug_lines.prepare(&self.device, &self.queue);
        self.gui.prepare(&self.device, &self.queue);

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

            render_pass.set_bind_group(0, &self.per_pass_bind_group, &[]);

            self.scene.render(&mut render_pass);
            self.debug_lines.render(&mut render_pass);
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
    #[allow(dead_code)]
    texture: wgpu::Texture,
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
        Self { texture, view }
    }
}

trait RenderPhase {
    fn render<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>);
    fn prepare(&mut self, device: &wgpu::Device, queue: &wgpu::Queue);
}
