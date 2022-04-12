use nalgebra_glm::{TVec3, TMat4, vec3, look_at};

pub struct Camera {
    pub position: TVec3<f32>,
    pub up: TVec3<f32>,
    pub mode: CameraMode
}

pub enum CameraMode {
    Offset (TVec3<f32>),
    Target (TVec3<f32>),
    Between (TVec3<f32>, TVec3<f32>)
}

impl Camera {
    pub fn at(x: f32, y: f32, z: f32) -> Self {
        Camera {
            position: vec3(x, y, z),
            up: vec3(0.0, 1.0, 0.0),
            mode: CameraMode::Offset (vec3(0.0, 0.0, -1.0))
        }
    }

    pub fn view(&self) -> TMat4<f32> {
        match self.mode {
            CameraMode::Offset (o) => look_at(&self.position, &(self.position + o), &self.up),
            CameraMode::Target (t) => look_at(&self.position, &t, &self.up),
            CameraMode::Between (t, u) => look_at(&self.position, &(t - u), &self.up)
        }
    }

    pub fn strafe(&mut self, by: f32) {
        self.position += vec3(-by, 0.0, 0.0);
    }

    pub fn step(&mut self, by: f32) {
        self.position += vec3(0.0, 0.0, by);
    }
}