#![allow(dead_code)]
#![allow(unused_variables)]

use cgmath::Vector3;

use crate::space::Transform;
use crate::space::v3_normalised;
use crate::space::rotation_to_direction;

struct Camera {
    transform: Transform
}

impl Camera {
    
    fn view_matrix (self, up: Vector3<f64>) -> [[f64; 4]; 4] {
       
        let dir = rotation_to_direction(self.transform.rotation);
        let dir_normalised = v3_normalised(dir);

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
