#![allow(dead_code)]
#![allow(unused_variables)]

use cgmath::Vector3;

use crate::space::Transform;
use crate::space::v3_normalised;
use crate::space::rotation_to_direction;

#[derive(Copy, Clone)]
pub struct Camera {
    pub transform: Transform,
    pub fov: f64,
}

impl Camera {
   
    pub fn get_transform (self) -> Transform {
        return self.transform;
    }

    pub fn set_position (mut self, new_pos: Vector3<f32>) -> () {
        self.transform.position = new_pos;
    }

    pub fn view_matrix (self, up: Vector3<f32>) -> [[f32; 4]; 4] {
       
        let dir = rotation_to_direction(self.transform.rotation);
        let dir_normalised = v3_normalised(dir);
        
        // vector that is orthogonal to the direction the camera is facing
        // and the absolute up direction
        // basically a relative right for the camera
        let s = Vector3::cross(dir_normalised, up);
        let s_normalised = v3_normalised(s);
    

        // similar thing, relative up for the camera
        let u = Vector3::cross(dir_normalised, s_normalised);
       
        
        let pos = self.transform.position;
        let p = [-pos[0] * s_normalised[0] - pos[1] * s_normalised[1] - pos[2] * s_normalised[2],
                -pos[0] * u[0] - pos[1] * u[1] - pos[2] * u[2],
                -pos[0] * dir_normalised[0] - pos[1] * dir_normalised[1] - pos[2] * dir_normalised[2],
        ];

        // added signs to fix things for now, will need to redo the maths
        let res =
            [
                [s_normalised[0], u[0], dir_normalised[0], 0.0],
                [s_normalised[1], u[1], dir_normalised[1], 0.0],
                [s_normalised[2], u[2], dir_normalised[2], 0.0],
                [p[0], p[1], p[2], 1.0f32],
            ];
        
        return res;
    }


}
