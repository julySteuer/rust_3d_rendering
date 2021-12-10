# Circle 
---
## Struct
````rust
pub struct Circle {
    pub point: Vec2d,
    radius: f32,
    color: Box<[u8]>
}

impl Shape2d for Circle 
````
## Specific Functions
````rust
fn new(point: Vec2d, radius:f32, color: Box<[u8]>) -> Circle
````
new: Creates new element of type Circle 
## Explanation
This struct can draw a Cricle at the any point on the screen


