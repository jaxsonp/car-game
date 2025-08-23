pub mod debug;
pub mod mesh;
mod model;

use assets::GameObject;
use nalgebra::Vector3;
use wgpu::{
    BindGroupDescriptor, BindGroupEntry, BindGroupLayoutDescriptor, BindGroupLayoutEntry,
    BindingType, BufferBindingType, BufferDescriptor, BufferUsages, Queue, RenderPipeline,
    ShaderStages,
    util::{BufferInitDescriptor, DeviceExt},
};

use crate::{
    DepthTexture,
    camera::{Camera, CameraUniformMatrix},
};
use debug::{DebugLineGroup, DebugLineVertex};
use model::Model;

pub struct Scene {
    mesh_render_pipeline: RenderPipeline,
    debug_render_pipeline: RenderPipeline,

    camera_buffer: wgpu::Buffer,
    scene_bind_group: wgpu::BindGroup,

    pub camera: Camera,
    pub floor: Model,
    pub car: Model,
    static_debug_lines: Vec<DebugLineGroup>,
}

impl Scene {
    pub fn new(device: &wgpu::Device, config: &wgpu::SurfaceConfiguration) -> Scene {
        let camera = Camera::new([5.0, 1.0, 2.0], [0.0, 0.0, 0.0], [0.0, 1.0, 0.0], &config);
        let camera_buffer = device.create_buffer(&BufferDescriptor {
            label: Some("Camera Buffer"),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            size: size_of::<CameraUniformMatrix>() as u64,
            mapped_at_creation: false,
        });

        let scene_bind_group_layout = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: Some("scene bind group layout"),
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
        let scene_bind_group = device.create_bind_group(&BindGroupDescriptor {
            label: Some("scene bind group"),
            layout: &scene_bind_group_layout,
            entries: &[BindGroupEntry {
                binding: 0,
                resource: camera_buffer.as_entire_binding(),
            }],
        });

        let mesh_render_pipeline = {
            let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some("scene mesh shader"),
                source: wgpu::ShaderSource::Wgsl(include_str!("../shaders/scene.wgsl").into()),
            });
            let mesh_render_pipeline_layout =
                device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: Some("scene mesh render pipeline layout"),
                    bind_group_layouts: &[
                        &scene_bind_group_layout,
                        &model::Model::get_bind_group_layout(&device),
                        &mesh::Mesh::get_bind_group_layout(&device),
                    ],
                    push_constant_ranges: &[],
                });
            device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("scene mesh render pipeline"),
                layout: Some(&mesh_render_pipeline_layout),
                vertex: wgpu::VertexState {
                    module: &shader,
                    entry_point: None,
                    buffers: &[mesh::Vertex::BUFFER_LAYOUT],
                    compilation_options: wgpu::PipelineCompilationOptions::default(),
                },
                fragment: Some(wgpu::FragmentState {
                    module: &shader,
                    entry_point: None,
                    targets: &[Some(wgpu::ColorTargetState {
                        format: config.format,
                        blend: Some(wgpu::BlendState::REPLACE),
                        write_mask: wgpu::ColorWrites::ALL,
                    })],
                    compilation_options: wgpu::PipelineCompilationOptions::default(),
                }),
                primitive: wgpu::PrimitiveState {
                    topology: wgpu::PrimitiveTopology::TriangleList,
                    strip_index_format: None,
                    front_face: wgpu::FrontFace::Ccw,
                    cull_mode: Some(wgpu::Face::Back),
                    polygon_mode: wgpu::PolygonMode::Fill,
                    unclipped_depth: false,
                    conservative: false,
                },
                depth_stencil: Some(wgpu::DepthStencilState {
                    format: DepthTexture::TEXTURE_FORMAT,
                    depth_write_enabled: true,
                    depth_compare: wgpu::CompareFunction::Less,
                    stencil: wgpu::StencilState::default(),
                    bias: wgpu::DepthBiasState::default(),
                }),
                multisample: wgpu::MultisampleState {
                    count: 1,
                    mask: !0,
                    alpha_to_coverage_enabled: false,
                },
                multiview: None,
                cache: None,
            })
        };

        let debug_render_pipeline = {
            let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some("scene debug shader"),
                source: wgpu::ShaderSource::Wgsl(include_str!("../shaders/debug.wgsl").into()),
            });

            let render_pipeline_layout =
                device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: Some("scene debug pipeline layout"),
                    bind_group_layouts: &[
                        &scene_bind_group_layout,
                        &model::Model::get_bind_group_layout(&device),
                    ],
                    push_constant_ranges: &[],
                });

            device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("scene debug render pipeline"),
                layout: Some(&render_pipeline_layout),
                vertex: wgpu::VertexState {
                    module: &shader,
                    entry_point: None,
                    buffers: &[DebugLineVertex::BUFFER_LAYOUT],
                    compilation_options: wgpu::PipelineCompilationOptions::default(),
                },
                fragment: Some(wgpu::FragmentState {
                    module: &shader,
                    entry_point: None,
                    targets: &[Some(wgpu::ColorTargetState {
                        format: config.format,
                        blend: Some(wgpu::BlendState::REPLACE),
                        write_mask: wgpu::ColorWrites::ALL,
                    })],
                    compilation_options: wgpu::PipelineCompilationOptions::default(),
                }),
                primitive: wgpu::PrimitiveState {
                    topology: wgpu::PrimitiveTopology::LineList,
                    strip_index_format: None,
                    front_face: wgpu::FrontFace::Ccw,
                    cull_mode: None,
                    polygon_mode: wgpu::PolygonMode::Fill,
                    unclipped_depth: false,
                    conservative: false,
                },
                depth_stencil: Some(wgpu::DepthStencilState {
                    format: DepthTexture::TEXTURE_FORMAT,
                    depth_write_enabled: true,
                    depth_compare: wgpu::CompareFunction::Less,
                    stencil: wgpu::StencilState::default(),
                    bias: wgpu::DepthBiasState::default(),
                }),
                multisample: wgpu::MultisampleState {
                    count: 1,
                    mask: !0,
                    alpha_to_coverage_enabled: false,
                },
                multiview: None,
                cache: None,
            })
        };

        Scene {
            mesh_render_pipeline,
            debug_render_pipeline,
            camera_buffer,
            scene_bind_group,

            camera,
            car: Model::from_object::<assets::objects::Car>("Car", device),
            floor: Model::from_object::<assets::objects::Floor>("Floor", device),
            static_debug_lines: vec![
                DebugLineGroup::from_raw(device, assets::objects::Floor::debug_lines),
                // TODO make this not static
                DebugLineGroup::from_raw(device, assets::objects::Car::debug_lines),
            ],
        }
    }

    pub fn prepare(&self, queue: &Queue) {
        queue.write_buffer(
            &self.camera_buffer,
            0,
            bytemuck::cast_slice(&[self.camera.get_view_projection_matrix()]),
        );
    }

    pub fn render<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>) {
        render_pass.set_bind_group(0, &self.scene_bind_group, &[]);

        render_pass.set_pipeline(&self.mesh_render_pipeline);
        self.car.render(render_pass);
        self.floor.render(render_pass);

        render_pass.set_pipeline(&self.debug_render_pipeline);
        for line_group in self.static_debug_lines.iter() {
            line_group.render(render_pass);
        }
        self.car.render_debug_lines(render_pass);
        self.floor.render_debug_lines(render_pass);
    }
}
