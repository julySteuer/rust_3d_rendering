# Shape 2d
---
## Trait
````rust
pub trait Shape2d{
    fn draw(&self, x:usize, y:usize)->Option<Box<[u8]>>;
}
````
## Functions
draw: checks if it should draw or not world gies x and y coords
## Explanation 
Vital for drawing 2d shapes in the world. On older versions Shape2d. In newer verion Shape 2d

