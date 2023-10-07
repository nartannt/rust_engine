#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unreachable_code)]

#[macro_use]
extern crate glium;
extern crate image;
extern crate libm;

//use legion::*;  doesn't seem to work
use legion::world::World;
use crate::camera::Camera;
use glutin::event_loop::ControlFlow;
use crate::fps_camera_controller::update_camera;
use glutin::event::WindowEvent;
//use crate::graphic_component::load_model;
use crate::scene::GameObject;
use crate::game::Game;
use crate::graphic_component::GraphicComponent;
use crate::space::rotation_to_direction;
use crate::space::Transform;
use cgmath::Vector3;
use glium::Surface;
use glutin::event::VirtualKeyCode;
use std::path::Path;
use crate::scene::Scene;
//use crate::graphic_component::load_shaders;
//use crate::graphic_component::Component;
//use crate::Component::GraphicComponent as GC;
use glutin::event::WindowEvent::Destroyed;

mod camera;
mod fps_camera_controller;
mod graphic_component;
mod space;
mod scene;
mod game;

// TODO
//  - implement an interface for the main game loop, ie: the user will provide a main loop under
//  the form of a closure that takes glutin events as a parameter, closure which will then be run in
//  game, where all the rest is handled
//  - work out a proper interface for something along the lines of a GameObjectTemplate for use
//  across different scenes
//  - make the project into a library
//  - provide a clean interface (ie: that doesn't expose legion) for the ecs)
//  - add getters and setters for all relevant functions
//  - make relevant fields of objects private once getters and setters have been implemented
//  - write documentation
//  - write tests and implement a pipeline
//  - comment what is happening
//  - get shaders directly from a file
//  - add relevant type or system for when a camera is fixed in order to render a scene
//  - maybe find a way to save gameobjects / components ?
//  - create interface for allowing graphic components to handle textures nicely
//  - make example toy project, to show the various features
//  - maybe add field to game object that would specify how they behave (like the unity
//  monobehaviour) ?
//  - add basic lighting
//  - reorganise the various files into more coherent ones
//  - use linter
//  - format code
//  - go through code and handle errors / warnings nicely, add asserts and other checks for
//  internal coherence
//  - check that we are indeed using the graphics card


fn main() {

    pub struct UsrGameStruct {
        viking_scene: Scene,
        main_camera: Camera,
    }

    let game: Game<UsrGameStruct> = Game::new();
    

    let init = || {
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

        let mut main_camera = Camera::new();

        let mut viking_scene = Scene::new(self.display.clone());

        let mut viking_house_gc = GraphicComponent::new(None, None, None);
        viking_house_gc.add_shaders(vertex_shader_src, fragment_shader_src);
        viking_house_gc.add_model(viking_house_model_path);
        
        let viking_house_go = GameObject::new(&mut viking_scene.world);

        viking_scene.world.entry(viking_house_go.entity).unwrap().add_component(viking_house_gc);

        viking_scene.add_object(viking_house_go);

        let game_state = UsrGameStruct{viking_scene, main_camera};

        game_state

    };

    let game_loop = |game_state: UsrGameStruct, ev: WindowEvent, control_flow: ControlFlow | {

    };

    game.run(init, game_loop);


}
