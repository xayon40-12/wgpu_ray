use cgmath::Vector3;

pub struct Camera {
    X: Vector3<f32>,
    Y: Vector3<f32>,
    Z: Vector3<f32>,
    pos: Vector3<f32>,
    pub ratio: f32
}

impl Camera {
    pub fn new(ratio: f32) -> Camera {
        Camera {
            X:   Vector3::new(1.0, 0.0, 0.0),
            Y:   Vector3::new(0.0, 1.0, 0.0),
            Z:   Vector3::new(0.0, 0.0, 1.0),
            pos: Vector3::new(0.0, 0.0, 0.0),
            ratio
        }
    }

    pub fn look_at(&mut self, eye: Vector3<f32>) {
        self.Z = (eye - self.pos).normalize();
        self.X = Vector3::<f32>::unit_y().cross(self.Z);
    }
}

