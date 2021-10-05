use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use ndarray;
use ndarray::arr1;
use pixels::{Error, Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use std::f64::consts::PI;
use std::time::{Duration, Instant};
use models::{Rectangle::Rect, World::World, Circle::Circle, Polygon::Polygon, Vector::Vec2d, Line::Line};

fn main() {//use vector
    let WIDTH:u32 = 600;
    let HEIGHT:u32 = 600;
    let mut fov = 100.0;
    let matrix = ndarray::arr2(&[[1,0,0],[0,1,0]]);
    let mut points = vec![  arr1(&[-100.0,-100.0,-100.0]),
    arr1(&[100.0,-100.0,-100.0]),
    arr1(&[100.0,100.0,-100.0]),
    arr1(&[-100.0,100.0,-100.0]),
    arr1(&[-100.0,-100.0,100.0]),
    arr1(&[100.0,-100.0,100.0]),
    arr1(&[100.0,100.0,100.0]),
    arr1(&[-100.0,100.0,100.0])//normalized
    ];
    let mut a = 0.0;
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
    event_loop.run(move |event, _, control_flow| {
        let now = Instant::now();
        //a += 0.2;
        //fov += 0.1;
        for i in 0..points.len(){
            points[i][0] += 1.0;
        }
        let z_near = ((WIDTH as f32/2.0)/((fov/2.0) * PI / 180.0).tan() as f32) as f32;
        let angle = a * (PI/180.0);
        let rotate_x = ndarray::arr2(&[[1.0,0.0,0.0], [0.0, angle.cos(), -angle.sin()], [0.0, angle.sin(), angle.cos()]]);
        for i in 0..points.len() {
            points[i] =rotate_x.dot(&points[i]);
        }
        *control_flow = ControlFlow::Poll;
        for (i, vec) in points.iter().enumerate(){
            let x_t = points[i][0] + 2.0;
            let y_t = points[i][1] + 2.0;
            let z_t = points[i][2] + 2.0;
            let x = x_t as f32 * z_near/(z_near+z_t as f32);
            let y = y_t as f32 * z_near/(z_near+z_t as f32);//test
            //let new_vert = matrix.dot(&arr1(&[x as isize,y as isize,points[i][2] as isize]));
            //println!("{}, {}, {}", points[i][0],points[i][1], points[i][2]);
            //println!("x: {}, y:{}, z:{}", x,y, z_t);//MAKE SOMETHING BIGGER
            world.add(Box::new(Circle::new(Vec2d{x:(x + 300.0) as isize, y:((y+ 300.0)) as isize}, 5.0, Box::new(rgba)))); //translate center
        } 
        //println!("-------------");
        world.clear(pixels.get_frame());
        world.update(pixels.get_frame());
        pixels.render().unwrap();
        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } if window_id == window.id() => *control_flow = ControlFlow::Exit,
            _ => (),
        }
        println!("{}", 1000/now.elapsed().as_millis());
    });
} 
