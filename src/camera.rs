#![allow(dead_code)]
#![allow(unused_variables)]

use cgmath::Vector3;
use num::Float;

fn v3_norm <S: Float> (vec: Vector3<S>) -> S {
    return (vec.x.powi(2) + vec.y.powi(2) + vec.z.powi(2)).sqrt();
}


struct Transform {
    position: Vector3<f32>,
    direction: Vector3<f32>,
    size: Vector3<f32>,
}

struct Camera {
    transform: Transform
}

impl Camera {
    
    fn view_matrix (self, up: Vector3<f32>) -> [[f32; 4]; 4] {
        
        let dir = self.transform.direction;
        let dir_norm = v3_norm(dir);
        let dir_normalised = dir.map(|x|{x/dir_norm});

        let s = Vector3::cross(dir_normalised, up);

        let res =
            [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ];
        return res;
    }
}
