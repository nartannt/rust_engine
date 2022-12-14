#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]

use crate::camera::Camera;
use crate::space::rotation_to_direction;
use cgmath::Vector3;
use glutin::event::VirtualKeyCode;
use glutin::event::WindowEvent;

// Once again could use more generic types, can't be bothered for now, might never be
enum CamInstr {
    Skip(),
    Move(Vector3<f32>),
    Rotate(Vector3<f32>),
}

pub fn update_camera(event: glutin::event::WindowEvent, camera: &mut Camera) -> () {
    let instr = get_camera_instr(event, camera);
    execute_camera_instr(instr, camera);
}

fn execute_camera_instr(instr: CamInstr, camera: &mut Camera) -> () {
   match instr {
       CamInstr::Skip() => {},

       CamInstr::Move(delta_pos) => {
           let pos = camera.transform.get_position();
           camera.transform.set_position(pos + delta_pos);
       },

       CamInstr::Rotate(delta_rot) => {
           camera.transform.rotate_by(delta_rot);
        },
    }
}

fn get_camera_instr(event: glutin::event::WindowEvent, camera: & Camera) -> CamInstr {
    let mspeed = 0.5;
    let rspeed = (3.1415 / 180.0) * 5.0;
    let pos = camera.transform.get_position();
    match event {
        glutin::event::WindowEvent::KeyboardInput { input, .. } => match input.virtual_keycode {
            Some(VirtualKeyCode::Up) => {
                let delta_rot = Vector3::new(rspeed, 0.0, 0.0);
                return CamInstr::Rotate(delta_rot);
            }
            Some(VirtualKeyCode::Down) => {
                let delta_rot = Vector3::new(-rspeed, 0.0, 0.0);
                return CamInstr::Rotate(delta_rot);
            }
            Some(VirtualKeyCode::Right) => {
                let delta_rot = Vector3::new(0.0, -rspeed, 0.0);
                return CamInstr::Rotate(delta_rot);
            }
            Some(VirtualKeyCode::Left) => {
                let delta_rot = Vector3::new(0.0, rspeed, 0.0);
                return CamInstr::Rotate(delta_rot);
            }
            Some(VirtualKeyCode::D) => {
                let delta_pos = Vector3::new(mspeed, 0.0, 0.0);
                return CamInstr::Move(delta_pos);
            }
            Some(VirtualKeyCode::Q) => {
                let delta_pos = Vector3::new(-mspeed, 0.0, 0.0);
                return CamInstr::Move(delta_pos);
            }
            Some(VirtualKeyCode::Z) => {
                let delta_pos = Vector3::new(0.0, mspeed, 0.0);
                return CamInstr::Move(delta_pos);
            }
            Some(VirtualKeyCode::S) => {
                let delta_pos = Vector3::new(0.0, -mspeed, 0.0);
                return CamInstr::Move(delta_pos);
            }
            Some(VirtualKeyCode::E) => {
                let fwd = Vector3::new(0.0, 0.0, 1.0);
                let cam_fwd = rotation_to_direction(camera.transform.get_rotation(), fwd);
                let delta_pos = cam_fwd * mspeed;
                return CamInstr::Move(delta_pos);
            }
            Some(VirtualKeyCode::R) => {
                let fwd = Vector3::new(0.0, 0.0, 1.0);
                let cam_fwd = rotation_to_direction(camera.transform.get_rotation(), fwd);
                let delta_pos = -cam_fwd * mspeed;
                return CamInstr::Move(delta_pos);
            }

            _ => return CamInstr::Skip(),
        },

        _ => return CamInstr::Skip(),
    }
}
