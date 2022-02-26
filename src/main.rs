extern crate sdl2;
extern crate gl;

mod shader;
mod sprite;

use std::error;
use std::time::Instant;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;
pub struct App {
    sdl_context: sdl2::Sdl,
    window: sdl2::video::Window,
    video_subsystem: sdl2::VideoSubsystem
}

impl App{
    pub fn init() -> Result<Self, Box<dyn error::Error>> {
        let sdl_context = sdl2::init()?;
        let video_subsystem = sdl_context.video()?;
        let gl_attr = video_subsystem.gl_attr();
        gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
        gl_attr.set_context_version(3, 3);
        let window = video_subsystem
            .window("Game", WIDTH, HEIGHT)
            .opengl()
            .position_centered()
            .resizable()
            .build()?;
        gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);
        Ok(App{sdl_context: sdl_context, video_subsystem: video_subsystem, window: window})
    }

    pub fn run(&mut self) {
        self.main_loop();
    }

    fn main_loop(&mut self) {
        let _gl_context = self.window.gl_create_context().unwrap();
        self.video_subsystem.gl_set_swap_interval(1).expect("can't set vsync");
        let mut event_pump = self.sdl_context.event_pump().unwrap();

        let sp = shader::Program::from_shaders("/src/triangle.vert", "/src/triangle.frag").unwrap();

        let mut triangle = sprite::Sprite::init(sp.id);

        let mut new_time = Instant::now();
        let mut delta_time: f32;
        let mut speed = nalgebra::Vector2::new(5.0, 1.0);
        speed.x = speed.x / f32::sqrt((speed.x * speed.x) + (speed.y * speed.y));
        speed.y = speed.y / f32::sqrt((speed.x * speed.x) + (speed.y * speed.y));
        let mut vector = nalgebra::Vector3::zeros();
        let time = Instant::now();


        'running: loop {
            delta_time = new_time.elapsed().as_millis() as f32 / 1000.0;
            new_time = Instant::now();
            unsafe{
                gl::ClearColor(0.3, 0.5, 1.0, 1.0);
                gl::Clear(gl::COLOR_BUFFER_BIT);
            }
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. } | 
                    Event::KeyDown { keycode: Some(Keycode::Escape), .. } => break 'running,
                    _ => {}
                }
            }
            vector.x += delta_time / speed.x; 
            vector.y += delta_time / speed.y; 
            if vector.x >= 1.0 {
                speed.x = -speed.x;
            }
            if vector.x <= -1.0 {
                speed.x = -speed.x;
            }
            if vector.y >= 1.0 {
                speed.y = -speed.y;
            }
            if vector.y <= -1.0 {
                speed.y = -speed.y;
            }

            triangle.update(vector, time);
            triangle.draw();
            self.window.gl_swap_window();
        }
    }
}

fn main() {
    let mut app = App::init().unwrap();
    app.run();
}
