use wgpu::{Device, RenderPass};

use crate::{load_obj, model::Model};

pub struct Scene {
    test_cube: Model,
}

impl Scene {
    pub fn new(device: &Device, queue: &wgpu::Queue) -> Scene {
        Scene {
            test_cube: Model::from_raw(load_obj!("test_cube.obj"), device, queue),
        }
    }

    pub fn render(&self, render_pass: &mut RenderPass) {
        self.test_cube.render(render_pass);
    }

    pub fn update(&self) {}
}
