use crate::Vector;
use crate::Mats;

struct Camera {
    camera_pos:Vector::Vec3d,
}

impl Camera {
    pub fn render_from_perspective(&self, point: &Vector::Vec3d, projection_matrix:ndarray::ArrayBase<ndarray::OwnedRepr<f32>, ndarray::Dim<[usize; 2]>>){  
         
    }

    pub fn new(camera_pos: Vector::Vec3d) -> Camera{
        Camera{
            camera_pos
        }
    }
}