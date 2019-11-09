use cgmath::prelude::*;
use cgmath::{Vector3,Vector2,Matrix3,Rad};

pub struct Camera {
    rot: Matrix3<f32>,
    pos: Vector3<f32>,
    pub ratio: f32,
    tan_angle: f32,
    move_speed: Vector3<f32>,
    rot_speed: Vector2<f32>
}

impl Camera {
    pub fn new<A: Into<Rad<f32>>>(ratio: f32, view_angle: A) -> Camera {
        Camera {
            rot: Matrix3::identity(), 
            pos: Vector3::new(0.0, 0.0, 0.0),
            ratio,
            tan_angle: f32::tan(view_angle.into().0),
            move_speed: Vector3::new(1.0, 1.0, 1.0)*0.1,
            rot_speed: Vector2::new(1.0, 1.0),
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
        let rot_unit_y = Matrix3::from_angle_y(ry.into()*self.rot_speed.y);
        let rot_x = Matrix3::from_axis_angle(self.rot.row(0),rx.into()*self.rot_speed.x);
        self.rot = self.rot*rot_x*rot_unit_y;
    }

    pub fn translate(&mut self, forward: f32, left: f32, up: f32) {
        let (vx,vy,vz) = self.move_speed.into();
        self.pos += self.rot*Vector3::new(left*vy,up*vz,forward*vx);
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

