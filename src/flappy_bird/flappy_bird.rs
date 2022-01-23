use std::time::{Duration, Instant};
use models::{Rectangle::Rect, World::World, Circle::Circle, Polygon::Polygon, Vector::Vec2d,Vector::Vec3d, Line::Line, Mats::Mats, dim3::Camera, dim3::Cube, dim3::Shape3d};
use crate::flappy_bird::Obstacle;
use rand::Rng;
use rand::thread_rng;
use minifb::{Key, Window, WindowOptions};
const WIDTH:usize = 600;
const HEIGHT:usize = 600;

/*
pub fn run(){
    let size = LogicalSize::new(WIDTH, HEIGHT);
    let event_loop = EventLoop::new();
    let mut world = World::new(&WIDTH, &HEIGHT, Box::new([0xff,0,0,0xff]));
    let window = WindowBuilder::new().with_inner_size(size).with_max_inner_size(size).build(&event_loop).unwrap();
    let window_size = window.inner_size();
    let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
    let mut pixels = {
        Pixels::new(WIDTH, HEIGHT, surface_texture).unwrap()
    };
    let rgba = [0,0xff,0,0xff];
    let mut num: i32 = rand::thread_rng().gen_range(0..HEIGHT)as i32;
    if num < (HEIGHT/2) as i32 {
        num = -num;
    }else {
        num = num%(HEIGHT/2) as i32;
    }
    println!("{}", num);
    let mut obs_holder = Obstacle::ObstacleHolder::new(WIDTH as usize, HEIGHT as usize, Box::new(rgba), 5);
    event_loop.run(move |event, _, control_flow| {//now it will only be rendered for 1 frame
        let now = Instant::now();
        *control_flow = ControlFlow::Poll;
        if let Event::RedrawRequested(_) = event {
            world.clear(pixels.get_frame());
            //obs_holder.update(5);
            //obs_holder.render(&mut world);
            world.update(pixels.get_frame());
            pixels.render().unwrap();
            println!("{}", 1000/now.elapsed().as_millis());
        }
        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } if window_id == window.id() => *control_flow = ControlFlow::Exit,
            Event::WindowEvent {
                event: WindowEvent::KeyboardInput{
                    input,..
                },
                window_id
            } => {
                match input.virtual_keycode.unwrap(){
                    winit::event::VirtualKeyCode::Space => {
                        println!("Space");
                    },
                    _ => ()
                }
            },
            _ => (),
        }
        window.request_redraw();
    });
}
*/

pub fn run() { // Front buffer and back buffer
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    let mut world: World = World::new(&WIDTH, &HEIGHT, Box::new([0xff,0,0,0xff]));
    let rgba = [0,0xff,0,0xff];
    let mut window = Window::new(
        "Test - ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    // Limit to max ~60 fps update rate
    let mut obs_holder = Obstacle::ObstacleHolder::new(WIDTH as usize, HEIGHT as usize, Box::new(rgba), 5);
    while window.is_open() && !window.is_key_down(Key::Escape) {
        let now = Instant::now();
        //world.clear(&mut buffer);
        //obs_holder.render(&mut world);
        //obs_holder.update(2);
        //world.update(&mut buffer);
        window
            .update_with_buffer(&buffer, WIDTH, HEIGHT)
            .unwrap();
        println!("{}", now.elapsed().as_millis());
    }
}