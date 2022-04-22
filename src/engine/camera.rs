use nalgebra_glm::{TVec3, TMat4, vec3, look_at};

pub struct Camera {
    pub position: TVec3<f32>,
    pub up: TVec3<f32>,
    pub mode: CameraMode,
    pub view: TMat4<f32>
}

#[derive(Debug, Clone)]
pub enum CameraMode {
    Offset (TVec3<f32>),
    Target (TVec3<f32>),
    Between (TVec3<f32>, TVec3<f32>)
}

impl Camera {
    pub fn offset(x: f32, y: f32, z: f32, r_x: f32, r_y: f32, r_z: f32) -> Self {
        let (p, u, m) = (
            vec3(x, y, z),
            vec3(0.0, 1.0, 0.0),
            CameraMode::Offset (vec3(r_x, r_y, r_z))
        );
        Camera {
            position: p,
            up: u,
            view: Camera::view(&p, &u, &m),
            mode: m
        }
    }

    fn view(position: &TVec3<f32>, up: &TVec3<f32>, mode: &CameraMode) -> TMat4<f32> {
        match mode {
            CameraMode::Offset (o) => look_at(position, &(position + o), up),
            CameraMode::Target (t) => look_at(position, t, up),
            CameraMode::Between (t, u) => look_at(position, &(t - u), up)
        }
    }

    pub fn strafe(&mut self, by: f32) {
        self.position += vec3(-by, 0.0, 0.0);
        self.view = Camera::view(&self.position, &self.up, &self.mode);
    }

    pub fn step(&mut self, by: f32) {
        self.position += vec3(0.0, 0.0, by);
        self.view = Camera::view(&self.position, &self.up, &self.mode);
    }
}