# Shape 2d
---
## Trait
````rust
pub trait Funnel{
    fn draw(&self, x:usize, y:usize)->Option<Box<[u8]>>;
}
````
## Explanation 
Vital for drawing 2d shapes in the world. On older versions Funnel. In newer verion Shape 2d

