# Cube
---
## Struct
````rust
pub struct Cube<'a> {
    pub position: Vector::Vec3d,
    points:std::vec::Vec<ndarray::ArrayBase<ndarray::OwnedRepr<f64>, ndarray::Dim<[usize; 1]>>>,
    rotation: Vector::Vec3d,
    world:& 'a mut World::World
}

impl Shape3d for Cube<'_>
````
---
## Functions
````rust
fn new(position: Vector::Vec3d, world:&mut World::World, rotation: Vector::Vec3d) -> Cube
````
new: Generates a new Cube
## Explanation
This a a basic structure to start rendering and build more complex figures