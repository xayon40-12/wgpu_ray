use cgmath::prelude::*;
use cgmath::{Vector3,Matrix3,Rad};

pub struct Camera {
    rot: Matrix3<f32>,
    pos: Vector3<f32>,
    pub ratio: f32,
    tan_angle: f32
}

impl Camera {
    pub fn new<A: Into<Rad<f32>>>(ratio: f32, view_angle: A) -> Camera {
        Camera {
            rot: Matrix3::identity(), 
            pos: Vector3::new(0.0, 0.0, 0.0),
            ratio,
            tan_angle: f32::tan(view_angle.into().0)
        }
    }

    pub fn set_view_angle<A: Into<Rad<f32>>>(&mut self, view_angle: A) {
        self.tan_angle = f32::tan(view_angle.into().0);
    }

    pub fn set_ratio(&mut self, ratio: f32) {
        self.ratio = ratio;
    }

    pub fn look_at(&mut self, eye: Vector3<f32>) {
        self.rot = Matrix3::look_at(eye-self.pos, Vector3::unit_y());
    }

    pub fn rotate<A: Into<Rad<f32>>>(&mut self, rx: A, ry: A) {
        let rot_unit_y = Matrix3::from_angle_y(ry);
        let rot_x = Matrix3::from_axis_angle(self.rot.row(0),rx);
        self.rot = self.rot*rot_x.transpose()*rot_unit_y.transpose();
    }

    pub fn as_float_array(&self) -> Vec<f32> {
        let mut tmp = Vec::new();
        <&[f32; 9]>::from(self.rot.as_ref()).iter().enumerate().for_each(|(i,e)| {
            tmp.push(*e);
            if (i+1)%3 == 0 { tmp.push(0.0); }
        });
        <&[f32; 3]>::from(self.pos.as_ref()).iter().for_each(|e| tmp.push(*e));
        tmp.push(self.ratio);
        tmp.push(self.tan_angle);

        tmp
    }
}

