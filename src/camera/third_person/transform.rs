use super::vector::ThirdPersonVector;

use bevy::math::prelude::*;

/// The control parameters for a third person camera transform.
#[derive(Clone, Copy, Debug)]
pub struct ThirdPersonTransform {
    pub eye_vec: ThirdPersonVector,
    pub target: Vec3,
    pub radius: f32,
}

impl ThirdPersonTransform {
    pub fn from_position_and_target(position: Vec3, target: Vec3) -> Self {
        let v = position - target;
        let mut eye_vec = ThirdPersonVector::default();
        eye_vec.set_vector(v);
        let radius = v.length();

        Self {
            target,
            radius,
            eye_vec,
        }
    }

    pub fn get_position_at_radius(&self, radius: f32) -> Vec3 {
        self.target + radius * self.eye_vec.unit_vector()
    }

    pub fn get_position(&self) -> Vec3 {
        self.get_position_at_radius(self.radius)
    }

    pub fn add_pitch(&mut self, dpitch: f32) {
        self.eye_vec.set_pitch(self.eye_vec.get_pitch() + dpitch)
    }

    pub fn add_yaw(&mut self, dyaw: f32) {
        self.eye_vec.set_yaw(self.eye_vec.get_yaw() + dyaw)
    }
}
