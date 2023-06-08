#![allow(dead_code)]
#![allow(unused_variables)]

use crate::graphic_object::GraphicComponent;
use crate::space::Transform;

#[derive(Default)]
pub struct GameObject<'a> {
    pub active: bool,
    pub graphic_component: Option<GraphicComponent<'a>>,
    pub transform: Transform
}


pub struct Scene<'a> {
    pub active: bool,
    pub game_objects: [GameObject<'a>]
}
