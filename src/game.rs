use crate::GameObject;
use crate::GraphicComponent;
use crate::Scene;
use crate::Vector3;
use cgmath;
use glutin::event::WindowEvent;
use glutin::event_loop::ControlFlow;
use glutin::event_loop::ControlFlow::WaitUntil;
use legion::EntityStore;
use std::cell::OnceCell;
use std::collections::HashMap;
use std::path::Path;
//use crate::load_model;
use crate::update_camera;
use crate::Camera;
use crate::Transform;
use glutin::event_loop::EventLoop;
use legion::world::World;

pub struct Game {
    pub display: glium::Display,
    pub event_loop: EventLoop<()>,

    pub scenes: Vec<Scene>,
}

impl Game {
    pub fn new() -> Game {
        let event_loop = glutin::event_loop::EventLoop::new();
        let wb = glutin::window::WindowBuilder::new();
        let cb = glutin::ContextBuilder::new().with_depth_buffer(24);
        let display = glium::Display::new(wb, cb, &event_loop).unwrap();

        Game {
            display,
            event_loop,
            scenes: Vec::new(),
        }
    }

    pub fn run(mut self) {
        let mut main_camera = Camera::new();

        self.scenes[0].load_all_gc();

        let game_loop = self.event_loop.run(move |ev, _, control_flow| {
            let begin_frame_time = std::time::Instant::now();

            match ev {
                glutin::event::Event::WindowEvent { event, .. } => match event {
                    WindowEvent::CloseRequested => {
                        *control_flow = glutin::event_loop::ControlFlow::Exit;
                        return;
                    }
                    _ => {
                        update_camera(event, &mut main_camera);
                    }
                },
                _ => (),
            }

            let target = self.display.draw();

            let viking_scene = &mut self.scenes[0];
            viking_scene.draw_scene(target, &main_camera);

            let next_frame_time = begin_frame_time + std::time::Duration::from_nanos(16_666_667);

            if std::time::Instant::now() > next_frame_time {
                println!("Warning: needed more time for this frame");
            }

            *control_flow = WaitUntil(next_frame_time);
        });
    }

    pub fn add_scene(&mut self, mut scene: Scene) {
        scene.add_display_clone(&self.display);
        self.scenes.push(scene);
    }
}
