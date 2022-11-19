#![allow(dead_code)]
#![allow(unused_variables)]

use cgmath::Vector3;
use cgmath::Zero;
use num::Float;
use libm::cosf;
use libm::sinf;
use cgmath::Quaternion;
use libm::atan2f;
use libm::asinf;

// norm of a vector2
pub fn v3_norm <S: Float> (vec: Vector3<S>) -> S {
    return (vec.x.powi(2) + vec.y.powi(2) + vec.z.powi(2)).sqrt();
}

// normalised vector2
pub fn v3_normalised <S: Float> (vec: Vector3<S>) ->  Vector3<S> {
    let norm = v3_norm(vec);
    let res = vec.map(|x|{x/norm});
    
    if norm.is_zero() {
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
    
    let new_quat = Quaternion::new(qw, qx, qy, qz);
    //println!("prev rotation {} {} {}", euler_rot.x, euler_rot.y, euler_rot.z);
    //println!("new quat {} {} {} {}", new_quat.s, new_quat.v.x, new_quat.v.y, new_quat.v.z);
    return quaternion_normalised(new_quat);

}

pub fn quaternion_to_euler (q: Quaternion<f32>) -> Vector3<f32> {
    
    let x_rot = atan2f(2.0*(q.s * q.v.x + q.v.y * q.v.z), 1.0 - 2.0*(q.v.x * q.v.x + q.v.y * q.v.y));
    // the ifs are necessary for some edge cases
    let y_temp = 2.0*(q.s * q.v.y - q.v.z * q.v.x);
    let y_rot;
    if y_temp > 1.0 {
        y_rot = asinf(1.0);
    } else if y_temp < -1.0 {
        y_rot = asinf(-1.0);
    } else {
        y_rot = asinf(y_temp);
    }
    let z_rot = atan2f(2.0*(q.s * q.v.z + q.v.x * q.v.y), 1.0 - 2.0*(q.v.y * q.v.y + q.v.z * q.v.z));
    
    // rework this line, seems to work but needs to be relaxed with regards to minor imprecision
    //assert_eq!(q, euler_to_quaternion(Vector3::new(x_rot, y_rot, z_rot)));
    return Vector3::new(x_rot, y_rot, z_rot);
}

// the information of how an object is in space
// should we be using a generic num::Float type instead?
// probably, but that is something that can be refactored later
#[derive(Copy, Clone)]
pub struct Transform {
    position: Vector3<f32>,
    rotation: Vector3<f32>,
    rotation_quat: Quaternion<f32>,
    size: Vector3<f32>,
}


impl Default for Transform {
    fn default() -> Transform {
        Transform::new(Vector3::new(0.0, 0.0, 0.0f32), Vector3::new(0.0, 0.0, 0.0f32), Vector3::new(1.0, 1.0, 1.0f32))
    }
}


impl Transform {
   
    // can only really print one of these, ugly but useful for quick debug
    pub fn pretty_print (&self) -> () {
        print!("\rPosition: x: {}, y: {}, z: {} -- Rotation: x: {}, y: {}, z: {}                      ", self.position.x, self.position.y, self.position.z, self.rotation.x*180.0/3.1415, self.rotation.y*180.0/3.1415, self.rotation.z*180.0/3.1415);
    }

    pub fn new(pos: Vector3<f32>, rot: Vector3<f32>, size: Vector3<f32>) -> Transform {
        let res = Transform {
            position: pos,
            rotation: rot,
            rotation_quat: euler_to_quaternion(rot),
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
    
    pub fn rotate_by(&mut self, rot_delta: Vector3<f32>) -> () {
        let delta_rot_quat = euler_to_quaternion(rot_delta);
        self.rotation_quat = delta_rot_quat * self.rotation_quat;
        self.rotation = quaternion_to_euler(self.rotation_quat);
        //println!("quaternion roatation: {} {} {} {}", self.rotation_quat.s, self.rotation_quat.v.x, self.rotation_quat.v.y, self.rotation_quat.v.z);
    }

    pub fn set_position(&mut self, new_pos: Vector3<f32>) -> () {
        self.position = new_pos;
    }

    pub fn set_rotation(&mut self, new_rot: Vector3<f32>) -> () {
        self.rotation_quat = euler_to_quaternion(new_rot);
        self.rotation = new_rot;
    }
}

//pub fn quaternion_normalised <S: BaseFloat> (quat: Quaternion<S>) -> Quaternion<S> {
pub fn quaternion_normalised (quat: Quaternion<f32>) -> Quaternion<f32> {
    let norm = num::Float::sqrt(quat.s*quat.s + quat.v.x*quat.v.x + quat.v.y*quat.v.y + quat.v.z*quat.v.z);
    if norm.is_zero() {
        println!("hmm");
        return quat;
    }
    else {
        return quat / norm;
    }
}

/*
pub fn quat_mult (quat1: Quaternion<f32>, quat2: Quaternion<f32>) -> Quaternion<f32> {
    let s = quat1.s * quat2.s - quat1.v.x * quat2.v.x - quat1.v.y * quat2.v.y - quat1.v.z * quat2.v.z;
    let vx = quat1.s * quat2.v.x + quat1.v.x * quat2.s + quat1.v.y * quat2.v.z - quat1.v.z * quat2.v.y;
    let vy = quat1.s * quat2.v.y - quat1.v.x * quat2.v.z + quat1.v.y * quat2.s + quat1.v.z * quat2.v.x;
    let vz = quat1.s * quat2.v.z + quat1.v.x * quat2.v.y - quat1.v.y * quat2.v.x + quat1.v.z * quat2.s;
    let v = Vector3::new(vx, vy, vz);
    let res = Quaternion::from_sv(s, v);

    return res;
}*/



// get the direction from a rotation vector
pub fn rotation_to_direction (rot: Vector3<f32>, initial_dir: Vector3<f32>) -> Vector3<f32> {
    let quat_rot = quaternion_normalised(euler_to_quaternion(rot));
    //print!("\rhmm: {}", quaternion_to_euler(quat_rot).y);
    //return v3_normalised(Vector3::new(x_dir, y_dir, z_dir));
    //println!("quat_rot.v: {} {} {}", quat_rot.v.x, quat_rot.v.y, quat_rot.v.z);
    //let new_quat = quat_rot * Quaternion::from_sv(0.0, initial_dir) * quat_rot.conjugate();
    let quat_dir = quaternion_normalised(Quaternion::from_sv(0.0, initial_dir));
    let new_quat = quat_rot.conjugate() * quat_dir * quat_rot;
    //println!("initial direction: {} {} {}", initial_dir.x, initial_dir.y, initial_dir.z);
    if initial_dir.z == 1.0 {
        println!("new_quat vector value: {} {} {}", new_quat.v.x, new_quat.v.y, new_quat.v.z);
        println!("quat_rot: {} {} {} {}", quat_rot.s, quat_rot.v.x, quat_rot.v.y, quat_rot.v.z);
    }
    //println!("should always be 0: {}", new_quat.s);
    // a faster way to multiply the vector and quaternion
    
    //let res = 2.0 * Vector3::dot(quat_rot.v, initial_dir) * quat_rot.v + (quat_rot.s * quat_rot.s - Vector3::dot(quat_rot.v, quat_rot.v)) * initial_dir + 2.0 * quat_rot.s * Vector3::cross(quat_rot.v, initial_dir);

    //let t = 2.0 * Vector3::cross(quat_rot.v, initial_dir);
    //let res = initial_dir + quat_rot.s * t + Vector3::cross(quat_rot.v, t);

    return v3_normalised(new_quat.v);
}

