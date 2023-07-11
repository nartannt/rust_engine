#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unreachable_code)]

#[macro_use]
extern crate glium;
extern crate image;
extern crate libm;

use crate::camera::Camera;
use crate::fps_camera_controller::update_camera;
use crate::scene::GameObject;
use crate::graphic_object::load_model;
use crate::graphic_object::GraphicComponent;
use crate::space::rotation_to_direction;
use crate::space::Transform;
use cgmath::Vector3;
use glium::Surface;
use glutin::event::VirtualKeyCode;
use std::path::Path;
use crate::scene::Scene;
use crate::graphic_object::load_shaders;
use crate::graphic_object::Component;
use crate::Component::GraphicComponent as GC;
use glutin::event::WindowEvent::Destroyed;

mod camera;
mod fps_camera_controller;
mod graphic_object;
mod space;
mod scene;

fn main() {
    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new();
    let cb = glutin::ContextBuilder::new().with_depth_buffer(24);
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    let test_path = Path::new("src/test2.obj");


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
    "#;

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
    "#;

    let mut viking_house_gc =  GC(
        GraphicComponent{
        is_active: true,
        geometry: load_model(test_path, &display),
        program: None,
        vertex_shader: vertex_shader_src,
        fragment_shader: fragment_shader_src
    });

    let mut viking_house_go = GameObject{
        is_active: true,
        transform: Transform::new(
            Vector3::new(0.0, 0.0, 0.0),
            Vector3::new(0.0, 0.0, 0.0),
            Vector3::new(1.0, 1.0, 1.0)
        ),
        //components: &mut[&mut viking_house_gc]
        components: &mut []
    };



    let mut main_scene = Scene {
        is_active: true,
        game_objects: &mut Vec::new()
    };



    let mut main_camera = Camera {
        transform: Transform::new(
            Vector3::new(0.0, 0.0, -5.0),
            Vector3::new(0.0, 0.0, 0.0),
            Vector3::new(1.0, 1.0, 1.0),
        ),
        fov: 0.1,
    };


    //main_scene.load_scene(&display);
    //update_camera(Destroyed, &mut main_camera);

    let game_loop = event_loop.run(move |ev, _, control_flow| {

        let begin_frame_time = std::time::Instant::now();

        match ev {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return;
                }
                _ => {
                    update_camera(event, &mut main_camera);
                }
            },
            _ => (),
        }

        let mut target = display.draw();

        main_scene.draw_scene(target, &main_camera);

        let next_frame_time = begin_frame_time + std::time::Duration::from_nanos(16_666_667);

        if std::time::Instant::now() > next_frame_time {
            println!("Warning: needed more time for this frame");
        }

        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);
    });

    // nohthing to see here
}
