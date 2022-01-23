use crate::World::Shape2d;
use crate::Vector::Vec2d;
use crate::Color;

pub struct Line {
    p1: Vec2d,
    p2: Vec2d,
    color:Box<[u32]>,
    width: usize
}

impl Line {
    pub fn new(points: Vec<Vec2d>, color:Box<[u32]>, width: usize) -> Line {
        if points.len() > 2 {
            panic!("Line only has two points");
        }
        Line {p1: points[0], p2: points[1], color, width}
    }
}

impl Shape2d for Line {
    fn draw(&mut self, frame: &mut Vec<u32>) {
        /*
            let line_dist = ((self.points.get(0).unwrap().x as isize - self.points.get(1).unwrap().x as isize).pow(2) + (self.points.get(0).unwrap().y as isize - self.points.get(1).unwrap().y as isize).pow(2))as f32;
            let dist = line_dist.sqrt();
            let proc_dist_1 = ((self.points.get(1).unwrap().x as isize - x as isize).pow(2) + (self.points.get(1).unwrap().y as isize - y as isize).pow(2))as f32;
            let dist_1 = proc_dist_1.sqrt();
            let proc_dist_2 = ((self.points.get(0).unwrap().x as isize - x as isize).pow(2) + (self.points.get(0).unwrap().y as isize - y as isize).pow(2))as f32;
            let dist_2 = proc_dist_2.sqrt();
            if dist_1 + dist_2 == dist {
                Some(self.color.clone())
            }
            else {
                None
            }
            */
            let d = self.p2 - self.p1; // Add fast selecotor for testing purpose x as fast
            let mut err = d.x/2.0;
            let mut y: usize = 0; 
            for x in 0..(d.x as usize) {
                err = err-d.y;
                if err < 0.0 {
                    y+=1;
                    err += d.y;
                }
                let ges = self.width * y + x;
                frame[ges] = Color::rgb_2_int(&Color::u32_arr_to_rgb(self.color.clone()));
            }
            //todo!();
    }
}