#![allow(dead_code)]
#![allow(unused_variables)]

use crate::graphic_object::GraphicComponent;
use crate::space::Transform;

#[derive(Default)]
pub struct GameObject<'a> {
    pub is_active: bool,
    // will need to transform this into a list of components
    pub graphic_component: Option<&'a GraphicComponent<'a>>,
    pub transform: Transform
}


pub struct Scene<'a> {
    pub is_active: bool,
    pub game_objects: &'a mut [&'a mut GameObject<'a>]
}
