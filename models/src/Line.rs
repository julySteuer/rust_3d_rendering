use crate::World::Funnel;
use crate::Vector::Vec2d;

pub struct Line {
    points: Vec<Vec2d>,
    color:Box<[u8]>,
}

impl Line {
    pub fn new(points: Vec<Vec2d>, color:Box<[u8]>) -> Line {
        Line {points, color}
    }
}

impl Funnel for Line {
    fn draw(&self, x: usize, y: usize) -> Option<Box<[u8]>> {
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
    }
}