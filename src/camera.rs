#![allow(dead_code)]
#![allow(unused_variables)]

use cgmath::Quaternion;
use cgmath::Vector3;

use crate::space::quaternion_normalised;
use crate::space::rotation_to_direction;
use crate::space::v3_normalised;
use crate::space::Transform;

#[derive(Copy, Clone)]
pub struct Camera {
    pub transform: Transform,
    pub fov: f64,
}

impl Camera {
    pub fn get_transform(self) -> Transform {
        return self.transform;
    }

    pub fn set_position(mut self, new_pos: Vector3<f32>) -> () {
        self.transform.set_position(new_pos);
    }

    pub fn get_fwd(self) -> Vector3<f32> {
        let fwd = Vector3::new(0.0, 0.0, 1.0);
        let fwd_quat = Quaternion::from_sv(0.0, fwd);
        let fwd_quat_n = quaternion_normalised(fwd_quat);
        let cam_rot = self.transform.get_qrot();
        let local_fwd_quat = cam_rot * fwd_quat_n * cam_rot.conjugate();
        let fwd_vec_n = v3_normalised(local_fwd_quat.v);
        return fwd_vec_n;
    }

    pub fn get_up(self) -> Vector3<f32> {
        let fwd = Vector3::new(0.0, 1.0, 0.0);
        let fwd_quat = Quaternion::from_sv(0.0, fwd);
        let fwd_quat_n = quaternion_normalised(fwd_quat);
        let cam_rot = self.transform.get_qrot();
        let local_fwd_quat = cam_rot * fwd_quat_n * cam_rot.conjugate();
        let fwd_vec_n = v3_normalised(local_fwd_quat.v);
        return fwd_vec_n;
    }
    pub fn view_matrix(self) -> [[f32; 4]; 4] {
        let fwd = Vector3::new(0.0, 0.0, 1.0);
        let up = Vector3::new(0.0, 1.0, 0.0);
        let position = self.transform.get_position();

        let cam_rot = self.transform.get_qrot();

        let fwd_r = cam_rot.conjugate() * Quaternion::from_sv(0.0, fwd) * cam_rot;
        let up_r = cam_rot.conjugate() * Quaternion::from_sv(0.0, up) * cam_rot;

        let fwd = fwd_r.v;
        let up = up_r.v;

        let f = v3_normalised(fwd);

        let s = Vector3::cross(up, f);

        let s_norm = v3_normalised(s);

        let u = Vector3::cross(f, s_norm);

        let p = [
            -position[0] * s_norm[0] - position[1] * s_norm[1] - position[2] * s_norm[2],
            -position[0] * u[0] - position[1] * u[1] - position[2] * u[2],
            -position[0] * f[0] - position[1] * f[1] - position[2] * f[2],
        ];

        let res = [
            [s_norm[0], u[0], f[0], 0.0],
            [s_norm[1], u[1], f[1], 0.0],
            [s_norm[2], u[2], f[2], 0.0],
            [p[0], p[1], p[2], 1.0],
        ];

        return res;
    }
}
