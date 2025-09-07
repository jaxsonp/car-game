mod camera;
#[cfg(debug_assertions)]
pub mod debug;
pub mod mesh;
mod model;
mod shadows;

use nalgebra::{Isometry3, Rotation3, Translation, Vector3};
use utils::*;
use wgpu::{
    BindGroupDescriptor, BindGroupEntry, BindGroupLayoutDescriptor, BindGroupLayoutEntry,
    BindingResource, BindingType, BufferBindingType, BufferDescriptor, BufferUsages, Queue,
    RenderPipeline, ShaderStages,
    util::{BufferInitDescriptor, DeviceExt},
};

use crate::{DepthTexture, uniforms::Vector3Uniform};
use camera::{CameraUniformMatrix, get_view_projection_matrix};
#[cfg(debug_assertions)]
use debug::DebugLineVertex;
use model::Model;
use shadows::{SUN_DIR, ShadowMapper};

pub struct Scene {
    mesh_render_pipeline: RenderPipeline,
    #[cfg(debug_assertions)]
    debug_render_pipeline: RenderPipeline,

    camera_buffer: wgpu::Buffer,
    scene_bind_group: wgpu::BindGroup,

    pub shadow_mapper: ShadowMapper,
    pub camera: Camera,
    pub static_models: Vec<Model>,
    pub car: Model,
    pub wheels: [Model; 4],
}

impl Scene {
    pub fn new(device: &wgpu::Device, config: &wgpu::SurfaceConfiguration) -> Scene {
        let scene_bind_group_layout = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: Some("scene bind group layout"),
            entries: &[
                BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStages::VERTEX,
                    ty: BindingType::Buffer {
                        ty: BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                BindGroupLayoutEntry {
                    binding: 1,
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Buffer {
                        ty: BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                BindGroupLayoutEntry {
                    binding: 2,
                    visibility: ShaderStages::VERTEX,
                    ty: BindingType::Buffer {
                        ty: BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 3,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        multisampled: false,
                        view_dimension: wgpu::TextureViewDimension::D2,
                        sample_type: wgpu::TextureSampleType::Depth,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 4,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Comparison),
                    count: None,
                },
            ],
        });

        let sun_dir_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("scene sun dir buffer"),
            contents: bytemuck::cast_slice(&Vector3Uniform::from(SUN_DIR.normalize()).get_slice()),
            usage: BufferUsages::UNIFORM,
        });

        let camera = Camera::new(
            [8.0, 4.0, 4.0],
            [0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            config.width as f32,
            config.height as f32,
        );
        let camera_buffer = device.create_buffer(&BufferDescriptor {
            label: Some("Camera Buffer"),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            size: size_of::<CameraUniformMatrix>() as u64,
            mapped_at_creation: false,
        });
        let shadow_mapper = ShadowMapper::new(device);

        let scene_bind_group = device.create_bind_group(&BindGroupDescriptor {
            label: Some("scene bind group"),
            layout: &scene_bind_group_layout,
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: camera_buffer.as_entire_binding(),
                },
                BindGroupEntry {
                    binding: 1,
                    resource: sun_dir_buffer.as_entire_binding(),
                },
                BindGroupEntry {
                    binding: 2,
                    resource: shadow_mapper.view_proj_buffer.as_entire_binding(),
                },
                BindGroupEntry {
                    binding: 3,
                    resource: BindingResource::TextureView(&shadow_mapper.texture_view),
                },
                BindGroupEntry {
                    binding: 4,
                    resource: BindingResource::Sampler(&shadow_mapper.texture_sampler),
                },
            ],
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

        #[cfg(debug_assertions)]
        let debug_render_pipeline = {
            let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some("scene debug shader"),
                source: wgpu::ShaderSource::Wgsl(include_str!("../shaders/debuglines.wgsl").into()),
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

        let car = Model::from_object::<assets::objects::Car>("Car", device, None);
        let wheels = [0, 1, 2, 3].map(|i| {
            Model::from_object::<assets::objects::Wheel>(
                format!("Wheel {}", i).as_str(),
                device,
                Some(Isometry3::from_parts(
                    Translation::identity(),
                    Rotation3::from_axis_angle(
                        &Vector3::z_axis(),
                        if i % 2 == 0 {
                            -std::f32::consts::FRAC_PI_2
                        } else {
                            std::f32::consts::FRAC_PI_2
                        },
                    )
                    .into(),
                )),
            )
        });
        let static_models: Vec<Model> = vec![
            Model::from_object::<assets::objects::Ground>("Ground", device, None),
            Model::from_object::<assets::objects::Roads>("Roads", device, None),
            Model::from_object::<assets::objects::WorldDecor>("Decor", device, None),
            Model::from_object::<assets::objects::Ocean>("Ocean", device, None),
        ];

        Scene {
            mesh_render_pipeline,
            #[cfg(debug_assertions)]
            debug_render_pipeline,
            camera_buffer,
            scene_bind_group,

            shadow_mapper,
            static_models,
            car,
            wheels,
            camera,
        }
    }

    pub fn prepare(&mut self, queue: &Queue, snapshot: &RenderSnapshot) {
        queue.write_buffer(
            &self.camera_buffer,
            0,
            bytemuck::cast_slice(&[get_view_projection_matrix(&self.camera)]),
        );

        self.shadow_mapper
            .prepare(queue, snapshot.car_transform.translation.vector.into());

        self.wheels.iter_mut().enumerate().for_each(|(i, w)| {
            w.set_transform(snapshot.wheel_transforms[i]);
        });

        self.car.set_transform(snapshot.car_transform);

        self.car.prepare(queue);
        self.wheels.iter_mut().for_each(|w| w.prepare(queue));
        self.static_models.iter_mut().for_each(|m| m.prepare(queue));
    }

    pub fn render<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>) {
        render_pass.set_bind_group(0, &self.scene_bind_group, &[]);
        render_pass.set_pipeline(&self.mesh_render_pipeline);

        self.car.render(render_pass);
        self.wheels.iter().for_each(|w| w.render(render_pass));
        self.static_models
            .iter()
            .for_each(|m| m.render(render_pass));

        #[cfg(debug_assertions)]
        {
            render_pass.set_pipeline(&self.debug_render_pipeline);
            self.car.render_debug_lines(render_pass);
            self.wheels
                .iter()
                .for_each(|w| w.render_debug_lines(render_pass));
            self.static_models
                .iter()
                .for_each(|m| m.render_debug_lines(render_pass));
        }
    }

    pub fn shadow_map_render<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>) {
        self.car.shadow_map_render(render_pass);
        self.wheels
            .iter()
            .for_each(|w| w.shadow_map_render(render_pass));
        self.static_models
            .iter()
            .for_each(|m| m.shadow_map_render(render_pass));
    }
}
