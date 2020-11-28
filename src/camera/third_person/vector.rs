use crate::geometry::{unit_vector_from_yaw_and_pitch, yaw_and_pitch_from_vector};

use approx::relative_eq;
use bevy::math::prelude::*;

const PI: f32 = std::f32::consts::PI;

/// The vector from target to camera, represented as pitch and yaw.
#[derive(Clone, Copy, Debug, Default)]
pub struct ThirdPersonVector {
    // The fields are protected to keep them in an allowable range for the camera transform.
    yaw: f32,
    pitch: f32,
}

impl ThirdPersonVector {
    pub fn unit_vector(self) -> Vec3 {
        unit_vector_from_yaw_and_pitch(self.yaw, self.pitch)
    }

    pub fn set_vector(&mut self, v: Vec3) {
        let (yaw, pitch) = yaw_and_pitch_from_vector(v);
        self.set_yaw(yaw);
        self.set_pitch(pitch);
    }

    pub fn set_yaw(&mut self, yaw: f32) {
        self.yaw = yaw % (2.0 * PI);
    }

    pub fn get_yaw(&self) -> f32 {
        self.yaw
    }

    pub fn set_pitch(&mut self, pitch: f32) {
        // Things can get weird if we are parallel to the UP vector.
        let up_eps = 0.01;
        self.pitch = pitch.min(PI / 2.0 - up_eps).max(-PI / 2.0 + up_eps);
    }

    pub fn get_pitch(&self) -> f32 {
        self.pitch
    }

    pub fn assert_not_looking_up(&self) {
        let is_looking_up = relative_eq!(self.unit_vector().dot(Vec3::unit_y()), -1.0);

        assert!(
            !is_looking_up,
            "Your camera transform is fucked up. Your look direction {} is probably bad.",
            self.unit_vector(),
        );
    }
}
