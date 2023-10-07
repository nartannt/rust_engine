use crate::Scene;
use glutin::event_loop::ControlFlow;
use std::cell::OnceCell;
use legion::EntityStore;
use glutin::event::WindowEvent;
use glutin::event_loop::ControlFlow::WaitUntil;
use std::collections::HashMap;
use crate::GraphicComponent;
use crate::GameObject;
use std::path::Path;
use cgmath;
use crate::Vector3;
//use crate::load_model;
use crate::Transform;
use crate::Camera;
use crate::update_camera;
use glutin::event_loop::EventLoop;
use legion::world::World;



pub struct Game<T> {
    pub display: glium::Display,
    pub event_loop: EventLoop<()>,
    pub user_data: OnceCell<T>,

    pub scenes: Vec<Scene>,

}

impl<T> Game<T> {

    pub fn new() -> Game<T> {
        let event_loop = glutin::event_loop::EventLoop::new();
        let wb = glutin::window::WindowBuilder::new();
        let cb = glutin::ContextBuilder::new().with_depth_buffer(24);
        let display = glium::Display::new(wb, cb, &event_loop).unwrap();

        Game{display, event_loop, user_data: OnceCell::new(), scenes: Vec::new()}
    }

    pub fn run(self, init: impl FnOnce() -> T, game_loop: impl Fn(T, WindowEvent, ControlFlow) -> ()) {

        init();

        let game_loop = self.event_loop.run(
            move |ev, _, control_flow| {

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

                if let Some(scene_to_render) = self.scenes.iter().find(|scene| scene.is_active() && scene.render_cam.is_some()) {
                    scene_to_render.draw_scene(target, &scene_to_render.render_cam.unwrap());
                }
                else {
                    println!("Warning, no scene will be drawn");
                }
                

                //viking_scene.draw_scene(target, &main_camera);

                let next_frame_time = begin_frame_time + std::time::Duration::from_nanos(16_666_667);

                if std::time::Instant::now() > next_frame_time {
                    println!("Warning: needed more time for this frame");
                }

                *control_flow = WaitUntil(next_frame_time);
        });


    }

}
