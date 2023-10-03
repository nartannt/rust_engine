#![allow(dead_code)]
#![allow(unused_variables)]

use crate::graphic_component::load_model;
use crate::graphic_component::load_shaders;
use crate::graphic_component::GraphicComponent;
use crate::space::Transform;
use glium::Frame;
use glium::Program;
use legion::storage::Component;
use legion::world::WorldOptions;
use legion::EntityStore;
use legion::IntoQuery;
use std::cell::RefCell;
use std::collections::HashMap;
use std::path::Path;
//use crate::graphic_component::Component;
use crate::glium::Surface;
use crate::graphic_component::ComponentTrait;
use crate::graphic_component::ComponentType;
use crate::graphic_component::ObjectModel;
use crate::Camera;
use glium::Display;
//use crate::load_shaders;
use legion::world::Entity;
use legion::world::Entry;
use legion::world::World;

// are only wrappers for legion entities
pub struct GameObject {
    pub is_active: bool,
    // clearly not a good interface but we are currently restructuring the whole project
    // we'll tolerate some clunkyness for now
    // this should probably be private
    pub entity: Entity,
    pub transform: Transform,
    //pub components: Vec<Box<Component<'a>>>
}

impl GameObject {
    pub fn new(world: &mut World) -> Self {
        let entity = world.push(());
        let test = world.entry(entity).unwrap();
        GameObject {
            is_active: true,
            transform: Transform::default(),
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

pub fn has_component<T: Component>(go: &GameObject, world: &World) -> bool {
    let entry = world.entry_ref(go.entity).unwrap();
    return entry
        .archetype()
        .layout()
        .has_component::<T>();
}

pub struct Scene {
    pub is_active: bool,

    // list of all the gameobjects in our scene
    pub game_objects: Vec<GameObject>,

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

impl Scene {
    pub fn new(display_clone: Display) -> Self {
        Scene {
            is_active: true,
            game_objects: Vec::new(),
            models: HashMap::new(),
            programs: HashMap::new(),
            world: World::new(WorldOptions::default()),
            display_clone,
        }
    }

    pub fn add_object(&mut self, go: GameObject) {
        let go_entry = self.world.entry(go.entity).unwrap();
        let Ok(gc) = go_entry.get_component::<GraphicComponent>() else { return };

        // loads and adds the model corresponding to the gc of the go if said model hasn't already
        // been loaded, when improving performance, will need to check that
        if let Some(geometry) = &gc.geometry {
            self.models
                .entry(geometry.to_string())
                .or_insert_with(|| load_model(Path::new(&geometry), &self.display_clone).unwrap());
        } else {
            println!("object has graphic component but no model");
        }

        // same thing as models but with shaders
        if let (Some(vertex_shader), Some(fragment_shader)) =
            (&gc.vertex_shader, &gc.fragment_shader)
        {
            let program_key = (vertex_shader.clone(), fragment_shader.clone());
            self.programs.entry(program_key).or_insert_with(|| {
                load_shaders(vertex_shader, fragment_shader, &self.display_clone).unwrap()
            });
        } else {
            println!("object has graphic cock but no shaders")
        }
        
        self.game_objects.push(go);
    }

    // the scene draws all its objects for now, might be subject to change later
    // will draw all active objects with active graphic components
    // note for now, we assume that all objects have at most one graphic component
    pub fn draw_scene(&mut self, mut target: Frame, camera: &Camera) {
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
        let mut draw_component = |gc: &GraphicComponent, obj_transform: &Transform| {
            //print!("drawing object in theory");
            //let go_entry = self.world.entry_ref(go.entity).unwrap();
            //let gc = go_entry.get_component::<GraphicComponent>().unwrap();
            if gc.is_active() && gc.can_be_drawn() {
                // TODO this is very unsatisfactory, need to find some way to not have to use clones
                let program_key = &(
                    gc.vertex_shader.clone().unwrap(),
                    gc.fragment_shader.clone().unwrap(),
                );
                let program = self.programs.get(program_key).unwrap();
                let object_geometry = self.models.get(gc.geometry.as_ref().unwrap()).unwrap();

                let positions = &object_geometry.vertices;
                let normals = &object_geometry.normals;
                let indices = &object_geometry.indices;

                let matrix = obj_transform.uniform_matrix();

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

        for go in &self.game_objects {
            if let Ok(gc) = self.world.entry(go.entity).unwrap().get_component::<GraphicComponent>() {
                draw_component(gc, &go.transform);
            }
        }

        target.finish().unwrap();
    }
}
