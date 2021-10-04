use crate::World::Funnel;
use crate::Vector::Vec2d;

pub struct Rect {
    p1:Vec2d,
    p2:Vec2d,
    color: Box<[u8]>
}

impl Funnel for Rect {
    fn draw(&self, x: usize, y:usize)->Option<Box<[u8]>> {
            let in_rect: bool = {
                 x < self.p1.x as usize && x > self.p2.x as usize && y > self.p1.x as usize && y < self.p2.y as usize 
            };
            if in_rect{
                Some(self.color.clone())
            }
            else {
                None
            }
    }
}

impl Rect {
    pub fn new(p1:Vec2d, p2:Vec2d, color:Box<[u8]>)->Rect{
        Rect {p1,p2, color}
    }
} 
