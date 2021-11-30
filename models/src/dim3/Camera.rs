use crate::Vector;
use crate::Mats;

pub struct Camera {
    camera_pos:Vector::Vec3d,
    aspect_ratio: f32,
    fov: f64,
    far:f64,
    near:f64,
    q:f64

}

impl Camera {
    pub fn render_from_perspective(&self, point: &Vector::Vec3d) -> Vector::Vec2d{
        let point = Vector::Vec3d::vec_2_wndarr(point) - Vector::Vec3d::vec_2_wndarr(&self.camera_pos);  
        let coords = Mats::Mats::projection_matrix(self.aspect_ratio, self.fov,self.far,self.near,self.q).dot(&point);
        Vector::Vec2d{
            x: coords[0]/coords[3],
            y: coords[1]/coords[3]
        }
    }

    pub fn new(camera_pos: Vector::Vec3d, aspect_ratio: f32, fov: f64, far: f64, near:f64, q:f64) -> Camera{
        Camera{
            camera_pos,
            aspect_ratio,
            fov,
            far,
            near,
            q
        }
    }
}