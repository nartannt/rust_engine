#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unreachable_code)]

#[macro_use]
extern crate glium;
extern crate image;
extern crate libm;

//use legion::*;  doesn't seem to work
use crate::camera::Camera;
use crate::fps_camera_controller::update_camera;
use glutin::event::WindowEvent;
use glutin::event_loop::ControlFlow;
use legion::world::World;
//use crate::graphic_component::load_model;
use crate::game::Game;
use crate::graphic_component::GraphicComponent;
use crate::scene::GameObject;
use crate::scene::Scene;
use crate::space::rotation_to_direction;
use crate::space::Transform;
use cgmath::Vector3;
use glium::Surface;
use glutin::event::VirtualKeyCode;
use glutin::event::WindowEvent::Destroyed;
use std::path::Path;

mod camera;
mod fps_camera_controller;
mod game;
mod graphic_component;
mod scene;
mod space;

// TODO
//  - implement an interface for the main game loop, ie: the user will provide a main loop under
//  the form of a closure that takes glutin events as a parameter, closure which will then be run in
//  game, where all the rest is handled
//  - work out a proper interface for something along the lines of a GameObjectTemplate for use
//  across different scenes
//  - make the project into a library
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
//  - enable adding textures
//  - reorganise the various files into more coherent ones
//  - use linter
//  - format code
//  - go through code and handle errors / warnings nicely, add asserts and other checks for
//  internal coherence
//  - check that we are indeed using the graphics card

fn main() {
    let viking_house_model_path = Path::new("src/test2.obj").to_str().unwrap().to_string();

    let cube_model_path = Path::new("src/cube.obj").to_str().unwrap().to_string();

    // eventually load them from seperate file
    let vertex_shader_src = "assets/shaders/vertex_shader".to_string();
    let fragment_shader_src = "assets/shaders/fragment_shader".to_string();

    let mut game: Game = Game::new();
    let mut viking_scene = Scene::new();

    let mut viking_house_gc = GraphicComponent::new(None, None, None);
    viking_house_gc.add_shaders(vertex_shader_src.clone(), fragment_shader_src.clone());
    viking_house_gc.add_model(viking_house_model_path);

    let mut cube_gc = GraphicComponent::new(None, None, None);
    cube_gc.add_shaders(vertex_shader_src.clone(), fragment_shader_src.clone());
    cube_gc.add_model(cube_model_path);

    let viking_house_go = GameObject::new(&mut viking_scene.world);
    let cube_go = GameObject::new(&mut viking_scene.world);

    viking_scene.add_component(&viking_house_go, viking_house_gc);
    viking_scene.add_component(&cube_go, cube_gc);

    viking_scene.add_object(viking_house_go);
    viking_scene.add_object(cube_go);

    game.add_scene(viking_scene);

    game.run();
}
