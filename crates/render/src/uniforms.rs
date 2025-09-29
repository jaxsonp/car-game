use nalgebra::{Isometry3, Matrix4, Vector3};

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vector3Uniform {
    val: [f32; 3],
    _pad: f32,
}
impl Vector3Uniform {
    pub fn get_slice(&self) -> [f32; 4] {
        return [self.val[0], self.val[1], self.val[2], self._pad];
    }
}
impl From<[f32; 3]> for Vector3Uniform {
    fn from(val: [f32; 3]) -> Self {
        Vector3Uniform { val, _pad: 0.0 }
    }
}
impl From<Vector3<f32>> for Vector3Uniform {
    fn from(v: Vector3<f32>) -> Self {
        Vector3Uniform {
            val: [v.x, v.y, v.z],
            _pad: 0.0,
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Matrix4Uniform {
    val: [f32; 16],
}
impl Matrix4Uniform {
    pub fn get_slice(&self) -> [f32; 16] {
        return self.val;
    }
}
impl From<Matrix4<f32>> for Matrix4Uniform {
    #[rustfmt::skip]
    fn from(m: Matrix4<f32>) -> Self {
        Matrix4Uniform::from(m.data.0)
    }
}
impl From<Isometry3<f32>> for Matrix4Uniform {
    fn from(transform: Isometry3<f32>) -> Self {
        let m: Matrix4<f32> = transform.to_homogeneous();
        Matrix4Uniform::from(m)
    }
}
impl From<[[f32; 4]; 4]> for Matrix4Uniform {
    #[rustfmt::skip]
    fn from(m: [[f32; 4]; 4]) -> Self {
        Matrix4Uniform {
            val: [
                m[0][0], m[0][1], m[0][2], m[0][3],
                m[1][0], m[1][1], m[1][2], m[1][3],
                m[2][0], m[2][1], m[2][2], m[2][3],
                m[3][0], m[3][1], m[3][2], m[3][3],
            ],
        }
    }
}
