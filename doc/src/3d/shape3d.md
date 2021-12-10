# Shape 3d
---
## Trait
````rust
pub trait Shape3d {
    fn draw(&self, world:World::World);
}
````
## Explanation
This trait is used do render all 3d shapes. Every 3d shape that has to be rendered need to implement this trait in order for it to work with the rendering pipeline