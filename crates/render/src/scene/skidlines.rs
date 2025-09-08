use nalgebra::{Point3, Vector3};
use utils::RenderSnapshot;
use wgpu::{Buffer, BufferDescriptor, BufferUsages, RenderPass};

// how many vertex's are saved at a time
const BUFFER_SIZE: u64 = 900;

const SKID_SIZE: f32 = 0.3;
const SKID_OFFSET: f32 = 0.05;

const SKID_SIZE_HALF: f32 = SKID_SIZE * 0.5;

/// Vertex buffer is a circular buffer
pub struct SkidLine {
    wheel_index: usize,
    vert_buffer: Buffer,
    cur_vert_index: u64,
    last_vert: Option<SkidLineVert>,
}
impl SkidLine {
    pub fn new(device: &wgpu::Device, wheel_index: usize) -> SkidLine {
        let vert_buffer = device.create_buffer(&BufferDescriptor {
            label: Some("skidline vert buffer"),
            size: SkidLineVert::SIZE * BUFFER_SIZE,
            usage: BufferUsages::VERTEX | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        SkidLine {
            wheel_index,
            vert_buffer,
            cur_vert_index: 0,
            last_vert: None,
        }
    }

    pub fn prepare(&mut self, queue: &wgpu::Queue, snapshot: &RenderSnapshot) {
        if let Some(contact_point) = snapshot.skid_contact_points[self.wheel_index] {
            let right_dir: Vector3<f32> = snapshot.car_transform.rotation * Vector3::x();
            let up_dir: Vector3<f32> = snapshot.car_transform.rotation * Vector3::y();
            let skid_center_pos = contact_point + up_dir.scale(SKID_OFFSET);

            let right_vert = SkidLineVert::from(skid_center_pos + right_dir.scale(SKID_SIZE_HALF));
            let left_vert = SkidLineVert::from(skid_center_pos + right_dir.scale(-SKID_SIZE_HALF));

            if self.last_vert.is_none() {
                // skidding just started, emit degen triangle to cut off strip
                self.push_vert(queue, right_vert);
                self.push_vert(queue, right_vert);
            }

            self.push_vert(queue, right_vert);
            self.push_vert(queue, left_vert);
            self.last_vert = Some(left_vert);
        } else {
            // not skidding
            if let Some(last_vert) = self.last_vert {
                // skidding just ended, emit degen triangle to cut off strip
                self.push_vert(queue, last_vert);
                self.push_vert(queue, last_vert);
                self.last_vert = None;
            }
        }
    }

    fn push_vert(&mut self, queue: &wgpu::Queue, v: SkidLineVert) {
        queue.write_buffer(
            &self.vert_buffer,
            self.cur_vert_index * SkidLineVert::SIZE,
            bytemuck::cast_slice(&[v]),
        );
        self.cur_vert_index += 1;
        // if this is one of the last two verts, also insert it at the beginning to "stitch" the vertex buffer in a loop
        if self.cur_vert_index == BUFFER_SIZE - 1 {
            queue.write_buffer(&self.vert_buffer, 0, bytemuck::cast_slice(&[v]));
        } else if self.cur_vert_index == BUFFER_SIZE {
            queue.write_buffer(
                &self.vert_buffer,
                SkidLineVert::SIZE,
                bytemuck::cast_slice(&[v]),
            );
            self.cur_vert_index = 2;
        }
    }

    pub fn render(&self, render_pass: &mut RenderPass) {
        render_pass.set_vertex_buffer(0, self.vert_buffer.slice(..));

        render_pass.draw(0..(self.cur_vert_index as u32), 0..1);
        render_pass.draw((self.cur_vert_index as u32)..(BUFFER_SIZE as u32), 0..1);
    }
}

#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct SkidLineVert {
    pos: [f32; 3],
}
impl SkidLineVert {
    pub const SIZE: u64 = size_of::<Self>() as u64;
    pub const BUFFER_LAYOUT: wgpu::VertexBufferLayout<'static> = wgpu::VertexBufferLayout {
        array_stride: Self::SIZE,
        step_mode: wgpu::VertexStepMode::Vertex,
        attributes: &wgpu::vertex_attr_array![0 => Float32x3],
    };
}
impl From<Point3<f32>> for SkidLineVert {
    fn from(point: Point3<f32>) -> Self {
        SkidLineVert {
            pos: [point.x, point.y, point.z],
        }
    }
}
