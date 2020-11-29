use crate::geometry::{unit_vector_from_yaw_and_pitch, yaw_and_pitch_from_vector};

use approx::relative_eq;
use bevy::{math::prelude::*, transform::components::Transform};

const PI: f32 = std::f32::consts::PI;

/// Two points, with one orbiting the other. The oribiting point is parametrized in polar
/// coordinates.
#[derive(Clone, Copy, Debug)]
pub struct OrbitTransform {
    pub pivot: Vec3,
    pub vector: PolarVector,
    pub radius: f32,
}

impl OrbitTransform {
    pub fn from_pivot_and_orbit(pivot: Vec3, orbit: Vec3) -> Self {
        let v = orbit - pivot;
        let vector = PolarVector::from_vector(v);
        let radius = v.length();

        Self {
            pivot,
            vector,
            radius,
        }
    }

    pub fn get_orbit_position(&self) -> Vec3 {
        self.get_orbit_position_at_radius(self.radius)
    }

    pub fn get_orbit_position_at_radius(&self, radius: f32) -> Vec3 {
        self.pivot + radius * self.vector.unit_vector()
    }

    pub fn add_pitch(&mut self, dpitch: f32) {
        self.vector.add_pitch(dpitch)
    }

    pub fn add_yaw(&mut self, dyaw: f32) {
        self.vector.add_yaw(dyaw)
    }

    pub fn get_pitch(&self) -> f32 {
        self.vector.get_pitch()
    }

    pub fn get_yaw(&self) -> f32 {
        self.vector.get_yaw()
    }

    pub fn unit_vector(&self) -> Vec3 {
        self.vector.unit_vector()
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct PolarVector {
    // The fields are protected to keep them in an allowable range for the camera transform.
    yaw: f32,
    pitch: f32,
}

impl PolarVector {
    pub fn from_vector(v: Vec3) -> Self {
        let mut p = Self::default();
        p.set_vector(v);

        p
    }

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

    pub fn add_yaw(&mut self, delta: f32) {
        self.set_yaw(self.get_yaw() + delta);
    }

    pub fn set_pitch(&mut self, pitch: f32) {
        // Things can get weird if we are parallel to the UP vector.
        let up_eps = 0.01;
        self.pitch = pitch.min(PI / 2.0 - up_eps).max(-PI / 2.0 + up_eps);
    }

    pub fn get_pitch(&self) -> f32 {
        self.pitch
    }

    pub fn add_pitch(&mut self, delta: f32) {
        self.set_pitch(self.get_pitch() + delta);
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

#[derive(Default)]
pub struct Smoother {
    lerp_tfm: Option<LerpOrbitTransform>,
}

impl Smoother {
    /// Do linear interpolation between the previous smoothed transform and the new transform. This
    /// is equivalent to an exponential smoothing filter.
    pub fn smooth_transform(
        &mut self,
        lag_weight: f32,
        new_tfm: &OrbitTransform,
    ) -> LerpOrbitTransform {
        debug_assert!(0.0 <= lag_weight);
        debug_assert!(lag_weight < 1.0);

        let old_lerp_tfm = self.lerp_tfm.unwrap_or_else(|| LerpOrbitTransform {
            orbit: new_tfm.get_orbit_position(),
            pivot: new_tfm.pivot,
        });

        let lead_weight = 1.0 - lag_weight;
        let lerp_pos = old_lerp_tfm.orbit * lag_weight + new_tfm.get_orbit_position() * lead_weight;
        let lerp_pivot = old_lerp_tfm.pivot * lag_weight + new_tfm.pivot * lead_weight;

        let new_lerp_tfm = LerpOrbitTransform {
            orbit: lerp_pos,
            pivot: lerp_pivot,
        };

        self.lerp_tfm = Some(new_lerp_tfm);

        new_lerp_tfm
    }
}

#[derive(Clone, Copy)]
pub struct LerpOrbitTransform {
    pub orbit: Vec3,
    pub pivot: Vec3,
}

impl LerpOrbitTransform {
    pub fn pivot_look_at_orbit_transform(&self) -> Transform {
        p1_look_at_p2_transform(self.pivot, self.orbit)
    }

    pub fn orbit_look_at_pivot_transform(&self) -> Transform {
        p1_look_at_p2_transform(self.orbit, self.pivot)
    }
}

pub fn p1_look_at_p2_transform(p1: Vec3, p2: Vec3) -> Transform {
    // If p1 and p2 are very close, we avoid imprecision issues by keeping the look vector a unit
    // vector.
    let look_vector = (p2 - p1).normalize();
    let look_at = p1 + look_vector;

    Transform::from_translation(p1).looking_at(look_at, Vec3::unit_y())
}
