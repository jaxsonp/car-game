use wgpu::{RenderPass, RenderPipeline};

use crate::{DepthTexture, Renderable, load_obj, mesh, model::Model};

pub struct RenderScene {
    render_pipeline: RenderPipeline,
    earth: Model,
    car: Model,
}

impl RenderScene {
    pub fn new(
        device: &wgpu::Device,
        config: &wgpu::SurfaceConfiguration,
        per_pass_bind_group_layout: &wgpu::BindGroupLayout,
    ) -> RenderScene {
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("scene shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shaders/scene.wgsl").into()),
        });

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("scene render pipeline layout"),
                bind_group_layouts: &[
                    &per_pass_bind_group_layout,
                    &mesh::Mesh::get_bind_group_layout(&device),
                ],
                push_constant_ranges: &[],
            });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("scene render pipeline"),
            layout: Some(&render_pipeline_layout),
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
        });

        RenderScene {
            render_pipeline,
            car: Model::from_raw("Car", load_obj!("car.obj"), device),
            earth: Model::from_raw("Test cube", load_obj!("earth.obj"), device),
        }
    }
}
impl Renderable for RenderScene {
    fn get_render_pipeline(&self) -> &wgpu::RenderPipeline {
        &self.render_pipeline
    }

    fn render(&mut self, render_pass: &mut wgpu::RenderPass) {
        self.car.render(render_pass);
        self.earth.render(render_pass);
    }
}
