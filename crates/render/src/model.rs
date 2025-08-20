use wgpu::RenderPass;

use crate::mesh::{Mesh, RawMesh};

pub struct Model {
    meshes: Vec<Mesh>,
}
impl Model {
    pub fn from_raw(raw_meshes: &[RawMesh], device: &wgpu::Device, queue: &wgpu::Queue) -> Model {
        Model {
            meshes: raw_meshes
                .into_iter()
                .map(|raw| Mesh::from_raw(*raw, device))
                .collect(),
        }
    }

    pub fn render(&self, render_pass: &mut RenderPass) {
        for mesh in self.meshes.iter() {
            render_pass.set_bind_group(1, &mesh.bind_group, &[]);
            render_pass.set_vertex_buffer(0, mesh.vertex_buffer.slice(..));
            render_pass.set_index_buffer(mesh.index_buffer.slice(..), wgpu::IndexFormat::Uint16); // 1.
            render_pass.draw_indexed(0..mesh.n_indices, 0, 0..1); // 2.
        }
    }
}
