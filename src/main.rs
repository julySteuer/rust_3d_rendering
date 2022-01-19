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
use models::{Rectangle::Rect, World::World, Circle::Circle, Polygon::Polygon, Vector::Vec2d,Vector::Vec3d, Line::Line, Mats::Mats, dim3::Camera, dim3::Cube, dim3::Shape3d};
mod flappy_bird;
//trait has to be in scpoe 
/*
fn main() {//use vector
    let WIDTH:u32 = 600;
    let HEIGHT:u32 = 600;
    let fov = 1.0 / (45.0/2.0 as f32).tan();
    let far = 1000.0;
    let near = 0.01;
    let ar = WIDTH as f32/HEIGHT as f32;
    let q = far/far-near;
    let mut points:std::vec::Vec<ndarray::ArrayBase<ndarray::OwnedRepr<f64>, ndarray::Dim<[usize; 1]>>> = vec![arr1(&[-1.0,-1.0,-1.0,1.0]),
    arr1(&[1.0,-1.0,-1.0,1.0]),
    arr1(&[1.0,1.0,-1.0,1.0]),
    arr1(&[-1.0,1.0,-1.0,1.0]),
    arr1(&[-1.0,-1.0,1.0,1.0]),
    arr1(&[1.0,-1.0,1.0,1.0]),
    arr1(&[1.0,1.0,1.0,1.0]),
    arr1(&[-1.0,1.0,1.0,1.0])
    ];
    let projection_matrix = Mats::projection_matrix(ar, fov as f64, far,near, q);
    let mut a = 10.0;
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
    let mut camera_pos = arr1(&[0.0,0.0,0.0]);
    let mut cube_pos = Vec3d{x:0.0,y:0.0,z:0.0};
    let mut cube_rotation = Vec3d{x:0.0,y:0.0,z:0.0};
    let cam = Camera::Camera::new(Vec3d{x:0.0, y:0.0, z:0.0}, ar, fov as f64, far, near,q);
    //let z_0 = ((WIDTH/2) as f32/((fov/2.0) * PI / 180.0).tan() as f32) as f32; //make here bigger the hardcoded value i guess 
    event_loop.run(move |event, _, control_flow| {//now it will only be rendered for 1 frame
        let now = Instant::now();
        //fov += 0.1;
        let angle = a * (PI/180.0);
        //let rotate_x = ndarray::arr2(&[[1.0,0.0,0.0], [0.0, angle.cos(), -angle.sin()], [0.0, angle.sin(), angle.cos()]]);
        //let cube_pos = arr1(&[0.0, 0.0, -20.0]);
        *control_flow = ControlFlow::Poll;
        if let Event::RedrawRequested(_) = event {
            let mut cube = Cube::Cube::new(cube_pos,&mut world,Vec3d{x:0.0,y:0.0, z:0.0});
            //cube.rotate(cube_rotation);
            cube.render(&cam);
            cube_rotation.x = cube_rotation.x + 1.0;
            world.clear(pixels.get_frame());
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
                event:WindowEvent::KeyboardInput { input, .. },
                window_id
            } => {
                let mut curr_pos = arr1(&[0.0,0.0,0.0]);
                match input.virtual_keycode.unwrap() {
                    winit::event::VirtualKeyCode::W => {
                        curr_pos[2] += 0.5;
                        cube_pos.z += 0.5
                    },
                    winit::event::VirtualKeyCode::S => {
                        curr_pos[2] -= 0.5
                    },
                    winit::event::VirtualKeyCode::A => {
                        curr_pos[0] += 0.5
                    }
                    winit::event::VirtualKeyCode::D => {
                        curr_pos[0] -= 0.5
                    }
                    winit::event::VirtualKeyCode::C => { // rotate by the inverse
                        let rotate_y = Mats::rotate_x(angle);
                        //a += 0.1;
                        for i in 0..points.len() {
                            points[i] = rotate_y.dot(&arr1(&[points[i][0], points[i][1], points[i][2]]));//rotate then move by aprox z_0
                            points[i] = arr1(&[points[i][0], points[i][1], points[i][2], 1.0]);
                        }
                    },
                    winit::event::VirtualKeyCode::X => {
                        let rotate_y = Mats::rotate_x(-angle);
                        for i in 0..points.len() {
                            points[i] = rotate_y.dot(&arr1(&[points[i][0], points[i][1], points[i][2]]));//rotate then move by aprox z_0
                            points[i] = arr1(&[points[i][0], points[i][1], points[i][2], 1.0]);
                        }
                    }
                    _ => ()
                }
                for i in 0..points.len(){
                    for elem in 0..3 {
                        points[i][elem] = &points[i][elem] - &curr_pos[elem];
                    }
                }
                camera_pos += &curr_pos;
            },
            _ => (),
        }
        window.request_redraw();
    });
} 
*/
fn main(){
    flappy_bird::run();
}