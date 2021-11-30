pub trait Shape3d {
    fn render(&mut self, camera:&Camera::Camera);
}

pub mod Camera;
pub mod Cube;