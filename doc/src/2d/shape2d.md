# Shape 2d
---
## Trait
````rust
pub trait Funnel{
    fn draw(&self, x:usize, y:usize)->Option<Box<[u8]>>;
}
````
## Functions
draw: checks if it should draw or not world gies x and y coords
## Explanation 
Vital for drawing 2d shapes in the world. On older versions Funnel. In newer verion Shape 2d

