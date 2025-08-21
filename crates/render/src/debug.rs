use wgpu::{
    BufferUsages, RenderPass,
    util::{BufferInitDescriptor, DeviceExt},
};

use crate::{DepthTexture, RenderPhase};

const RED: [f32; 3] = [1.0, 0.0, 0.0];
const GREEN: [f32; 3] = [0.0, 1.0, 0.0];
const BLUE: [f32; 3] = [0.0, 0.0, 1.0];
const GRAY: [f32; 3] = [0.5, 0.5, 0.5];

pub struct DebugLines {
    render_pipeline: wgpu::RenderPipeline,
    origin: DebugLineGroup,
}
impl DebugLines {
    pub fn new(
        device: &wgpu::Device,
        config: &wgpu::SurfaceConfiguration,
        per_pass_bind_group_layout: &wgpu::BindGroupLayout,
    ) -> DebugLines {
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("debug shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shaders/debug.wgsl").into()),
        });

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("debug render pipeline layout"),
                bind_group_layouts: &[&per_pass_bind_group_layout],
                push_constant_ranges: &[],
            });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("debug render pipeline"),
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
        });

        let origin = DebugLineGroup::new(device, ORIGIN_LINES);

        DebugLines {
            origin,
            render_pipeline,
        }
    }
}
impl RenderPhase for DebugLines {
    fn render<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>) {
        render_pass.set_pipeline(&self.render_pipeline);
        self.origin.render(render_pass);
    }

    fn prepare(&mut self, _device: &wgpu::Device, _queue: &wgpu::Queue) {}
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct DebugLineVertex {
    pub pos: [f32; 3],
    pub col: [f32; 3],
}
impl DebugLineVertex {
    pub const BUFFER_LAYOUT: wgpu::VertexBufferLayout<'static> = wgpu::VertexBufferLayout {
        array_stride: std::mem::size_of::<DebugLineVertex>() as wgpu::BufferAddress,
        step_mode: wgpu::VertexStepMode::Vertex,
        attributes: &wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x3],
    };
}

struct DebugLineGroup {
    vertex_buffer: wgpu::Buffer,
    n_verts: u32,
}
impl DebugLineGroup {
    pub fn new(device: &wgpu::Device, verts: &[DebugLineVertex]) -> DebugLineGroup {
        let vertex_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("debug line group vertex buffer"),
            contents: bytemuck::cast_slice(verts),
            usage: BufferUsages::VERTEX,
        });
        Self {
            vertex_buffer,
            n_verts: verts.len() as u32,
        }
    }

    fn render(&self, render_pass: &mut RenderPass) {
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.draw(0..(self.n_verts), 0..1);
    }
}

#[rustfmt::skip]
const ORIGIN_LINES: &[DebugLineVertex] = &[
    DebugLineVertex { pos: [0.0, 0.0, 0.0], col: RED, }, // x
    DebugLineVertex { pos: [1.0, 0.0, 0.0], col: RED, },
    DebugLineVertex { pos: [0.0, 0.0, 0.0], col: GRAY, }, // -x
    DebugLineVertex { pos: [-1.0, 0.0, 0.0], col: GRAY, },
    DebugLineVertex { pos: [0.0, 0.0, 0.0], col: GREEN, }, // y
    DebugLineVertex { pos: [0.0, 1.0, 0.0], col: GREEN, },
    DebugLineVertex { pos: [0.0, 0.0, 0.0], col: GRAY, }, // -y
    DebugLineVertex { pos: [0.0, -1.0, 0.0], col: GRAY, },
    DebugLineVertex { pos: [0.0, 0.0, 0.0], col: BLUE, }, // z
    DebugLineVertex { pos: [0.0, 0.0, 1.0], col: BLUE, },
    DebugLineVertex { pos: [0.0, 0.0, 0.0], col: GRAY, }, // -z
    DebugLineVertex { pos: [0.0, 0.0, -1.0], col: GRAY, },
];
