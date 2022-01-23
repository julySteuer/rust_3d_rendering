use models::{Vector::Vec2d, World::Shape2d, Rectangle::Rect, World::World};
use rand::Rng;

#[derive(Clone)]
pub struct Obstacle {
    pub pos: Vec2d,
    color: Box<[u32]>,
    width: usize,
    height: usize
}

impl Obstacle {
    pub fn new(pos: Vec2d, color: Box<[u32]>, width: usize, height: usize) -> Obstacle{
        println!("{}", pos.y);
        Obstacle {
            pos, 
            color,
            width,
            height
        }
    }

    fn out_of_bounds(&mut self) -> bool{
        if self.upper_bound() || self.lower_bound() {
            return true
        }
        false
    }

    fn upper_bound(&mut self) -> bool{
        self.pos.x > self.width as f64 
    }

    fn lower_bound(&self) -> bool {
        self.pos.x < 0.0 
    }
}

impl Shape2d for Obstacle {
    fn draw(&mut self, frame: &mut Vec<u32>){
        if self.lower_bound() {
            return
        }
        let mut upper = Rect::new(Vec2d{x:500.0-self.pos.x,y:0.0}, Vec2d{x:600.0-self.pos.x,y:250.0-self.pos.y}, self.color.clone(), self.width); // sum math thing
        let mut lower = Rect::new(Vec2d{x:500.0-self.pos.x,y:300.0-self.pos.y}, Vec2d{x:600.0-self.pos.x,y:600.0}, self.color.clone(), self.width);
        upper.draw(frame);
        lower.draw(frame);
    }
}

pub struct ObstacleHolder { // First spwaed in spwing on the now spwaning element
    obj: Vec<Obstacle>
}

impl ObstacleHolder {
    pub fn new(width: usize, height: usize, color: Box<[u32]>, capacity: usize) -> ObstacleHolder {
        let mut obj: Vec<Obstacle> = Vec::with_capacity(capacity);
        let seed = height-100;
        for i in 0..4 {
            let mut num: i32 = rand::thread_rng().gen_range(0..seed)as i32;
            if num < (height/2) as i32 {
                num = -num;
            }else {
                num = num%(height/2) as i32;
            }
            let mut color = color.clone();
            color[0] = num as u32;
            color[1] = (num/2 * num/2 )as u32;
            obj.push(Obstacle::new(Vec2d{x: 200.0 * -i as f64, y: num as f64}, color.clone(), width, height))
        }
        ObstacleHolder {
            obj
        }
    }

    pub fn render(&self, world: &mut World){ // Fancy copy to inside of array function
        for i in self.obj.iter() {
            world.add(Box::new(i.clone()));
        }
    }

    pub fn update(&mut self, step_size: usize){
        for i in self.obj.iter_mut() {
                //i.pos = Vec2d{x: 0.0, y: ObstacleHolder::random_new_digit(i.height-100, i.height) as f64};
            i.pos.x += step_size as f64;
        }
    }

    pub fn random_new_digit(seed: usize, height: usize) -> i32{
        let mut num: i32 = rand::thread_rng().gen_range(0..seed)as i32;
        if num < (height/2) as i32 {
            num = -num;
            return num
        }
        //num%(height/2) as i32
        num
        /*
        num = num%(height/2) as i32;
        num
        */
    }
}