use wgpu::{
    BufferUsages, RenderPass,
    util::{BufferInitDescriptor, DeviceExt},
};

pub struct DebugLineGroup {
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

    pub fn from_raw(device: &wgpu::Device, raw_lines: &[assets::RawDebugLine]) -> DebugLineGroup {
        let mut verts: Vec<DebugLineVertex> = Vec::new();
        for line in raw_lines {
            verts.push(DebugLineVertex {
                pos: line.pos1,
                col: line.col,
            });
            verts.push(DebugLineVertex {
                pos: line.pos2,
                col: line.col,
            });
        }
        Self::new(device, &verts)
    }

    pub fn render(&self, render_pass: &mut RenderPass) {
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.draw(0..(self.n_verts), 0..1);
    }
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
