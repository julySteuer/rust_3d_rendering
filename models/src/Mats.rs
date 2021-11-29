pub struct Mats {

}

impl Mats {
    pub fn rotate_x(angle: f64) -> ndarray::ArrayBase<ndarray::OwnedRepr<f64>, ndarray::Dim<[usize; 2]>>{
        let rot = ndarray::arr2(&[[angle.cos(), 0.0 , angle.sin()], [0.0,1.0,0.0], [-angle.sin(), 0.0, angle.cos()]]);
        rot
    }

    pub fn rotate_y(angle: f64) -> ndarray::ArrayBase<ndarray::OwnedRepr<f64>, ndarray::Dim<[usize; 2]>> {
        let rot = ndarray::arr2(&[[1.0,0.0,0.0], [0.0, angle.cos(), -angle.sin()], [0.0, angle.sin(), angle.cos()]]);
        rot
    }

    pub fn projection_matrix(aspect_ratio: f32, fov: f64, far: f64, near:f64, q:f64) -> ndarray::ArrayBase<ndarray::OwnedRepr<f64>, ndarray::Dim<[usize; 2]>> {
        let proj = ndarray::arr2(&[
            [aspect_ratio as f64*fov as f64, 0.0, 0.0, 0.0], 
            [0.0, fov as f64, 0.0, 0.0], 
            [0.0,0.0,q,1.0], 
            [0.0,0.0,(-far * near)/(far - near),0.0],
            ]);
        proj
    } 
}