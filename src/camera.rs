use cgmath::prelude::*;
use cgmath::{Vector3,Matrix3,Rad};

pub struct Camera {
    rot: Matrix3<f32>,
    pos: Vector3<f32>,
    pub ratio: f32
}

impl Camera {
    pub fn new(ratio: f32) -> Camera {
        Camera {
            rot: Matrix3::identity(), 
            pos: Vector3::new(0.0, 0.0, 0.0),
            ratio
        }
    }

    pub fn look_at(&mut self, eye: Vector3<f32>) {
        self.rot = Matrix3::look_at(eye-self.pos, Vector3::unit_y());
    }

    pub fn rotate(&mut self, theta: f32, phi: f32) {
        let rot_unit_y = Matrix3::from_angle_y(Rad(phi));
        let rot_x = Matrix3::from_angle_x(Rad(theta));
        self.rot = self.rot*rot_x*self.rot.transpose()*rot_unit_y.transpose();
    }
}

