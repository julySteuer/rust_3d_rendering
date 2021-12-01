use crate::dim3::Shape3d;
use crate::Vector;
use ndarray::arr1;
use crate::World;
use crate::Circle;
use crate::dim3::Camera;
use crate::Mats;

pub struct Cube<'a> {
    pub position: Vector::Vec3d,
    points:std::vec::Vec<ndarray::ArrayBase<ndarray::OwnedRepr<f64>, ndarray::Dim<[usize; 1]>>>,
    rotation: Vector::Vec3d,
    world:& 'a mut World::World
}

impl Cube<'_> {
    pub fn new(position: Vector::Vec3d, world:&mut World::World, rotation: Vector::Vec3d) -> Cube{
        Cube {
            position,
            points: vec![arr1(&[-1.0,-1.0,-1.0,1.0]),
            arr1(&[1.0,-1.0,-1.0,1.0]),
            arr1(&[1.0,1.0,-1.0,1.0]),
            arr1(&[-1.0,1.0,-1.0,1.0]),
            arr1(&[-1.0,-1.0,1.0,1.0]),
            arr1(&[1.0,-1.0,1.0,1.0]),
            arr1(&[1.0,1.0,1.0,1.0]),
            arr1(&[-1.0,1.0,1.0,1.0])
            ],
            rotation,
            world
        }
    }
}

impl Shape3d for Cube<'_> {
    fn render(&mut self, camera:&Camera::Camera){
        let mut new_point:std::vec::Vec<ndarray::ArrayBase<ndarray::OwnedRepr<f64>, ndarray::Dim<[usize; 1]>>> = vec![arr1(&[0.0,0.0,0.0]),
        arr1(&[0.0,0.0,0.0]),
        arr1(&[0.0,0.0,0.0]),
        arr1(&[0.0,0.0,0.0]),
        arr1(&[0.0,0.0,0.0]),
        arr1(&[0.0,0.0,0.0]),
        arr1(&[0.0,0.0,0.0]),
        arr1(&[0.0,0.0,0.0])];
        for i in 0..self.points.len() {
            for elem in 0..3 {
                new_point[i][elem] = &self.points[i][elem] - &Vector::Vec3d::vec_2_ndarr(&self.position)[elem];
            }
        }
        
        for i in 0..self.points.len() {
            let coords = camera.render_from_perspective(&Vector::Vec3d::ndarr_2_vec(&new_point[i]));
            self.world.add(Box::new(Circle::Circle::new(Vector::Vec2d{x:(coords.x + 300.0), y:(coords.y+ 300.0)}, 5.0, Box::new([0,0xff,0,0xff])))); // make bigger coords between -1 and 1 add 300
        }
    }

    fn rotate(&mut self, new_rotation:Vector::Vec3d){
        self.rotation = new_rotation;
        for i in 0..self.points.len(){
            self.points[i] = Mats::Mats::rotate_x(new_rotation.x).dot(&arr1(&[self.points[i][0], self.points[i][1], self.points[i][2]]));
            self.points[i] = Mats::Mats::rotate_y(new_rotation.y).dot(&self.points[i]);
            //self.points[i] = Mats::Mats::rotate_z(new_position.x).dot(&self.points[i]); add rotate around z
        }
    }
}