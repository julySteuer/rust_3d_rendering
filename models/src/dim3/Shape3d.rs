use crate::World;

pub trait Shape3d {
    fn draw(&self, world:World::World);
}