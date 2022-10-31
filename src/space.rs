#![allow(dead_code)]
#![allow(unused_variables)]

use cgmath::Vector3;
use num::Float;
use libm::cosf;
use libm::sinf;
use cgmath::Quaternion;
use libm::atan2f;
use libm::asinf;

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

// conversion between quaternions and euler angles
pub fn euler_to_quaternion (euler_rot: Vector3<f32>) -> Quaternion<f32> {

    let cx = cosf(euler_rot.x/2.0);
    let sx = sinf(euler_rot.x/2.0);
    let cy = cosf(euler_rot.y/2.0);
    let sy = sinf(euler_rot.y/2.0);
    let cz = cosf(euler_rot.z/2.0);
    let sz = sinf(euler_rot.z/2.0);

    let qw = cx * cy * cz + sx * sy * sz;
    let qx = sx * cy * cz - cx * sy * sz;
    let qy = cx * sy * cz + sx * cy * sz;
    let qz = cx * cy * sz - sx * sy * cz;
 
    return Quaternion::new(qw, qx, qy, qz);
}

pub fn quaternion_to_euler (q: Quaternion<f32>) -> Vector3<f32> {
    
    let x_rot = atan2f(2.0*(q.s * q.v.x + q.v.y * q.v.z), 1.0 - 2.0*(q.v.x * q.v.x + q.v.y * q.v.y));
    let y_rot = asinf(2.0*(q.s * q.v.y - q.v.z * q.v.x));
    let z_rot = atan2f(2.0*(q.s * q.v.z + q.v.x * q.v.y), 1.0 - 2.0*(q.v.y * q.v.y + q.v.z * q.v.z));

    return Vector3::new(x_rot, y_rot, z_rot);
}

// the information of how an object is in space
// should we be using a generic num::Float type instead?
// probably, but that is something that can be refactored later
#[derive(Copy, Clone)]
pub struct Transform {
    position: Vector3<f32>,
    rotation: Vector3<f32>,
    //rotation_quat: Quaternion<f32>,
    size: Vector3<f32>,
}


impl Default for Transform {
    fn default() -> Transform {
        Transform::new(Vector3::new(0.0, 0.0, 0.0f32), Vector3::new(0.0, 0.0, 0.0f32), Vector3::new(1.0, 1.0, 1.0f32))
    }
}


impl Transform {
   
    pub fn new(pos: Vector3<f32>, rot: Vector3<f32>, size: Vector3<f32>) -> Transform {
        let res = Transform {
            position: pos,
            rotation: rot,
            size: size,
        };
        return res;
    }

    pub fn get_position(&self) -> Vector3<f32> {
        return self.position;
    }

    pub fn get_rotation(&self) -> Vector3<f32> {
        return self.rotation;
    }

    pub fn set_position(&mut self, new_pos: Vector3<f32>) -> () {
        self.position = new_pos;
    }

    pub fn set_rotation(&mut self, new_rot: Vector3<f32>) -> () {
        self.rotation = new_rot;
    }
}

// get the direction from a rotation vector
pub fn rotation_to_direction (rot: Vector3<f32>) -> Vector3<f32> {
    let x_dir = (1.0) * sinf(rot.y) * cosf(rot.z);
    let y_dir = (1.0) * sinf(rot.x) * cosf(rot.z);
    let z_dir = (1.0) * cosf(rot.x) * cosf(rot.y);
    return v3_normalised(Vector3::new(x_dir, y_dir, z_dir));
}

