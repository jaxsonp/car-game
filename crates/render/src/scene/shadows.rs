use nalgebra::{Matrix4, Orthographic3, Point3, Vector3};
use wgpu::{
    BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayoutDescriptor,
    BindGroupLayoutEntry, BindingType, Buffer, BufferBindingType, BufferDescriptor,
    CompareFunction, Device, Extent3d, PipelineCompilationOptions, PipelineLayoutDescriptor, Queue,
    RenderPipeline, Sampler, ShaderStages, TextureView,
};

use crate::scene::model::Model;

pub struct ShadowMapper {
    pub view_proj_buffer: Buffer,

    pub render_pipeline: RenderPipeline,
    pub bind_group: BindGroup,

    pub texture_view: TextureView,
    pub texture_sampler: Sampler,
}
impl ShadowMapper {
    const SHADOW_MAP_DIM: u32 = 2048;
    pub const SUN_DIR: Vector3<f32> = Vector3::new(-1.0, 2.0, -1.0);
    const TEX_SIZE: Extent3d = Extent3d {
        width: Self::SHADOW_MAP_DIM,
        height: Self::SHADOW_MAP_DIM,
        depth_or_array_layers: 1,
    };

    pub fn new(device: &Device) -> ShadowMapper {
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("shadow map texture"),
            size: Self::TEX_SIZE,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Depth32Float,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        });
        let texture_view = texture.create_view(&wgpu::TextureViewDescriptor::default());

        let texture_sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("shadow map sampler"),
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            compare: Some(CompareFunction::LessEqual),
            ..Default::default()
        });

        let bind_group_layout = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: Some("shadow map scene bind group layout"),
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

        let view_proj_buffer = device.create_buffer(&BufferDescriptor {
            label: Some("shadow map view proj buffer"),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            size: size_of::<ViewProjUniform>() as u64,
            mapped_at_creation: false,
        });

        let bind_group = device.create_bind_group(&BindGroupDescriptor {
            label: Some("scene bind group"),
            layout: &bind_group_layout,
            entries: &[BindGroupEntry {
                binding: 0,
                resource: view_proj_buffer.as_entire_binding(),
            }],
        });

        let render_pipeline = {
            let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some("shadow map shader"),
                source: wgpu::ShaderSource::Wgsl(
                    include_str!("../shaders/shadow_pass.wgsl").into(),
                ),
            });

            let pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
                label: Some("shadow map pipeline layout"),
                bind_group_layouts: &[&bind_group_layout, &Model::get_bind_group_layout(device)],
                push_constant_ranges: &[],
            });

            device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("shadow map rendering pipeline"),
                layout: Some(&pipeline_layout),
                vertex: wgpu::VertexState {
                    module: &shader,
                    entry_point: None,
                    buffers: &[crate::scene::mesh::Vertex::BUFFER_LAYOUT],
                    compilation_options: PipelineCompilationOptions::default(),
                },
                fragment: None, // no frag shader
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
                    format: wgpu::TextureFormat::Depth32Float,
                    depth_write_enabled: true,
                    depth_compare: wgpu::CompareFunction::Less,
                    stencil: wgpu::StencilState::default(),
                    bias: wgpu::DepthBiasState {
                        // preventative
                        constant: 2,
                        slope_scale: 2.0,
                        clamp: 0.0,
                    },
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
        ShadowMapper {
            view_proj_buffer,
            bind_group,
            texture_view,
            texture_sampler,
            render_pipeline,
        }
    }

    pub fn prepare(&mut self, queue: &Queue, car_pos: Point3<f32>) {
        let view_proj: ViewProjUniform = Self::get_view_projection_matrix(car_pos).into();
        queue.write_buffer(
            &self.view_proj_buffer,
            0,
            bytemuck::cast_slice(&view_proj.matrix),
        );
    }

    fn get_view_projection_matrix(car_pos: Point3<f32>) -> Matrix4<f32> {
        let view = Matrix4::look_at_rh(
            &(car_pos + (Self::SUN_DIR * 300.0)),
            &car_pos,
            &Vector3::y(),
        );
        const SIZE: f32 = 90.0;
        let proj = Orthographic3::new(-SIZE, SIZE, -SIZE, SIZE, 100.0, 700.0).to_homogeneous();
        return OPENGL_TO_WGPU_MATRIX * proj * view;
    }
}

#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
struct ViewProjUniform {
    matrix: [[f32; 4]; 4],
}
impl From<Matrix4<f32>> for ViewProjUniform {
    #[rustfmt::skip]
    fn from(m: Matrix4<f32>) -> Self {
        ViewProjUniform {
            matrix: [
                [m[0], m[1], m[2], m[3]],
                [m[4], m[5], m[6], m[7]],
                [m[8], m[9], m[10], m[11]],
                [m[12], m[13], m[14], m[15]],
            ],
        }
    }
}

#[rustfmt::skip]
pub const OPENGL_TO_WGPU_MATRIX: Matrix4<f32> = Matrix4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.0,
    0.0, 0.0, 0.5, 1.0,
);
