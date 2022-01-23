use std::collections::HashMap;
use crate::Color;

pub trait Shape2d{
    fn draw(&mut self, frame: &mut Vec<u32>);//something to return elements getter setter and vec! for positions
}

pub struct World {
    width:usize,
    height:usize,
    pub world:Vec<Box<Shape2d>>,
    background:Box<[u8]>,
}

impl World {
    pub fn new(width:&usize, height:&usize, background:Box<[u8]>)->World{
        let proc_width = *width;
        let proc_height = *height;
        let proc_background = background.clone();
        World {width:proc_width, height:proc_height, world:Vec::new(), background:proc_background}
    }

    pub fn add(&mut self, object:Box<Shape2d>){
            self.world.push(object);   
    }

    pub fn change(&mut self, index: usize, object:Box<Shape2d>)->Box<Shape2d> {
        let val = std::mem::replace(&mut self.world[index], object);
        val
    }

    pub fn update(&mut self, frame:&mut Vec<u32>){
        for i in &mut self.world {
            i.draw(frame);
        }
        self.world.clear();
    }

    pub fn clear(&self,frame:&mut Vec<u32>) {
        for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
            pixel.copy_from_slice(&[0,0,0,0]);
        }
    }
}