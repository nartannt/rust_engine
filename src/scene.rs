#![allow(dead_code)]
#![allow(unused_variables)]


use crate::graphic_object::GraphicComponent;
use glium::Frame;
use crate::space::Transform;
use crate::graphic_object::Component;
use crate::graphic_object::ComponentTrait;
use crate::graphic_object::ObjectModel;
use glium::Display;
use crate::glium::Surface;
use crate::Camera;
use crate::load_shaders;

#[derive(Default)]
pub struct GameObject<'a> {
    pub is_active: bool,
    pub transform: Transform,
    pub components: &'a mut[&'a mut Component<'a>]
}

impl <'a> GameObject<'a> {
    pub fn add_component(&'a mut self, component: &'a mut Component<'a>) -> () {
        // TODO
        ();
    }

    // will return the first graphic component of the object
    // TODO handle multiple graphic components
    pub fn get_graphic_component (&self) -> Option<&mut GraphicComponent> {
        let graphic_components = self.components.iter().filter(|c| (***c).component_type() == "graphic");
        // TODO finish this
        return None;
    }

    // will return true iff the go has a graphic component
    pub fn has_graphic_component(&self) -> bool {
        match self.get_graphic_component() {
            None => return false,
            Some(_) => return true
        }
    }

    pub fn is_active(&self) -> bool {
        return self.is_active;
    }

}

pub struct Scene<'a> {
    pub is_active: bool,
    // TODO need a more appropriate structure
    pub game_objects: &'a mut [&'a mut GameObject<'a>]
}

impl <'a> Scene<'a> {
    // loads all active objects
    // so far is only useful for object with graphic components
    pub fn load_scene(&'a mut self, display: &Display) {
        let go_to_load= self.game_objects.iter().filter(|go| go.is_active() && go.has_graphic_component());
        go_to_load.for_each( |go| {
            let gc = go.get_graphic_component().unwrap();
            load_shaders(gc, display);
        });
    }



    // the scene draws all its objects for now, might be subject to change later
    // will draw all active objects with active graphic components
    // note for now, we assume that all objects have at most one graphic component
    pub fn draw_scene (&self, mut target: Frame, camera: &Camera) {


        // refreshes the background colour 
        target.clear_color_and_depth((0.0, 0.0, 1.0, 1.0), 1.0);


        // TODO set up lights properly (can wait some)
        let light = [-1.0, 0.4, 0.9f32];


        // parameters, not 100% what they do
        let params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::draw_parameters::DepthTest::IfLess,
                write: true,
                ..Default::default()
            },
            ..Default::default()
        };


        // computes the camera's veiw matrix
        let view = camera.view_matrix();

        // computes the perspective matrix
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


        // we need the game object in order to draw the object because that is where its
        // transform is stored
        let mut draw_object = |go: &GameObject| {
            let gc = go.get_graphic_component().unwrap();
            if gc.is_active() {

                let program = gc.program.as_ref().unwrap();

                let object_geometry = gc.geometry.as_ref().unwrap();

                let positions = &object_geometry.vertices;
                let normals = &object_geometry.normals;
                let indices = &object_geometry.indices;


                // TODO depends on the transform of each object
                let matrix = [
                    [1.0, 0.0, 0.0, 0.0],
                    [0.0, 1.0, 0.0, 0.0],
                    [0.0, 0.0, 1.0, 0.0],
                    [0.0, 0.0, 1.0, 1.0f32],
                ];

                
                // this is where it should be
                target
                    .draw(
                        (positions, normals),
                        indices,
                        &program,
                        &uniform! {matrix: matrix, view: view, u_light: light, perspective: perspective},
                        &params,
                    )
                    .unwrap();
            }
            
        };
        
        let go_to_draw = self.game_objects.iter().filter(|go| go.is_active() && go.has_graphic_component());

        go_to_draw.for_each(|go| draw_object(*go));

        target.finish().unwrap();

    }
}

