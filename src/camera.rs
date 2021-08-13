use crate::graphics::WindowBundle;
use nalgebra_glm as glm;

#[rustfmt::skip]
pub const OPENGL_TO_WGPU_MATRIX: glm::Mat4 = glm::Mat4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.0,
    0.0, 0.0, 0.5, 1.0,
);

const TO_RADIAN: f32 = std::f32::consts::PI / 180.;

pub struct Camera {
    eye: glm::Vec3,
    target: glm::Vec3,
    up: glm::Vec3,
    aspect: f32,
    fovy: f32,
    znear: f32,
    zfar: f32,
}

impl Camera {
    pub fn build_view_projection_matrix(&self) -> glm::Mat4 {
        let view = glm::look_at(&self.eye, &self.target, &self.up);
        let proj = glm::perspective(self.aspect, self.fovy * TO_RADIAN, self.znear, self.zfar);
        OPENGL_TO_WGPU_MATRIX * proj * view
    }
    pub fn default(window_bundle: &WindowBundle) -> Self {
        Self {
            eye: glm::vec3(0., 1., 2.),
            target: glm::vec3(0., 0., 0.),
            up: glm::Vec3::y(),
            aspect: window_bundle.size.width as f32 / window_bundle.size.height as f32,
            fovy: 45.,
            znear: 0.1,
            zfar: 100.,
        }
    }
}
