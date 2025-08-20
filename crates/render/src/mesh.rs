use std::cell::OnceCell;

use wgpu::{
    BindGroupDescriptor, BindGroupLayoutDescriptor, BindGroupLayoutEntry, BindingType,
    BufferBindingType, BufferUsages, ShaderStages,
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

        let bind_group = device.create_bind_group(&BindGroupDescriptor {
            label: Some("mesh bind group"),
            layout: &Self::bind_group_layout(device),
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

    const BIND_GROUP_LAYOUT: OnceCell<wgpu::BindGroupLayout> = OnceCell::new();
    pub fn bind_group_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
        Self::BIND_GROUP_LAYOUT
            .get_or_init(|| {
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
            })
            .clone()
    }
}

#[derive(Clone, Copy)]
/// Represents the raw data loaded in by the build script
pub struct RawMesh {
    pub material: Material,
    pub verts: &'static [Vertex],
    pub indices: &'static [u16],
}
