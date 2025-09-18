use car_game_utils::RenderSnapshot;
use wgpu::{
    AddressMode, BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout,
    BindGroupLayoutDescriptor, BindGroupLayoutEntry, BindingResource, BindingType, Buffer,
    BufferBindingType, BufferDescriptor, BufferUsages, Extent3d, PipelineCompilationOptions,
    PipelineLayoutDescriptor, Queue, RenderPass, RenderPipeline, RenderPipelineDescriptor, Sampler,
    ShaderStages, SurfaceConfiguration, VertexState,
    util::{BufferInitDescriptor, DeviceExt},
};

use crate::{DepthTexture, uniforms::Vector3Uniform};

pub struct Ocean {
    bind_group: BindGroup,
    bind_group_layout: BindGroupLayout,
    render_pipeline: RenderPipeline,
    index_buffer: Buffer,
    water_level_and_size_buffer: Buffer,
    depth_texture_sampler: Sampler,
}
impl Ocean {
    pub fn new(
        device: &wgpu::Device,
        config: &SurfaceConfiguration,
        scene_bind_group_layout: &BindGroupLayout,
        depth_texture: &DepthTexture,
    ) -> Self {
        let water_level_and_size_buffer = device.create_buffer(&BufferDescriptor {
            label: Some("ocean rendering water level buffer"),
            usage: BufferUsages::UNIFORM.union(BufferUsages::COPY_DST),
            size: size_of::<Vector3Uniform>() as u64,
            mapped_at_creation: false,
        });

        let indices: [u16; 24] = [
            0, 1, 4, 0, 4, 3, 1, 2, 5, 1, 5, 4, 3, 4, 7, 3, 7, 6, 4, 5, 8, 4, 8, 7,
        ];
        let index_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("ocean index buffer"),
            contents: bytemuck::cast_slice(&indices),
            usage: BufferUsages::INDEX,
        });

        let bind_group_layout = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: Some("ocean renderer bind group layout"),
            entries: &[
                BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStages::VERTEX | ShaderStages::FRAGMENT,
                    ty: BindingType::Buffer {
                        ty: BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        multisampled: false,
                        view_dimension: wgpu::TextureViewDimension::D2,
                        sample_type: wgpu::TextureSampleType::Depth,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 2,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Comparison),
                    count: None,
                },
            ],
        });

        let depth_texture_sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("depth texture sampler"),
            address_mode_u: AddressMode::Repeat,
            address_mode_v: AddressMode::Repeat,
            address_mode_w: AddressMode::Repeat,
            mag_filter: wgpu::FilterMode::Nearest,
            min_filter: wgpu::FilterMode::Nearest,
            compare: Some(wgpu::CompareFunction::Greater),
            ..Default::default()
        });

        let bind_group = device.create_bind_group(&BindGroupDescriptor {
            label: Some("ocean renderer bind group"),
            layout: &bind_group_layout,
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: water_level_and_size_buffer.as_entire_binding(),
                },
                BindGroupEntry {
                    binding: 1,
                    resource: BindingResource::TextureView(&depth_texture.view),
                },
                BindGroupEntry {
                    binding: 2,
                    resource: BindingResource::Sampler(&depth_texture_sampler),
                },
            ],
        });

        let render_pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some("ocean render pipeline layout"),
            bind_group_layouts: &[scene_bind_group_layout, &bind_group_layout],
            push_constant_ranges: &[],
        });

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("ocean shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("../shaders/ocean.wgsl").into()),
        });

        let render_pipeline = device.create_render_pipeline(&RenderPipelineDescriptor {
            label: Some("ocean render pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: VertexState {
                module: &shader,
                entry_point: None,
                compilation_options: PipelineCompilationOptions::default(),
                buffers: &[],
            },
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: None,
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: None,
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            }),
            // handled manually
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
            cache: None,
        });

        Ocean {
            bind_group,
            bind_group_layout,
            render_pipeline,
            water_level_and_size_buffer,
            index_buffer,
            depth_texture_sampler,
        }
    }

    pub fn handle_resize(&mut self, device: &wgpu::Device, depth_texture: &DepthTexture) {
        self.bind_group = device.create_bind_group(&BindGroupDescriptor {
            label: Some("ocean renderer bind group"),
            layout: &self.bind_group_layout,
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: self.water_level_and_size_buffer.as_entire_binding(),
                },
                BindGroupEntry {
                    binding: 1,
                    resource: BindingResource::TextureView(&depth_texture.view),
                },
                BindGroupEntry {
                    binding: 2,
                    resource: BindingResource::Sampler(&self.depth_texture_sampler),
                },
            ],
        });
    }

    pub fn prepare(&mut self, queue: &Queue, render_snapshot: &RenderSnapshot, size: Extent3d) {
        queue.write_buffer(
            &self.water_level_and_size_buffer,
            0,
            bytemuck::cast_slice(
                &Vector3Uniform::from([
                    render_snapshot.water_level,
                    size.width as f32,
                    size.height as f32,
                ])
                .get_slice(),
            ),
        );
    }

    pub fn render(&self, render_pass: &mut RenderPass) {
        render_pass.set_pipeline(&self.render_pipeline);
        render_pass.set_bind_group(1, &self.bind_group, &[]);
        render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        render_pass.draw_indexed(0..24, 0, 0..1);
    }
}
