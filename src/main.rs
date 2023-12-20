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


    let game: Game = Game::new();

    game.run();


}
