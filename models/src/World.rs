use std::collections::HashMap;

pub trait Funnel{
    fn draw(&self, x:usize, y:usize)->Option<Box<[u8]>>;//something to return elements getter setter and vec! for positions
}

pub struct World {
    width:usize,
    height:usize,
    pub world:Vec<Box<Funnel>>,
    background:Box<[u8]>
}

impl World {
    pub fn new(width:&u32, height:&u32, background:Box<[u8]>)->World{
        let proc_width = *width as usize;
        let proc_height = *height as usize;
        let proc_background = background.clone();
        World {width:proc_width, height:proc_height, world:Vec::new(), background:proc_background}
    }

    pub fn add(&mut self, object:Box<Funnel>){
            self.world.push(object);   
    }

    pub fn change(&mut self, index: usize, object:Box<Funnel>)->Box<Funnel> {
        let val = std::mem::replace(&mut self.world[index], object);
        val
    }
    pub fn update(&mut self, frame:&mut [u8]){
        let mut updated = HashMap::new();
        for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
            let x = i % self.width;
            let y = i / self.height;
            for object in self.world.iter() { //only last elem gets draw FIX
                let drawer = object.draw(x,y);
                let rgba;
                if drawer != None {
                    rgba = drawer.unwrap();
                    pixel.copy_from_slice(&rgba);
                    updated.insert(i, true);
                }
                else if updated.contains_key(&i) == false { 
                    pixel.copy_from_slice(&self.background.clone());
                }
            }
        }
        self.world.clear();
    }
}