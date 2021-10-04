use crate::World::Funnel;
use crate::Vector::Vec2d;

pub struct Circle {
    pub point: Vec2d,
    radius: f32,
    color: Box<[u8]>
}

impl Circle {
    pub fn new(point: Vec2d, radius:f32, color: Box<[u8]>) -> Circle {
        Circle {point,radius, color}
    }
}

impl Funnel for Circle {
    fn draw(&self, x: usize, y: usize)->Option<Box<[u8]>> {
        let is_circle = {
            let proc = ((self.point.x as i32-x as i32).pow(2) + (self.point.y as i32-y as i32).pow(2)) as f32;
            let dist = proc.sqrt();
            proc.abs() <=  self.radius.powf(2.0)
        };
        if is_circle {
            Some(self.color.clone())
        }
        else {
            None
        }
    }
}