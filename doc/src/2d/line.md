# Line
---
## Struct
````rust
pub struct Line {
    points: Vec<Vec2d>,
    color:Box<[u8]>,
}

impl Shape2d for Line
````
---
## Functions
````rust
fn new(points: Vec<Vec2d>, color:Box<[u8]>) -> Line
````
new: creates a new instance of Line
## Explanation
Draws a line between the first two Vec2d in the points Vector
