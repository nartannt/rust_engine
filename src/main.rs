#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unreachable_code)]

#[macro_use]
extern crate glium;
extern crate image;
extern crate libm;

use crate::camera::Camera;
use crate::fps_camera_controller::update_camera;
use crate::graphic_object::load_model;
use crate::graphic_object::GraphicObject;
use crate::space::rotation_to_direction;
use crate::space::Transform;
use cgmath::Vector3;
use glium::Surface;
use glutin::event::VirtualKeyCode;
use std::path::Path;

mod camera;
mod fps_camera_controller;
mod graphic_object;
mod space;
mod teapot;

fn main() {
    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new();
    let cb = glutin::ContextBuilder::new().with_depth_buffer(24);
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    let positions = glium::VertexBuffer::new(&display, &teapot::VERTICES).unwrap();
    let normals = glium::VertexBuffer::new(&display, &teapot::NORMALS).unwrap();
    let indices = glium::IndexBuffer::new(
        &display,
        glium::index::PrimitiveType::TrianglesList,
        &teapot::INDICES,
    )
    .unwrap();

    let test_path = Path::new("src/test2.obj");
    let test = GraphicObject {
        transform: Transform::default(),
        is_active: true,
        geometry: load_model(test_path, &display),
    };

    let test_geometry = test.geometry.unwrap();

    let positions = test_geometry.vertices;
    let normals = test_geometry.normals;
    let indices = test_geometry.indices;

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

    let program =
        glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None)
            .unwrap();

    let mut t: f32 = 0.0;

    let mut main_camera = Camera {
        transform: Transform::new(
            Vector3::new(0.0, 0.0, -5.0),
            Vector3::new(0.0, 0.0, 0.0),
            Vector3::new(1.0, 1.0, 1.0),
        ),
        fov: 0.1,
    };

    event_loop.run(move |ev, _, control_flow| {
        let begin_frame_time = std::time::Instant::now();
        let speed = (3.1415 / 180.0) * 0.25f32;
        let mspeed = 0.1f32;

        let rot = main_camera.transform.get_rotation();
        let pos = main_camera.transform.get_position();

        match ev {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return;
                }
                _ =>  {update_camera(event, &mut main_camera);},
            },
            _ => (),
        }

        t += 0.002;

        main_camera.transform.rotate_by(Vector3::new(0.0, 0.0, 0.0));

        let mut target = display.draw();

        let light = [-1.0, 0.4, 0.9f32];

        let matrix = [
            [t.cos(), t.sin(), 0.0, 0.0],
            [-t.sin(), t.cos(), 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 2.0, 1.0f32],
        ];
        let view = main_camera.view_matrix();

        let perspective = {
            let (width, height) = target.get_dimensions();
            let aspect_ratio = height as f32 / width as f32;

            let fov: f32 = 3.1 / 3.0;
            let zfar = 1024.0;
            let znear = 0.1;

            let f = 1.0 / (fov / 2.0).tan();

            [
                [f * aspect_ratio, 0.0, 0.0, 0.0],
                [0.0, f, 0.0, 0.0],
                [0.0, 0.0, (zfar + znear) / (zfar - znear), 1.0],
                [0.0, 0.0, -(2.0 * zfar * znear) / (zfar - znear), 0.0],
            ]
        };

        let params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::draw_parameters::DepthTest::IfLess,
                write: true,
                ..Default::default()
            },
            ..Default::default()
        };

        target.clear_color_and_depth((0.0, 0.0, 1.0, 1.0), 1.0);
        target
            .draw(
                (&positions, &normals),
                &indices,
                &program,
                &uniform! {matrix: matrix, view: view, u_light: light, perspective: perspective},
                &params,
            )
            .unwrap();
        target.finish().unwrap();

        let next_frame_time = begin_frame_time + std::time::Duration::from_nanos(16_666_667);

        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);
    });
}
