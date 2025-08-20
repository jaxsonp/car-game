use wgpu::RenderPass;

use crate::mesh::{Mesh, RawMesh};

pub struct Model {
    name: String,
    meshes: Vec<Mesh>,
}
impl Model {
    pub fn from_raw<S: Into<String>>(
        name: S,
        raw_meshes: &[RawMesh],
        device: &wgpu::Device,
    ) -> Model {
        let model = Model {
            name: name.into(),
            meshes: raw_meshes
                .into_iter()
                .map(|raw| Mesh::from_raw(*raw, device))
                .collect(),
        };

        log::info!(
            "Loaded model \"{}\" ({} verts, {} faces)",
            model.name,
            raw_meshes.iter().map(|m| m.verts.len()).sum::<usize>(),
            raw_meshes.iter().map(|m| m.indices.len()).sum::<usize>() / 3
        );

        model
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
