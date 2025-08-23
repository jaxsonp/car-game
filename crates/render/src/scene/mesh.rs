use assets::{RawMaterial, RawMesh, RawVertex};
use wgpu::{
    BindGroupDescriptor, BindGroupLayout, BindGroupLayoutDescriptor, BindGroupLayoutEntry,
    BindingType, BufferBindingType, BufferUsages, ShaderStages,
    util::{BufferInitDescriptor, DeviceExt},
};

pub struct Mesh {
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    /// Number of indices in index buffer, for draw call
    pub n_indices: u32,
    /// Per-mesh bind group. Bindings:
    /// 0: material color
    pub bind_group: wgpu::BindGroup,
}

impl Mesh {
    pub fn from_raw(raw: RawMesh, device: &wgpu::Device) -> Mesh {
        let verts: Vec<Vertex> = raw.verts.iter().map(|raw| (*raw).into()).collect();
        let vertex_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("mesh vertex buffer"),
            contents: bytemuck::cast_slice(verts.as_slice()),
            usage: BufferUsages::VERTEX,
        });

        let index_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("mesh index buffer"),
            contents: bytemuck::cast_slice(raw.indices),
            usage: BufferUsages::INDEX,
        });

        let material: Material = raw.material.into();
        let material_color_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("mesh material color buffer"),
            contents: bytemuck::cast_slice(&material.color),
            usage: BufferUsages::UNIFORM.union(BufferUsages::COPY_DST),
        });

        let bind_group_layout = Self::get_bind_group_layout(device);
        let bind_group = device.create_bind_group(&BindGroupDescriptor {
            label: Some("mesh bind group"),
            layout: &bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: material_color_buffer.as_entire_binding(),
            }],
        });

        Mesh {
            vertex_buffer,
            index_buffer,
            n_indices: raw.indices.len() as u32,
            bind_group,
        }
    }

    pub fn get_bind_group_layout(device: &wgpu::Device) -> BindGroupLayout {
        device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: Some("mesh bind group layout"),
            entries: &[BindGroupLayoutEntry {
                binding: 0,
                visibility: ShaderStages::FRAGMENT,
                ty: BindingType::Buffer {
                    ty: BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        })
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub pos: [f32; 3],
    pub normal: [f32; 3],
}
impl Vertex {
    pub const BUFFER_LAYOUT: wgpu::VertexBufferLayout<'static> = wgpu::VertexBufferLayout {
        array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
        step_mode: wgpu::VertexStepMode::Vertex,
        attributes: &wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x3],
    };
}
impl From<RawVertex> for Vertex {
    fn from(raw: RawVertex) -> Self {
        Vertex {
            pos: raw.pos,
            normal: raw.normal,
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Material {
    /// RGB
    pub color: [f32; 4],
}
impl From<RawMaterial> for Material {
    fn from(raw: RawMaterial) -> Self {
        Material {
            color: [raw.color[0], raw.color[1], raw.color[2], 1.0],
        }
    }
}
