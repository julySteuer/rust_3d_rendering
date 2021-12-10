# Shape 3d
---
## Trait
````rust
pub trait Shape3d {
    fn render(&mut self, camera:&Camera::Camera);
    fn rotate(&mut self, new_rotation:Vec3d);
}
````
---
## Functions
render: for the render code
rotate: for relative element rotation
## Explanation
This trait is used do render all 3d shapes. Every 3d shape that has to be rendered need to implement this trait in order for it to work with the rendering pipeline