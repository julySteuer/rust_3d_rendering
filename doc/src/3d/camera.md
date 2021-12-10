# Camera
---
## Struct
````rust
pub struct Camera {
    camera_pos:Vector::Vec3d,
    aspect_ratio: f32,
    fov: f64,
    far:f64,
    near:f64,
    q:f64
}
````
---
## Functions
````rust
fn new(camera_pos: Vector::Vec3d, aspect_ratio: f32, fov: f64, far: f64, near:f64, q:f64) -> Camera

fn render_from_perspective(&self, point: &Vector::Vec3d) -> Vector::Vec2d
````
new: generates a new camera</br>
render_from_perspective: renderes 3d point to 2d coordinate
## Explanation
This camera after init you can use it to render every 3d point in 3 dims. use the render_from_perspective method
