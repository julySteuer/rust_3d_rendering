# Polygon 
---
## Struct
```rust
pub struct Polygon { 
    pub vectors: Vec<Vec2d>,
    pub color:Box<[u8]>,
    min_x: f64,
    min_y:f64,
    max_x:f64,
    max_y:f64,
    height: usize
}
```
---
## Functions
````rust
fn new(vectors:Vec<Vec2d>,color:Box<[u8]>, height:usize)->Polygon

fn get_min_max(&self, vectors:Vec<Vec2d>)

fn intersect(p0:&Vec2d, p1:&Vec2d, p2:&Vec2d, p3:&Vec2d)->(f32,f32)
````

new: generates a new Polygon with the given parameters</br>
get_min_max: calculates the minx miny maxx and maxy values. Important for drawing</br>
intersect: checks for interections
## Explanation
Draws a n-Edged polygon to the screen