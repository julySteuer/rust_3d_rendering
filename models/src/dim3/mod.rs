use crate::Vector::Vec3d;
pub trait Shape3d {
    fn render(&mut self, camera:&Camera::Camera);
    fn rotate(&mut self, new_rotation:Vec3d);
}

pub mod Camera;
pub mod Cube;