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
        self.transform.set_position(new_pos);
    }

    pub fn view_matrix (self) -> [[f32; 4]; 4] {
        let fwd = Vector3::new(0.0, 0.0, 1.0); 
        let dir = rotation_to_direction(self.transform.get_rotation(), fwd);
        //print!("\rfwd norm: {}", (dir.x*dir.x + dir.y*dir.y + dir.z*dir.z).sqrt());
        //println!("relative forward: x: {}, y: {}, z: {}", dir.x, dir.y, dir.z);
        let dir_normalised = v3_normalised(dir);
        
        // vector that is orthogonal to the direction the camera is facing
        // and the relative up direction of the camera
        // basically a relative right for the camera
        let up = Vector3::new(0.0, 1.0, 0.0);
        let relative_up = rotation_to_direction(self.transform.get_rotation(), up);
        let s = Vector3::cross(relative_up, dir_normalised);
        let s_normalised = v3_normalised(s);
    

        // similar thing, relative up for the camera
        //let u = Vector3::cross(dir_normalised, s_normalised);
        let u = v3_normalised(Vector3::cross(dir_normalised, s_normalised));
        //println!("u, recalculated: {} {} {}", u.x, u.y, u.z);
        //print!("\rdirection: {} {} {}                   ", dir.x, dir.y, dir.z);
        //let u = relative_up;
       
        
        let pos = self.transform.get_position();
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
