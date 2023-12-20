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

        Game{display, event_loop, scenes: Vec::new()}
    }

    pub fn run(self) {

        let viking_house_model_path = Path::new("src/test2.obj").to_str().unwrap().to_string();

        // eventually load them from seperate file
        let vertex_shader_src = r#"
            #version 150

            in vec3 position;
            in vec3 normal;
        
            out vec3 v_normal;
            
            uniform mat4 matrix;
            uniform mat4 perspective;
            uniform mat4 view;
            //uniform mat4 resize;
            
            void main() {
                mat4 modelview = view * matrix;
                v_normal = transpose(inverse(mat3(modelview))) * normal;
                gl_Position = perspective * modelview * vec4(position, 1.0);
            }
        "#.to_string();
        let fragment_shader_src = r#"
            #version 140
            
            in vec3 v_normal; 
            out vec4 color;
            uniform vec3 u_light; 
            
            void main() {
                float brightness = dot(normalize(v_normal), normalize(u_light));
                vec3 dark_color = vec3(0.5, 0.0, 0.0);
                vec3 regular_color = vec3(1.0, 0.0, 0.0);
                color = vec4(mix(dark_color, regular_color, brightness), 1.0);
            }
        "#.to_string();


        let mut viking_scene = Scene::new(self.display.clone());

        let mut viking_house_gc = GraphicComponent::new(None, None, None);
        viking_house_gc.add_shaders(vertex_shader_src, fragment_shader_src);
        viking_house_gc.add_model(viking_house_model_path);
        
        let viking_house_go = GameObject::new(&mut viking_scene.world);

        viking_scene.world.entry(viking_house_go.entity).unwrap().add_component(viking_house_gc);

        viking_scene.add_object(viking_house_go);
        let mut main_camera = Camera::new();

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


                viking_scene.draw_scene(target, &main_camera);

                let next_frame_time = begin_frame_time + std::time::Duration::from_nanos(16_666_667);

                if std::time::Instant::now() > next_frame_time {
                    println!("Warning: needed more time for this frame");
                }

                *control_flow = WaitUntil(next_frame_time);
        });


    }

}
