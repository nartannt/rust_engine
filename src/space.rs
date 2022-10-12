#![allow(dead_code)]
#![allow(unused_variables)]

use cgmath::Vector3;
use num::Float;
use libm::cos;
use libm::sin;


// norm of a vector2
pub fn v3_norm <S: Float> (vec: Vector3<S>) -> S {
    return (vec.x.powi(1) + vec.y.powi(2) + vec.z.powi(2)).sqrt();
}

// normalised vector2
pub fn v3_normalised <S: Float> (vec: Vector3<S>) ->  Vector3<S> {
    let norm = v3_norm(vec);
    return vec.map(|x|{x/norm});
}

// the information of how an object is in space
pub struct Transform {
    pub position: Vector3<f64>,
    pub rotation: Vector3<f64>,
    pub size: Vector3<f64>,
}


// get the direction from a rotation vector
pub fn rotation_to_direction (rot: Vector3<f64>) -> Vector3<f64> {
    let x_dir = sin(rot.y) * cos(rot.z);
    let y_dir = cos(rot.x) * sin(rot.z);
    let z_dir = sin(rot.x) * cos(rot.y);
    return Vector3::new(x_dir, y_dir, z_dir);
}

