# World
---
## Struct
````rust
pub struct World {
    width:usize,
    height:usize,
    pub world:Vec<Box<Funnel>>,
    background:Box<[u8]>,
}
````
---
## Functions
````rust
fn new(width:&u32, height:&u32, background:Box<[u8]>)->World

fn add(&mut self, object:Box<Funnel>)

fn change(&mut self, index: usize, object:Box<Funnel>)->Box<Funnel>

fn update(&mut self, frame:&mut [u8])

fn clear(&self,frame: &mut [u8]) {
````
new: Generates a new World</br>
add: adds a element to the world</br>
change: changes element of index in world</br>
update: updates the screen</br>
clear: clears the frame

## Explanation
The world is the heard of the programm. With add you can add a shape to the screen .this shape have to implement the shape 2d trait. update renders directly to the screen so you have to call update to render the things you added to the screen  