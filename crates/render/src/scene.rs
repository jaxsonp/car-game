use wgpu::{Device, RenderPass, SurfaceConfiguration};

use crate::{
    camera::{Camera, CameraController},
    load_obj,
    model::Model,
};

pub struct Scene {
    pub cam: Camera,
    pub cam_controller: CameraController,
    test_cube: Model,
}

impl Scene {
    pub fn new(device: &Device, config: &SurfaceConfiguration) -> Scene {
        let cam = Camera::new([5.0, 1.0, 2.0], [0.0, 0.0, 0.0], [0.0, 1.0, 0.0], config);

        Scene {
            cam,
            cam_controller: CameraController::new(),
            test_cube: Model::from_raw(load_obj!("test_cube.obj"), device),
        }
    }

    pub fn render(&self, render_pass: &mut RenderPass) {
        self.test_cube.render(render_pass);
    }

    pub fn update(&mut self) {
        self.cam_controller.update_camera(&mut self.cam);
    }
}
