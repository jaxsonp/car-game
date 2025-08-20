use wgpu::{
    BindGroupDescriptor, BindGroupLayout, BindGroupLayoutDescriptor, BindGroupLayoutEntry,
    BindingType, BufferBindingType, BufferUsages, ShaderStages,
    util::{BufferInitDescriptor, DeviceExt},
};

use crate::{material::Material, vert::Vertex};

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
        let vertex_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("mesh vertex buffer"),
            contents: bytemuck::cast_slice(raw.verts),
            usage: BufferUsages::VERTEX,
        });

        let index_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("mesh index buffer"),
            contents: bytemuck::cast_slice(raw.indices),
            usage: BufferUsages::INDEX,
        });

        let material_color_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("mesh material color buffer"),
            contents: bytemuck::cast_slice(&raw.material.color),
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

#[derive(Clone, Copy)]
/// Represents the raw data loaded in by the build script
pub struct RawMesh {
    pub material: Material,
    pub verts: &'static [Vertex],
    pub indices: &'static [u16],
}
