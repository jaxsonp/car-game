use wgpu::{Device, RenderPass, SurfaceConfiguration};

use crate::{camera::Camera, load_obj, model::Model};

pub struct RenderScene {
    pub cam: Camera,
    car: Model,
    test_cube: Model,
}

impl RenderScene {
    pub fn new(device: &Device, config: &SurfaceConfiguration) -> RenderScene {
        let cam = Camera::new([5.0, 1.0, 2.0], [0.0, 0.0, 0.0], [0.0, 1.0, 0.0], config);

        RenderScene {
            cam,
            car: Model::from_raw("Car", load_obj!("car.obj"), device),
            test_cube: Model::from_raw("Test cube", load_obj!("test_cube.obj"), device),
        }
    }

    pub fn render(&self, render_pass: &mut RenderPass) {
        self.car.render(render_pass);
        //self.test_cube.render(render_pass);
    }
}
