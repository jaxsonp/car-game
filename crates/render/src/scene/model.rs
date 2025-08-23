use assets::GameObject;
use nalgebra::{Point3, Rotation3};
use wgpu::{
    BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout, BindGroupLayoutDescriptor,
    BindGroupLayoutEntry, BindingType, Buffer, BufferBindingType, BufferDescriptor, BufferUsages,
    RenderPass, ShaderStages,
    util::{BufferInitDescriptor, DeviceExt},
};

use super::mesh::Mesh;
use crate::scene::debug::DebugLineGroup;

// TODO remove this
#[allow(dead_code)]
pub struct Model {
    name: String,
    meshes: Vec<Mesh>,
    debug_lines: DebugLineGroup,
    bind_group: BindGroup,
    pos_buffer: Buffer,
    rotation_buffer: Buffer,
}
impl Model {
    pub fn from_object<GO: GameObject>(name: &str, device: &wgpu::Device) -> Model {
        let meshes: Vec<Mesh> = GO::render_meshes
            .into_iter()
            .map(|raw| Mesh::from_raw(*raw, device))
            .collect();
        let debug_lines = DebugLineGroup::from_raw(device, GO::debug_lines);

        let pos_buffer = device.create_buffer(&BufferDescriptor {
            label: Some("Model position buffer"),
            size: 4 * size_of::<f32>() as u64,
            usage: BufferUsages::COPY_DST.union(BufferUsages::UNIFORM),
            mapped_at_creation: false,
        });
        let rotation_buffer = device.create_buffer(&BufferDescriptor {
            label: Some("Model rotation buffer"),
            size: 12 * size_of::<f32>() as u64,
            usage: BufferUsages::COPY_DST.union(BufferUsages::UNIFORM),
            mapped_at_creation: false,
        });

        let bind_group_layout = Self::get_bind_group_layout(device);
        let bind_group = device.create_bind_group(&BindGroupDescriptor {
            label: Some("model bind group"),
            layout: &bind_group_layout,
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: pos_buffer.as_entire_binding(),
                },
                BindGroupEntry {
                    binding: 1,
                    resource: rotation_buffer.as_entire_binding(),
                },
            ],
        });

        #[cfg(debug_assertions)]
        {
            log::info!(
                "Loaded model \"{}\" ({} verts, {} faces)",
                name,
                GO::render_meshes
                    .iter()
                    .map(|m| m.verts.len())
                    .sum::<usize>(),
                GO::render_meshes
                    .iter()
                    .map(|m| m.indices.len())
                    .sum::<usize>()
                    / 3
            );
        }
        Model {
            name: name.into(),
            meshes,
            debug_lines,
            bind_group,
            pos_buffer,
            rotation_buffer,
            //pos: Point3::new(0.0, 0.0, 0.0),
            //rotation: Rotation3::identity(),
        }
    }

    pub fn get_bind_group_layout(device: &wgpu::Device) -> BindGroupLayout {
        device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: Some("model bind group layout"),
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
                    visibility: ShaderStages::VERTEX,
                    ty: BindingType::Buffer {
                        ty: BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
            ],
        })
    }

    pub fn render(&self, render_pass: &mut RenderPass) {
        render_pass.set_bind_group(1, &self.bind_group, &[]);
        for mesh in self.meshes.iter() {
            render_pass.set_bind_group(2, &mesh.bind_group, &[]);
            render_pass.set_vertex_buffer(0, mesh.vertex_buffer.slice(..));
            render_pass.set_index_buffer(mesh.index_buffer.slice(..), wgpu::IndexFormat::Uint16); // 1.
            render_pass.draw_indexed(0..mesh.n_indices, 0, 0..1); // 2.
        }
    }

    pub fn render_debug_lines(&self, render_pass: &mut RenderPass) {
        render_pass.set_bind_group(1, &self.bind_group, &[]);
        self.debug_lines.render(render_pass);
    }
}

#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
struct RotationUniform([f32; 9]);
