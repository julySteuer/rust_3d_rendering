# First App
---
````rust
fn main() {//use vector
    let size = LogicalSize::new(WIDTH, HEIGHT);
    let event_loop = EventLoop::new();
    let mut world = World::new(&WIDTH, &HEIGHT, Box::new([0xff,0,0,0xff]));
    let window = WindowBuilder::new().with_inner_size(size).with_max_inner_size(size).build(&event_loop).unwrap();
    let window_size = window.inner_size();
    let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
    let mut pixels = {
        Pixels::new(WIDTH, HEIGHT, surface_texture).unwrap()
    };
    world.add(Box::new(Circle::Circle::new(Vector::Vec2d{x:300.0, y:300.0}, 5.0, Box::new([0,0xff,0,0xff]))))
    event_loop.run(move |event, _, control_flow| {//now it will only be rendered for 1 frame
        *control_flow = ControlFlow::Poll;
        if let Event::RedrawRequested(_) = event {
            world.clear(pixels.get_frame());
            world.update(pixels.get_frame());
            pixels.render().unwrap();
        }
        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } if window_id == window.id() => *control_flow = ControlFlow::Exit,
            
            _ => (),
        }
        window.request_redraw();
    });
} 
````