use assets::GameObject;
use nalgebra::Isometry3;
use wgpu::{
    BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout, BindGroupLayoutDescriptor,
    BindGroupLayoutEntry, BindingType, Buffer, BufferBindingType, BufferDescriptor, BufferUsages,
    Queue, RenderPass, ShaderStages,
};

use super::mesh::Mesh;
use crate::scene::debug::DebugLineGroup;

/// Represents an object made up of meshes with materials with a position and rotation to be rendered.
/// Also contains associated debug lines
pub struct Model {
    _name: String,
    meshes: Vec<Mesh>,
    debug_lines: DebugLineGroup,
    bind_group: BindGroup,

    static_transform: Option<Isometry3<f32>>,
    new_transform: Option<Isometry3<f32>>,
    transform_buffer: Buffer,
}
impl Model {
    pub fn from_object<GO: GameObject>(
        name: &str,
        device: &wgpu::Device,
        static_transform: Option<Isometry3<f32>>,
    ) -> Model {
        let meshes: Vec<Mesh> = GO::render_meshes
            .into_iter()
            .map(|raw| Mesh::from_raw(*raw, device))
            .collect();
        let debug_lines = DebugLineGroup::from_raw(device, GO::debug_lines);

        let transform_buffer = device.create_buffer(&BufferDescriptor {
            label: Some("model transform buffer"),
            size: size_of::<TransformUniform>() as u64,
            usage: BufferUsages::COPY_DST.union(BufferUsages::UNIFORM),
            mapped_at_creation: false,
        });

        let bind_group_layout = Self::get_bind_group_layout(device);
        let bind_group = device.create_bind_group(&BindGroupDescriptor {
            label: Some("model bind group"),
            layout: &bind_group_layout,
            entries: &[BindGroupEntry {
                binding: 0,
                resource: transform_buffer.as_entire_binding(),
            }],
        });

        log::info!(
            "Loaded meshes for model \"{}\" ({} verts, {} faces)",
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
        Model {
            _name: name.into(),
            meshes,
            debug_lines,
            bind_group,
            static_transform,
            new_transform: Some(Isometry3::identity()),
            transform_buffer,
        }
    }

    pub fn get_bind_group_layout(device: &wgpu::Device) -> BindGroupLayout {
        device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: Some("model bind group layout"),
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
        })
    }

    pub fn prepare(&mut self, queue: &Queue) {
        if let Some(mut transform) = self.new_transform {
            if let Some(static_transform) = self.static_transform {
                transform *= static_transform;
            };
            let uniform: TransformUniform = transform.into();
            queue.write_buffer(
                &self.transform_buffer,
                0,
                bytemuck::cast_slice(&uniform.val),
            );
            self.new_transform = None;
        }
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

    pub fn set_transform(&mut self, transform: Isometry3<f32>) {
        self.new_transform = Some(transform);
    }
}

#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
struct TransformUniform {
    val: [f32; 16],
}
impl From<Isometry3<f32>> for TransformUniform {
    #[rustfmt::skip]
    fn from(transform: Isometry3<f32>) -> Self {
        let m = transform.to_homogeneous();
        TransformUniform {
            val: [
                m[0], m[1], m[2], m[3],
                m[4], m[5], m[6], m[7],
                m[8], m[9], m[10], m[11],
                m[12], m[13], m[14], m[15],
            ],
        }
    }
}
