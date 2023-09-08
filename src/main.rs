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
use crate::fps_camera_controller::update_camera;
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

fn main() {
    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new();
    let cb = glutin::ContextBuilder::new().with_depth_buffer(24);
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    let world = World::default();
    let game = Game{display: display};

    game.run(event_loop);


        // nohthing to see here
}
