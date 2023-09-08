#![allow(dead_code)]
#![allow(unused_variables)]


use crate::graphic_component::GraphicComponent;
use crate::graphic_component::load_shaders;
use legion::IntoQuery;
use legion::EntityStore;
use std::path::Path;
use glium::Program;
use crate::graphic_component::load_model;
use legion::world::WorldOptions;
use legion::storage::Component;
use std::cell::RefCell;
use glium::Frame;
use crate::space::Transform;
use std::collections::HashMap;
//use crate::graphic_component::Component;
use crate::graphic_component::ComponentTrait;
use crate::graphic_component::ObjectModel;
use crate::graphic_component::ComponentType;
use glium::Display;
use crate::glium::Surface;
use crate::Camera;
//use crate::load_shaders;
use legion::world::Entity;
use legion::world::World;
use legion::world::Entry;

// are only wrappers for legion entities
pub struct GameObject {
    pub is_active: bool,
    // clearly not a good interface but we are currently restructuring the whole project
    // we'll tolerate some clunkyness for now
    // this should probably be private
    pub entity: Entity,
    //pub transform: Transform,
    //pub components: Vec<Box<Component<'a>>>
}

impl GameObject{
    
    pub fn new(world: &mut World) -> Self {
        let entity = world.push(());
        let test = world.entry(entity).unwrap();
        GameObject{
            is_active: true,
            //transform: Transform::default(),
            entity: entity,
        }
    }

    /*pub fn add_component(&mut self, component: box<component<'a>>) {
        self.components.push(component);
    }

    // will return a graphic component of the object
    // todo handle multiple graphic components
    // returns ownership of the graphic component
    pub fn get_graphic_component (&mut self) -> option<box<graphiccomponent<'a>>> {
        //let graphic_components = self.components.iter().filter(|c| (***c).component_type() == "graphic");
        let mut gc_list = self.components.iter().filter_map(|c| match *c { &component::graphiccomponent(gc) => some(gc), _ => none});
        // todo finish this
        return gc_list.next();
        //return none;
    }

    // need to find better name
    // returns a reference to the graphic component
    // todo handle multiple graphic components
    pub fn read_graphic_component(&self) -> option<&box<graphiccomponent<'a>>> {
        return none;
    }*/

    pub fn is_active(&self) -> bool {
        return self.is_active;
    }

}

// TODO generalise this
pub fn has_graphic_component(go: &GameObject, world: &World) -> bool {
    let entry = world.entry_ref(go.entity).unwrap();
    return entry.archetype().layout().has_component::<GraphicComponent>();
}

pub struct Scene{
    pub is_active: bool,

    // since we will only deal with go in relation to their scene
    // it makes sense to have a world per scene
    pub world: World,

    // when we add a GameObject to a scene,
    // if it has a GraphicComponent, if its model already exists, we don't do anything
    // else, we fetch it in the files and add it to the scene models
    pub models: HashMap<String, ObjectModel>,

    // same thing as models except for shaders
    // the first String is for the vertex shaders and the second one for fragment shaders
    pub programs: HashMap<(String, String), Program>,

    // not sure if this is the right way to do things
    pub display_clone: Display,

}

impl<'a> Scene{

    pub fn new(display_clone: Display) -> Self {
        Scene {
            is_active: true,
            models: HashMap::new(),
            programs: HashMap::new(),
            world: World::new(WorldOptions::default()),
            display_clone: display_clone,
        }
    }

    pub fn add_object(&mut self, go: GameObject) {
        let go_entry = self.world.entry(go.entity).unwrap();
        let gc_res = go_entry.get_component::<GraphicComponent>();
        match gc_res {
            Ok(gc) => {
                // TODO same things as models except for shaders
                match &gc.geometry {
                    None => print!("object has graphic component but no model\n"),
                    Some(geometry) => 
                        if ! self.models.contains_key(geometry) {
                            let model = load_model(Path::new(&geometry), &self.display_clone).unwrap();
                            self.models.insert(geometry.to_string(), model);
                        },
                }


                // this is beyond horrendous, need to find a clever way to work around all the
                // clones
                match (&gc.vertex_shader, &gc.fragment_shader) {
                    (Some(vertex_shader), Some(fragment_shader)) => {
                        let program_key = &(vertex_shader.clone(), fragment_shader.clone());
                        if ! self.programs.contains_key(program_key) {
                            let program = load_shaders(vertex_shader.clone(), fragment_shader.clone(), &self.display_clone);
                            self.programs.insert(program_key.clone(), program.unwrap());
                        }
                    },
                    _ => (),
                }
            }
            Err(_) => (),
        }
    }
    
        

        
    // loads all active objects
    // so far is only useful for object with graphic components
    /*pub fn load_scene(&'a mut self, display: &Display) {
        /*let go_to_load= self.game_objects.iter().filter(|go| go.is_active() && go.has_graphic_component());
        go_to_load.for_each( |go| {
            let gc = go.get_graphic_component().unwrap();
            load_shaders(gc, display);
        });*/
    }*/



    // the scene draws all its objects for now, might be subject to change later
    // will draw all active objects with active graphic components
    // note for now, we assume that all objects have at most one graphic component
    pub fn draw_scene (&mut self, mut target: Frame, camera: &Camera) {

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
        let mut draw_component = |gc: &GraphicComponent| {
            //print!("drawing object in theory");
            //let go_entry = self.world.entry_ref(go.entity).unwrap();
            //let gc = go_entry.get_component::<GraphicComponent>().unwrap();
            if gc.is_active() && gc.can_be_drawn() {

                // TODO this is very unsatisfactory, need to find some way to not have to use clones 
                let program_key = &(gc.vertex_shader.clone().unwrap(), gc.fragment_shader.clone().unwrap());
                let program = self.programs.get(program_key).unwrap();
                let object_geometry = self.models.get(gc.geometry.as_ref().unwrap()).unwrap();


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
        
        let mut gc_query = <&GraphicComponent>::query();

        for gc in gc_query.iter(&self.world) {
            draw_component(gc);
        }

        /*let go_to_draw = self.game_objects.iter().filter(
            |go| go.is_active() && has_graphic_component(&***go, &self.world));

        go_to_draw.for_each(|go| draw_object(go));*/

        target.finish().unwrap();

    }
}


