# Rectangle
---
## Struct
````rust
pub struct Rect {
    p1:Vec2d,
    p2:Vec2d,
    color: Box<[u8]>
}
````
---
## Functions
````rust
fn new(p1:Vec2d, p2:Vec2d, color:Box<[u8]>)->Rect
````
new: creates a new Rect
## Explanation
Draws a rectangle to at the given points