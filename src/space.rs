#![allow(dead_code)]
#![allow(unused_variables)]

use cgmath::Vector3;
use num::Float;
use libm::cosf;
use libm::sinf;


// norm of a vector2
pub fn v3_norm <S: Float> (vec: Vector3<S>) -> S {
    return (vec.x.powi(1) + vec.y.powi(2) + vec.z.powi(2)).sqrt();
}

// normalised vector2
pub fn v3_normalised <S: Float> (vec: Vector3<S>) ->  Vector3<S> {
    let norm = v3_norm(vec);
    let res = vec.map(|x|{x/norm});
    
    // if the norm is zero
    // need to find a better solution
    if res.x.is_nan() {
        return vec;
    }
    else {
        return res;
    }
    
}

// the information of how an object is in space
// should we be using a generic num::Float type instead?
// probably, but that is something that can be refactored later
#[derive(Copy, Clone)]
pub struct Transform {
    pub position: Vector3<f32>,
    pub rotation: Vector3<f32>,
    pub size: Vector3<f32>,
}

impl Default for Transform {
    fn default() -> Transform {
        Transform {
            position: Vector3::new(0.0, 0.0, 0.0f32),
            rotation: Vector3::new(0.0, 0.0, 0.0f32),
            size: Vector3::new(1.0, 1.0, 1.0f32),
        }
    }
}

// get the direction from a rotation vector
pub fn rotation_to_direction (rot: Vector3<f32>) -> Vector3<f32> {
    let x_dir = (1.0) * sinf(rot.y) * cosf(rot.z);
    let y_dir = (1.0) * sinf(rot.x) * cosf(rot.z);
    let z_dir = (1.0) * cosf(rot.x) * cosf(rot.y);
    return v3_normalised(Vector3::new(x_dir, y_dir, z_dir));
}

