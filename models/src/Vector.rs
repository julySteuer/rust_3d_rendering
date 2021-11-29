#[derive(Debug, Copy, Clone)]
pub struct Vec2d {
    pub x: f32,
    pub y: f32
}

#[derive(Debug, Copy, Clone)]
pub struct Vec3d {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

impl Vec3d {
    pub fn vec_2_ndarr(vec: Vec3d)->ndarray::ArrayBase<ndarray::OwnedRepr<f32>, ndarray::Dim<[usize; 1]>> {
        ndarray::arr1(&[vec.x, vec.y, vec.z])
    }

    pub fn ndarr_2_vec(arr:ndarray::ArrayBase<ndarray::OwnedRepr<f32>, ndarray::Dim<[usize; 1]>>)->Vec3d {
        Vec3d{
            x:arr[0],
            y:arr[1],
            z:arr[2]
        }
    }
}

impl Vec2d {
    pub fn vec_2_ndarr(vec: Vec2d)->ndarray::ArrayBase<ndarray::OwnedRepr<f32>, ndarray::Dim<[usize; 1]>> {
        ndarray::arr1(&[vec.x, vec.y])
    }

    pub fn ndarr_2_vec(arr:ndarray::ArrayBase<ndarray::OwnedRepr<f32>, ndarray::Dim<[usize; 1]>>)->Vec2d {
        Vec2d{
            x:arr[0],
            y:arr[1],
        }
    }
}