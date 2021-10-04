use crate::World::Funnel;
use crate::Vector::Vec2d;

#[derive(Clone)]
pub struct Polygon { 
    pub vectors: Vec<Vec2d>,
    pub color:Box<[u8]>,
    pub min_x: isize,
    min_y:isize,
    max_x:isize,
    max_y:isize,
    height: usize
}

impl Polygon {
    pub fn new (vectors:Vec<Vec2d>,color:Box<[u8]>, height:usize)->Polygon {
        let v1:Vec<Vec2d> = vectors.clone();
        let mut min_x = vectors.get(0).unwrap().x;
        let mut min_y = vectors.get(0).unwrap().y;
        let mut max_x = vectors.get(0).unwrap().x;
        let mut max_y = vectors.get(0).unwrap().y;
        for i in 0..vectors.len() {
            if vectors.get(i).unwrap().x > max_x {
                max_x = vectors.get(i).unwrap().x;
            }
            if vectors.get(i).unwrap().x < min_x {
                min_x = vectors.get(i).unwrap().x;
            }
            if vectors.get(i).unwrap().y < min_y {
                min_y = vectors.get(i).unwrap().y;
            }
            if vectors.get(i).unwrap().y > max_y {
                max_y = vectors.get(i).unwrap().y;
            }
        }
        Polygon {vectors:v1,color, min_x, min_y, max_x, max_y, height}
    }

    fn get_min_max(&self, vectors:Vec<Vec2d>) {
        let v1:Vec<Vec2d> = vectors.clone();
        let mut min_x = vectors.get(0).unwrap().x;
        let mut min_y = vectors.get(0).unwrap().y;
        let mut max_x = vectors.get(0).unwrap().x;
        let mut max_y = vectors.get(0).unwrap().y;
        for i in 0..vectors.len() {
            if vectors.get(i).unwrap().x > max_x {
                max_x = vectors.get(i).unwrap().x;
            }
            if vectors.get(i).unwrap().x < min_x {
                min_x = vectors.get(i).unwrap().x;
            }
            if vectors.get(i).unwrap().y < min_y {
                min_y = vectors.get(i).unwrap().y;
            }
            if vectors.get(i).unwrap().y > max_y {
                max_y = vectors.get(i).unwrap().y;
            }
        }
    }

    fn intersect(p0:&Vec2d, p1:&Vec2d, p2:&Vec2d, p3:&Vec2d)->(f32,f32){
        let a1 = p1.y - p0.y;
        let b1 = p0.x - p1.x;
        let c1 = a1 * p0.x + b1 * p0.y;
        let a2 = p3.y - p2.y;
        let b2 = p2.x - p3.x;
        let c2 = a2 * p2.x + b2 * p2.y;
        let denominator = a1 * b2 - a2 * b1;
        ((b2 * c1 - b1 * c2) as f32 / denominator as f32, (a1 * c2 - a2 * c1) as f32 / denominator as f32)
    }
}

impl Funnel for Polygon {
    fn draw(&self,x: usize, y: usize)->Option<Box<[u8]>>{ 
        if x < self.max_x as usize && x > self.min_x as usize && y > self.min_y as usize && y < self.max_y as usize{
            let mut intersections:i8 = 0;
            let proc_x = x as isize;
            let proc_y = y as isize;
            let inp_to_vec = Vec2d{x:proc_x, y:proc_y};
            for (i, value) in self.vectors.iter().enumerate() {
                let intersects = Polygon::intersect(&inp_to_vec, &Vec2d{x:proc_x, y:self.height as isize}, value, self.vectors.get((i+1)%self.vectors.len()).clone().unwrap());//hardcoded change here with code check if intersect lises behind or in front of point
                if intersects.0 < self.max_x as f32 && intersects.0 > self.min_x as f32 && intersects.1 > self.min_y as f32 && intersects.1 < self.max_y as f32 && intersects.1 < y as f32 {
                    intersections += 1;
                }
            }
            if intersections == 1 {
                Some(self.color.clone())
            }
            else {
                None
            }
        }
        else {
            None
        }
    }
}