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

pub struct ObjectModel {
    pub vertices: glium::VertexBuffer<Vertex>,
    pub normals: glium::VertexBuffer<Normal>,
    pub indices: glium::IndexBuffer<u16>,
}

// we probably want to create a component trait
// i want to approach things in a clean and generic manner
// however, in order to keep things grounded, i won't implement a generic trait or method unless it
// is used more than once
#[derive(Default)]
pub struct GraphicComponent <'a>{
    pub is_active: bool,
    pub geometry: Option<ObjectModel>,
    pub program: Option<Program>,
    pub vertex_shader: &'a str,
    pub fragment_shader: &'a str
}

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
pub fn load_shaders<F: Facade>(graph_comp: &GraphicComponent, facade: &F) -> Option<Program> {
    let res = glium::Program::from_source(facade, graph_comp.vertex_shader, graph_comp.fragment_shader, None);
    match res {
        Err(prog_err) => {
            println!("WARNING: shaders have failed to compile");
            println!("{}", prog_err.to_string());
            return None;
        },
        Ok(prog_res) =>
            return Some(prog_res)
    }
}

