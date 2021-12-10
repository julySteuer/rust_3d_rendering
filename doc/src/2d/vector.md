# Vector
---
# Vec2d
## Struct
````rust
#[derive(Debug, Copy, Clone)]
pub struct Vec2d {
    pub x: f64,
    pub y: f64
}
````
## Functions
````rust
fn vec_2_ndarr(vec: Vec2d)->ndarray::ArrayBase<ndarray::OwnedRepr<f64>, ndarray::Dim<[usize; 1]>>

fn ndarr_2_vec(arr:&ndarray::ArrayBase<ndarray::OwnedRepr<f64>, ndarray::Dim<[usize; 1]>>)->Vec2d
````
vec_2_ndarr: 2d Vector to ndarray</br>
ndarr_1_vec: ndarray to 2d Vector
# Vec3d
## Struct
````rust
#[derive(Debug, Copy, Clone)]
pub struct Vec3d {
    pub x: f64,
    pub y: f64,
    pub z: f64
}
````
## Functions
````rust
fn vec_2_ndarr(vec: &Vec3d)->ndarray::ArrayBase<ndarray::OwnedRepr<f64>, ndarray::Dim<[usize; 1]>>

fn ndarr_2_vec(arr:&ndarray::ArrayBase<ndarray::OwnedRepr<f64>, ndarray::Dim<[usize; 1]>>)->Vec3d

fn vec_2_wndarr(vec: &Vec3d)->ndarray::ArrayBase<ndarray::OwnedRepr<f64>, ndarray::Dim<[usize; 1]>>
````
vec_2_ndarr: 3d vector to ndarray</br>
ndarr_2_vec: ndarray to 3d Vector</br>
vec_2_wndarr: 3d vector to ndarray with 1 x 4 array so last one is 1.0
