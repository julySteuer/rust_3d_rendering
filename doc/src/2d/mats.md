# Mats
---
## Struct
````rust
pub struct Mats {

}
````
---
## Functions
Rotations and projections:
````rust
fn rotate_x(angle: f64) -> ndarray::ArrayBase<ndarray::OwnedRepr<f64>, ndarray::Dim<[usize; 2]>> {}

fn rotate_y(angle: f64) -> ndarray::ArrayBase<ndarray::OwnedRepr<f64>, ndarray::Dim<[usize; 2]>> {}

fn projection_matrix(aspect_ratio: f32, fov: f64, far: f64, near:f64, q:f64) -> ndarray::ArrayBase<ndarray::OwnedRepr<f64>, ndarray::Dim<[usize; 2]>>
````

rotate_x: rotate x around angle</br>
rotate_y: rotate y around angle </br>
projection_matrix: returns a projection with the given parameters 
## Explanation
A struct where all matrices are packt together and can be used for  
