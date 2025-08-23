use assets::GameObject;
use nalgebra::{Point3, Rotation3};
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

    // saving pos and rotation
    new_pos: Option<Point3<f32>>,
    pos_buffer: Buffer,
    new_rotation: Option<Rotation3<f32>>,
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
            label: Some("model transform buffer"),
            size: size_of::<PosUniform>() as u64,
            usage: BufferUsages::COPY_DST.union(BufferUsages::UNIFORM),
            mapped_at_creation: false,
        });

        let rotation_buffer = device.create_buffer(&BufferDescriptor {
            label: Some("model rotation buffer"),
            size: size_of::<RotationUniform>() as u64,
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
            new_pos: Some(Point3::new(0.0, 0.0, 0.0)),
            pos_buffer,
            new_rotation: Some(Rotation3::identity()),
            rotation_buffer,
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

    pub fn prepare(&mut self, queue: &Queue) {
        if let Some(pos) = self.new_pos {
            let uniform: PosUniform = pos.into();
            queue.write_buffer(&self.pos_buffer, 0, bytemuck::cast_slice(&uniform.val));
            self.new_pos = None;
        }
        if let Some(rot) = self.new_rotation {
            let uniform: RotationUniform = rot.into();
            queue.write_buffer(&self.rotation_buffer, 0, bytemuck::cast_slice(&uniform.val));
            self.new_rotation = None;
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

    pub fn update_pos(&mut self, new_pos: Point3<f32>) {
        self.new_pos = Some(new_pos);
    }

    pub fn update_rotation(&mut self, new_rotation: Rotation3<f32>) {
        self.new_rotation = Some(new_rotation);
    }
}

#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
struct PosUniform {
    val: [f32; 4],
}
impl From<Point3<f32>> for PosUniform {
    fn from(p: Point3<f32>) -> Self {
        PosUniform {
            val: [p.x, p.y, p.z, 0.0],
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
struct RotationUniform {
    val: [f32; 16],
}
impl From<Rotation3<f32>> for RotationUniform {
    #[rustfmt::skip]
    fn from(rot: Rotation3<f32>) -> Self {
        let m = rot.matrix();
        RotationUniform {
            val: [
                m[0], m[3], m[6], 0.0,
                m[1], m[4], m[7], 0.0,
                m[2], m[5], m[8], 0.0,
                0.0,  0.0,  0.0,  1.0
            ]
        }
    }
}
