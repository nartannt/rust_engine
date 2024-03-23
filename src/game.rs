use crate::Scene;
use crate::Camera;
use crate::Transform;
use crate::input::KeyboardState;
use crate::fps_camera_controller::update_camera;

use std::cell::OnceCell;
use std::collections::HashMap;
use std::path::Path;
use std::thread::sleep;

use glutin::event::VirtualKeyCode;
use glutin::event::WindowEvent;
use glutin::event_loop::ControlFlow::WaitUntil;
use glutin::event_loop::EventLoop;


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

    pub fn add_scene(&mut self, mut scene: Scene) {
        scene.add_display_clone(&self.display);
        self.scenes.push(scene);
    }

    pub fn run(mut self) {
        let mut keyboard_state = KeyboardState::new();
        let mut main_camera = Camera::new();

        self.scenes[0].load_all_gc();

        let game_loop = self.event_loop.run(move |ev, _, control_flow| {
            let begin_frame_time = std::time::Instant::now();
            let next_frame_time = begin_frame_time + std::time::Duration::from_nanos(16_666_667);


            match ev {
                glutin::event::Event::WindowEvent { event, .. } => match event {
                    WindowEvent::CloseRequested => {
                        *control_flow = glutin::event_loop::ControlFlow::Exit;
                        return;
                    }
                    WindowEvent::KeyboardInput {
                        input:
                            glutin::event::KeyboardInput {
                                virtual_keycode: Some(VirtualKeyCode::X),
                                ..
                            },
                        ..
                    } => {
                        *control_flow = glutin::event_loop::ControlFlow::Exit;
                        return;
                    }
                    WindowEvent::KeyboardInput {
                        input:
                            glutin::event::KeyboardInput {
                                virtual_keycode: Some(key),
                                state,
                                ..
                            },
                        ..
                    } => {
                        keyboard_state.process_event(state, key);
                    }
                    _ => {
                        //println!("event: {:?}", event);
                    }
                },
                glutin::event::Event::MainEventsCleared => {
                    update_camera(&keyboard_state, &mut main_camera);

                    let target = self.display.draw();

                    self.scenes[0].draw_scene(target, &main_camera);


                },
                glutin::event::Event::RedrawEventsCleared => {

                    if std::time::Instant::now() > next_frame_time {
                        println!("Warning: needed more time for this frame");
                    }

                    sleep(next_frame_time - std::time::Instant::now());
                    
                    *control_flow = WaitUntil(next_frame_time);
                },
                _ => (),
            }


        });
    }

}
