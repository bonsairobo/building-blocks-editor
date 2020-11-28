use super::ThirdPersonTransform;

use bevy::{math::prelude::*, transform::components::Transform};

#[derive(Default)]
pub struct TransformSmoother {
    lerp_tfm: Option<LerpTransform>,
}

#[derive(Clone, Copy)]
pub struct LerpTransform {
    pub position: Vec3,
    pub target: Vec3,
}

impl LerpTransform {
    pub fn transform(&self) -> Transform {
        // Even though it's simpler, it's possible to get NaN in the transform if you look at the
        // target when it's too close. Make sure we look at a point that's never too close to the
        // position.
        let look_at = self.position + (self.target - self.position).normalize();

        Transform::from_translation(self.position).looking_at(look_at, Vec3::unit_y())
    }
}

impl TransformSmoother {
    /// Do linear interpolation between the previous smoothed transform and the new transform.
    pub fn smooth_transform(
        &mut self,
        lag_weight: f32,
        new_tfm: &ThirdPersonTransform,
    ) -> LerpTransform {
        debug_assert!(0.0 <= lag_weight);
        debug_assert!(lag_weight < 1.0);

        let old_lerp_tfm = self.lerp_tfm.unwrap_or_else(|| LerpTransform {
            position: new_tfm.get_position(),
            target: new_tfm.target,
        });

        let lead_weight = 1.0 - lag_weight;
        let lerp_pos = old_lerp_tfm.position * lag_weight + new_tfm.get_position() * lead_weight;
        let lerp_target = old_lerp_tfm.target * lag_weight + new_tfm.target * lead_weight;

        let new_lerp_tfm = LerpTransform {
            position: lerp_pos,
            target: lerp_target,
        };

        self.lerp_tfm = Some(new_lerp_tfm);

        new_lerp_tfm
    }
}
