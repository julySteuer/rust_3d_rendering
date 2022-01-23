use crate::World;

pub trait Shape3d {
    fn render(&self, world:World::World);
}