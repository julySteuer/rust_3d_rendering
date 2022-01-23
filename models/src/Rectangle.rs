use crate::World::Shape2d;
use crate::Vector::Vec2d;
use crate::Color;

pub struct Rect {
    p1:Vec2d,
    p2:Vec2d,
    color: Box<[u32]>,
    width: usize
}

impl Shape2d for Rect {
    fn draw(&mut self, frame: &mut Vec<u32>) {
            let offset_x = self.p1.x;
            let offset_y = self.p1.y;
            let size_x = self.p2.x - self.p1.x;
            let size_y = self.p2.y - self.p1.y;
            let color = Color::u32_arr_to_rgb(self.color.clone());
            for x in 0..size_x as usize {
                for y in 0..size_y as usize {
                    frame[(self.width as f64*(y as f64 + offset_y) + x as f64 + offset_x) as usize] = Color::rgb_2_int(&color);
                }
            }
    }
}

impl Rect {
    pub fn new(p1:Vec2d, p2:Vec2d, color:Box<[u32]>, width: usize)->Rect{
        Rect {p1,p2, color, width}
    }
} 
