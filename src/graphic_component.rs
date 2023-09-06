#![allow(dead_code)]
#![allow(unused_variables)]

use glium::Display;
use glium::Program;
use glium::backend::Facade;
use glium::ProgramCreationError;
use glium::IndexBuffer;
use glium::VertexBuffer;
use obj::load_obj;
use obj::Obj;
use obj::ObjError;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

extern crate glium;
extern crate obj;

use crate::Transform;

#[derive(Copy, Clone)]
pub struct Vertex {
    position: (f32, f32, f32),
}

implement_vertex!(Vertex, position);

#[derive(Copy, Clone)]
pub struct Normal {
    normal: (f32, f32, f32),
}

implement_vertex!(Normal, normal);

// this should be in its own file TODO
// also need to find a better name
pub trait ComponentTrait<'a> {
    fn is_active(&self) -> bool;
    fn set_active(&mut self, activation: bool) -> ();

    // it would be great to find a more graceful way to handle this (i don't have any ideas)
    fn component_type(&self) -> ComponentType;

}

#[derive(PartialEq)]
pub enum ComponentType {
    GraphicComponent
}


pub struct ObjectModel {
    pub vertices: glium::VertexBuffer<Vertex>,
    pub normals: glium::VertexBuffer<Normal>,
    pub indices: glium::IndexBuffer<u16>,
}

//#[derive(Default)]
pub struct GraphicComponent {
    pub is_active: bool,
    // path to the model (should probably have a specific type for that)
    // will become an option if necessary
    pub geometry: String,
    pub vertex_shader: String,
    pub fragment_shader: String,
}

impl <'a> GraphicComponent {
    pub fn new(model_path: String, vertex_shader_path: String, fragment_shader_path: String) -> Self {
        GraphicComponent {
            is_active: true,
            geometry: model_path,
            vertex_shader: vertex_shader_path,
            fragment_shader: fragment_shader_path,
        }
    }

    pub fn add_shaders(&mut self, vertex_shader: String, fragment_shader: String) {
        self.vertex_shader = vertex_shader;
        self.fragment_shader = fragment_shader;
    }

    /*pub fn add_geometry(&mut self, model: ObjectModel) {
        self.geometry = Some(model);
    }*/

    // the lifetime is way too long, we only need the matrix for a single frame
    // could be solved by passing a mut as parameter and modifying that but it seems disgusting
   pub fn get_uniform_matrix (transform: Transform) -> &'a[&'a[f32; 4]; 4] {
       return &[
            &[1.0, 0.0, 0.0, 0.0],
            &[0.0, 1.0, 0.0, 0.0],
            &[0.0, 0.0, 1.0, 0.0],
            &[0.0, 0.0, 0.0, 1.0f32],
       ];
    }


}

impl <'a> ComponentTrait<'a> for GraphicComponent {
    
    fn is_active(&self) -> bool {
       return self.is_active;
    }
    
    fn set_active(&mut self, activation: bool) {
        self.is_active = activation;
    }

    // TODO replace string by enum
    fn component_type (&self) -> ComponentType {
        return ComponentType::GraphicComponent;
    }

}
/*
// TODO should be in its own file
// do i actually need this? yes, yes you do, maybe not anymore ...
pub enum Component<'a> {
    GraphicComponent(Box<GraphicComponent<'a>>)
}

impl <'a> ComponentTrait<'a> for Component<'a> {
    fn is_active(&self) -> bool {
        match self {
            Component::GraphicComponent(gc) => GraphicComponent::<'_>::is_active(&gc)
        }
    }
    fn set_active(&mut self, activation: bool) {
        match self {
            Component::GraphicComponent(ref mut gc) => gc.set_active(activation)
        }

    }
    fn component_type(&self) -> ComponentType {
        match self {
            Component::GraphicComponent(gc) => GraphicComponent::<'_>::component_type(&gc)
        }
    }

}*/

pub fn load_model(model_file_path: &Path, display: &Display) -> Option<ObjectModel> {
    let file_result = File::open(model_file_path);
    match file_result {
        Err(err) => {
            println!("Warning, failed to open file: {}", err);
            return None;
        }
        Ok(file) => {
            let input = BufReader::new(file);
            let model_result: Result<Obj, ObjError> = load_obj(input);
            match model_result {
                Err(err) => {
                    println!("Warning, failed to load object: {}", err);
                    return None;
                }
                Ok(model) => {
                    let new_vertices: Vec<[f32; 3]>;
                    let new_normals: Vec<[f32; 3]>;
                    (new_vertices, new_normals) = model
                        .vertices
                        .iter()
                        .map(|vertex| (vertex.position, vertex.normal))
                        .unzip();

                    // need to factorise this, possibly with the previous line to avoid doing
                    // two loops
                    let vertices_vec: Vec<Vertex>;
                    let normals_vec: Vec<Normal>;
                    vertices_vec = new_vertices
                        .iter()
                        .map(|vertex| Vertex {
                            position: (vertex[0], vertex[1], vertex[2]),
                        })
                        .collect();
                    normals_vec = new_normals
                        .iter()
                        .map(|normal| Normal {
                            normal: (normal[0], normal[1], normal[2]),
                        })
                        .collect();

                    let vertices_vertex_buffer = VertexBuffer::new(display, &vertices_vec);
                    let normals_vertex_buffer = VertexBuffer::new(display, &normals_vec);
                    let indices_vertex_buffer = IndexBuffer::new(
                        display,
                        glium::index::PrimitiveType::TrianglesList,
                        &model.indices,
                    );

                    if vertices_vertex_buffer.is_err()
                        || normals_vertex_buffer.is_err()
                        || indices_vertex_buffer.is_err()
                    {
                        println!("Error, could not create index buffers for this object");
                        return None;
                    } else {
                        // can use the unwraps because the if guarantees that they will not be
                        // errors
                        let new_geometry = ObjectModel {
                            vertices: vertices_vertex_buffer.unwrap(),
                            normals: normals_vertex_buffer.unwrap(),
                            indices: indices_vertex_buffer.unwrap(),
                        };
                        return Some(new_geometry);
                    }
                }
            }
        }
    }
}


// we have two options, we can chose to have the engine crash if anything unexpected happens or
// wait until we have no choice, will go with the middle ground of waiting as long as possible
// whilst loudly complaining
// check if shaders already loaded?
// TODO return an error, print warning and continue the best we can if function fails
/*pub fn load_shaders<'a, F: Facade>(mut graph_comp: Box<GraphicComponent>, facade: &'a F) {
    let res = glium::Program::from_source(
            facade, graph_comp.vertex_shader.unwrap(), graph_comp.fragment_shader.unwrap(), None);
    match res {
        Err(prog_err) => {
            println!("WARNING: shaders have failed to compile");
            println!("{}", prog_err.to_string());
            graph_comp.program = None;
        },
        Ok(prog_res) => {
            graph_comp.program = Some(prog_res);
        }
    }
}*/

