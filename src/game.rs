use crate::GameObject;
use std::thread::sleep;
use crate::GraphicComponent;
use crate::Scene;
use crate::Vector3;
use cgmath;
use glutin::event::VirtualKeyCode;
use glutin::event::WindowEvent;
use glutin::event_loop::ControlFlow;
use glutin::event_loop::ControlFlow::WaitUntil;
use legion::EntityStore;
use std::cell::OnceCell;
use std::collections::HashMap;
use std::path::Path;
use crate::update_camera;
use crate::Camera;
use crate::Transform;
use glutin::event_loop::EventLoop;
use legion::world::World;
use glutin::event::ElementState;

// this code was stolen from https://github.com/rust-windowing/glutin/issues/708 because i couldn't
// be asked to write it myself
/// Keeps track of which keys have been pressed.
pub struct KeyboardState {
    state: HashMap<VirtualKeyCode, ElementState>,
}
impl KeyboardState {
    /// Constructs a new KeyboardState with all the keys released.
    pub fn new() -> KeyboardState {
        KeyboardState {
            state: HashMap::new(),
        }
    }

    /// Returns true if `key` is pressed.
    pub fn is_pressed(&self, key: &VirtualKeyCode) -> bool {
        self.state.get(key).map(|&s| s == ElementState::Pressed).unwrap_or(false)
    }
    /// Returns true if `key` is released.
    pub fn is_released(&self, key: &VirtualKeyCode) -> bool {
        !self.is_pressed(key)
    }

    /// Processes a keyboard event and updated the internal state.
    pub fn process_event(&mut self, key_state: ElementState, code: VirtualKeyCode) {
        match key_state {
            ElementState::Pressed => {
                self.state.insert(code, key_state);
            },
            ElementState::Released => {
                self.state.remove(&code);
            }
        }
    }
}

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
